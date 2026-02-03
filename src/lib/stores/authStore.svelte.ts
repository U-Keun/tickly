import * as authApi from '$lib/api/authApi';
import { syncStore } from './syncStore.svelte';
import type { AuthSession, UserProfile } from '../../types';

// Pending OAuth callback resolver
let pendingOAuthResolve: ((session: AuthSession) => void) | null = null;
let pendingOAuthReject: ((error: Error) => void) | null = null;

// Called from deep link handler when OAuth callback is received
export async function handleOAuthCallback(code: string): Promise<void> {
  try {
    const session = await authApi.completeMobileGoogleOAuth(code);
    if (pendingOAuthResolve) {
      pendingOAuthResolve(session);
      pendingOAuthResolve = null;
      pendingOAuthReject = null;
    }
  } catch (error) {
    if (pendingOAuthReject) {
      pendingOAuthReject(error instanceof Error ? error : new Error(String(error)));
      pendingOAuthResolve = null;
      pendingOAuthReject = null;
    }
  }
}

class AuthStore {
  isLoggedIn = $state(false);
  isLoading = $state(true);
  user = $state<UserProfile | null>(null);
  session = $state<AuthSession | null>(null);

  async checkSession(): Promise<void> {
    this.isLoading = true;
    try {
      const loggedIn = await authApi.isLoggedIn();
      this.isLoggedIn = loggedIn;

      if (loggedIn) {
        this.session = await authApi.getCurrentSession();
        this.user = await authApi.getUserProfile();
      } else {
        this.session = null;
        this.user = null;
      }
    } catch (error) {
      console.error('Failed to check session:', error);
      this.isLoggedIn = false;
      this.session = null;
      this.user = null;
    } finally {
      this.isLoading = false;
    }
  }

  async signInWithApple(): Promise<void> {
    this.isLoading = true;
    try {
      // Uses native Apple Sign In on iOS
      this.session = await authApi.performAppleSignIn();
      this.isLoggedIn = true;
      this.user = await authApi.getUserProfile();
    } catch (error) {
      console.error('Apple sign in failed:', error);
      throw error;
    } finally {
      this.isLoading = false;
    }
  }

  async signInWithGoogle(idToken: string): Promise<void> {
    this.isLoading = true;
    try {
      this.session = await authApi.signInWithGoogle(idToken);
      this.isLoggedIn = true;
      this.user = await authApi.getUserProfile();
    } catch (error) {
      console.error('Google sign in failed:', error);
      throw error;
    } finally {
      this.isLoading = false;
    }
  }

  // Desktop Google OAuth using PKCE flow
  async signInWithGoogleDesktop(): Promise<void> {
    this.isLoading = true;
    try {
      this.session = await authApi.performGoogleSignIn();
      this.isLoggedIn = true;
      this.user = await authApi.getUserProfile();
    } catch (error) {
      console.error('Google sign in (Desktop) failed:', error);
      throw error;
    } finally {
      this.isLoading = false;
    }
  }

  // Mobile Google OAuth using deep links (iOS/Android)
  async signInWithGoogleMobile(): Promise<void> {
    this.isLoading = true;
    try {
      // 1. Get OAuth URL with deep link redirect
      const oauthUrl = await authApi.startMobileGoogleOAuth();

      // 2. Create a promise that will be resolved when deep link callback is received
      const sessionPromise = new Promise<AuthSession>((resolve, reject) => {
        pendingOAuthResolve = resolve;
        pendingOAuthReject = reject;

        // Timeout after 5 minutes
        setTimeout(() => {
          if (pendingOAuthReject) {
            pendingOAuthReject(new Error('OAuth timeout'));
            pendingOAuthResolve = null;
            pendingOAuthReject = null;
          }
        }, 5 * 60 * 1000);
      });

      // 3. Open the OAuth URL in browser
      const { openUrl } = await import('@tauri-apps/plugin-opener');
      await openUrl(oauthUrl);

      // 4. Wait for the callback (handled by deep link listener)
      this.session = await sessionPromise;
      this.isLoggedIn = true;
      this.user = await authApi.getUserProfile();
    } catch (error) {
      console.error('Google sign in (Mobile) failed:', error);
      throw error;
    } finally {
      this.isLoading = false;
    }
  }

  async signOut(): Promise<void> {
    this.isLoading = true;
    try {
      // Disconnect realtime before signing out
      await syncStore.disconnectRealtime();

      await authApi.signOut();
      this.session = null;
      this.user = null;
      this.isLoggedIn = false;
    } catch (error) {
      console.error('Sign out failed:', error);
      throw error;
    } finally {
      this.isLoading = false;
    }
  }

  async refreshSession(): Promise<void> {
    try {
      this.session = await authApi.refreshSession();
    } catch (error) {
      console.error('Session refresh failed:', error);
      // If refresh fails, sign out
      await this.signOut();
      throw error;
    }
  }
}

export const authStore = new AuthStore();
