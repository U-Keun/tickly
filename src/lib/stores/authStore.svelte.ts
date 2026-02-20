import * as authApi from '$lib/api/authApi';
import { openUrl } from '@tauri-apps/plugin-opener';
import { syncStore } from './syncStore.svelte';
import type { AuthSession, UserProfile } from '../../types';

// Pending OAuth callback resolver
let pendingOAuthResolve: ((session: AuthSession) => void) | null = null;
let pendingOAuthReject: ((error: Error) => void) | null = null;

function clearPendingOAuthCallbacks(): void {
  pendingOAuthResolve = null;
  pendingOAuthReject = null;
}

function createOAuthCallbackPromise(timeoutMs: number): Promise<AuthSession> {
  return new Promise<AuthSession>((resolve, reject) => {
    pendingOAuthResolve = resolve;
    pendingOAuthReject = reject;

    setTimeout(() => {
      if (pendingOAuthReject) {
        pendingOAuthReject(new Error('OAuth timeout'));
        clearPendingOAuthCallbacks();
      }
    }, timeoutMs);
  });
}

// Called from deep link handler when OAuth callback is received
export async function handleOAuthCallback(code: string): Promise<void> {
  try {
    const session = await authApi.completeMobileGoogleOAuth(code);
    if (pendingOAuthResolve) {
      pendingOAuthResolve(session);
      clearPendingOAuthCallbacks();
    }
  } catch (error) {
    if (pendingOAuthReject) {
      pendingOAuthReject(error instanceof Error ? error : new Error(String(error)));
      clearPendingOAuthCallbacks();
    }
  }
}

class AuthStore {
  isLoggedIn = $state(false);
  isLoading = $state(true);
  user = $state<UserProfile | null>(null);
  session = $state<AuthSession | null>(null);

  private clearAuthState(): void {
    this.session = null;
    this.user = null;
    this.isLoggedIn = false;
  }

  private async completeSignIn(sessionPromise: Promise<AuthSession>): Promise<void> {
    this.session = await sessionPromise;
    this.isLoggedIn = true;
    this.user = await authApi.getUserProfile();
  }

  private async withLoading<T>(operation: () => Promise<T>): Promise<T> {
    this.isLoading = true;
    try {
      return await operation();
    } finally {
      this.isLoading = false;
    }
  }

  async checkSession(): Promise<void> {
    await this.withLoading(async () => {
      try {
        const loggedIn = await authApi.isLoggedIn();
        this.isLoggedIn = loggedIn;

        if (loggedIn) {
          this.session = await authApi.getCurrentSession();
          this.user = await authApi.getUserProfile();
        } else {
          this.clearAuthState();
        }
      } catch (error) {
        console.error('Failed to check session:', error);
        this.clearAuthState();
      }
    });
  }

  async signInWithApple(): Promise<void> {
    try {
      await this.withLoading(async () => {
        // Uses native Apple Sign In on iOS
        await this.completeSignIn(authApi.performAppleSignIn());
      });
    } catch (error) {
      console.error('Apple sign in failed:', error);
      throw error;
    }
  }

  async signInWithGoogle(idToken: string): Promise<void> {
    try {
      await this.withLoading(async () => {
        await this.completeSignIn(authApi.signInWithGoogle(idToken));
      });
    } catch (error) {
      console.error('Google sign in failed:', error);
      throw error;
    }
  }

  // Desktop Google OAuth using PKCE flow
  async signInWithGoogleDesktop(): Promise<void> {
    try {
      await this.withLoading(async () => {
        await this.completeSignIn(authApi.performGoogleSignIn());
      });
    } catch (error) {
      console.error('Google sign in (Desktop) failed:', error);
      throw error;
    }
  }

  // Mobile Google OAuth using deep links (iOS/Android)
  async signInWithGoogleMobile(): Promise<void> {
    try {
      await this.withLoading(async () => {
        // 1. Get OAuth URL with deep link redirect
        const oauthUrl = await authApi.startMobileGoogleOAuth();

        // 2. Create a promise that will be resolved when deep link callback is received
        const sessionPromise = createOAuthCallbackPromise(5 * 60 * 1000);

        // 3. Open the OAuth URL in browser
        await openUrl(oauthUrl);

        // 4. Wait for the callback (handled by deep link listener)
        await this.completeSignIn(sessionPromise);
      });
    } catch (error) {
      console.error('Google sign in (Mobile) failed:', error);
      throw error;
    }
  }

  async signOut(): Promise<void> {
    try {
      await this.withLoading(async () => {
        // Disconnect realtime before signing out
        await syncStore.disconnectRealtime();

        await authApi.signOut();
        this.clearAuthState();
      });
    } catch (error) {
      console.error('Sign out failed:', error);
      throw error;
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
