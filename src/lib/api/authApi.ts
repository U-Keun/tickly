import type { AuthSession, UserProfile } from '../../types';
import { invoke } from './client';
import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';

// Types for Apple Sign In plugin
type AppleScope = 'fullName' | 'email';

interface AppleIDAuthorizationRequest {
  scope: AppleScope[];
  nonce?: string;
  state?: string;
}

interface AppleIDAuthorizationResponse {
  userIdentifier: string | null;
  givenName: string | null;
  familyName: string | null;
  email: string | null;
  authorizationCode: string;
  identityToken: string | null;
  state: string | null;
}

// Call the native Apple Sign In plugin (iOS only)
async function getAppleIdCredential(
  request: AppleIDAuthorizationRequest
): Promise<AppleIDAuthorizationResponse> {
  return tauriInvoke<AppleIDAuthorizationResponse>(
    'plugin:sign-in-with-apple|get_apple_id_credential',
    { payload: request }
  );
}

// Perform Apple Sign In flow: native dialog -> Supabase
export async function performAppleSignIn(): Promise<AuthSession> {
  // 1. Get credential from native Apple Sign In
  const credential = await getAppleIdCredential({
    scope: ['fullName', 'email'],
  });

  if (!credential.identityToken) {
    throw new Error('No identity token received from Apple');
  }

  // 2. Send identity token to Supabase via our backend
  return signInWithApple(credential.identityToken);
}

// Internal: send ID token to backend
export async function signInWithApple(
  idToken: string,
  nonce?: string
): Promise<AuthSession> {
  return invoke<AuthSession>('sign_in_with_apple', { idToken, nonce });
}

export async function signInWithGoogle(idToken: string): Promise<AuthSession> {
  return invoke<AuthSession>('sign_in_with_google', { idToken });
}

// ===== Mobile Google OAuth (iOS/Android) =====

// Start mobile OAuth flow - returns OAuth URL with deep link redirect
export async function startMobileGoogleOAuth(): Promise<string> {
  return invoke<string>('start_mobile_google_oauth');
}

// Complete mobile OAuth with code from deep link
export async function completeMobileGoogleOAuth(code: string): Promise<AuthSession> {
  return invoke<AuthSession>('complete_mobile_google_oauth', { code });
}

// ===== Desktop Google OAuth =====

export async function startGoogleOAuth(): Promise<string> {
  return invoke<string>('start_google_oauth');
}

export async function completeGoogleOAuth(): Promise<AuthSession> {
  return invoke<AuthSession>('complete_google_oauth');
}

// Perform Google OAuth flow for Desktop
export async function performGoogleSignIn(): Promise<AuthSession> {
  // 1. Start OAuth and get the URL
  const oauthUrl = await startGoogleOAuth();

  // 2. Open the URL in the default browser
  await openUrl(oauthUrl);

  // 3. Wait for the OAuth callback (backend handles this)
  return completeGoogleOAuth();
}

export async function getCurrentSession(): Promise<AuthSession | null> {
  return invoke<AuthSession | null>('get_current_session');
}

export async function refreshSession(): Promise<AuthSession> {
  return invoke<AuthSession>('refresh_session');
}

export async function signOut(): Promise<void> {
  return invoke<void>('sign_out');
}

export async function getUserProfile(): Promise<UserProfile | null> {
  return invoke<UserProfile | null>('get_user_profile');
}

export async function isLoggedIn(): Promise<boolean> {
  return invoke<boolean>('is_logged_in');
}
