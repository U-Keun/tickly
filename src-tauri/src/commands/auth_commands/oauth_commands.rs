use tauri::State;

use super::{
    complete_google_oauth_session, require_supabase_client, take_code_verifier, OAuthStateStore,
};
use crate::models::AuthSession;
use crate::service::OAuthService;
use crate::AppState;

// ===== Mobile OAuth Commands =====

/// Start the Google OAuth flow for Mobile (iOS/Android)
/// Returns the OAuth URL with deep link redirect
#[tauri::command]
pub fn start_mobile_google_oauth(
    state: State<'_, AppState>,
    oauth_state: State<'_, OAuthStateStore>,
) -> Result<String, String> {
    let client = require_supabase_client(&state)?;

    // Use deep link as redirect URL
    let redirect_url = "tickly://auth/callback";

    // Generate PKCE codes
    let code_verifier = crate::service::supabase_client::SupabaseClient::generate_code_verifier();
    let code_challenge =
        crate::service::supabase_client::SupabaseClient::generate_code_challenge(&code_verifier);

    // Get OAuth URL
    let oauth_url = client.get_google_oauth_url(redirect_url, &code_challenge);

    // Store the code verifier for later
    *oauth_state
        .code_verifier
        .lock()
        .map_err(|e| e.to_string())? = Some(code_verifier);

    Ok(oauth_url)
}

/// Complete the Mobile OAuth flow with the authorization code from deep link
#[tauri::command]
pub async fn complete_mobile_google_oauth(
    state: State<'_, AppState>,
    oauth_state: State<'_, OAuthStateStore>,
    code: String,
) -> Result<AuthSession, String> {
    let client = require_supabase_client(&state)?;
    let code_verifier = take_code_verifier(&oauth_state)?;

    complete_google_oauth_session(&state, client, &code, &code_verifier).await
}

// ===== Desktop OAuth Commands =====

/// Start the Google OAuth flow for Desktop
/// Returns the OAuth URL to open in the browser
#[tauri::command]
pub async fn start_google_oauth(
    state: State<'_, AppState>,
    oauth_state: State<'_, OAuthStateStore>,
) -> Result<String, String> {
    let client = require_supabase_client(&state)?;

    let (oauth_url, code_verifier, port) = OAuthService::start_google_oauth(client).await?;

    // Store the code verifier and port for later
    *oauth_state
        .code_verifier
        .lock()
        .map_err(|e| e.to_string())? = Some(code_verifier);
    *oauth_state.port.lock().map_err(|e| e.to_string())? = Some(port);

    Ok(oauth_url)
}

/// Wait for the OAuth callback and complete the sign-in
#[tauri::command]
pub async fn complete_google_oauth(
    state: State<'_, AppState>,
    oauth_state: State<'_, OAuthStateStore>,
) -> Result<AuthSession, String> {
    let client = require_supabase_client(&state)?;
    let code_verifier = take_code_verifier(&oauth_state)?;

    let port = oauth_state
        .port
        .lock()
        .map_err(|e| e.to_string())?
        .take()
        .ok_or("No OAuth flow in progress")?;

    // Wait for the callback (timeout: 5 minutes)
    let code = OAuthService::wait_for_callback(port, 300).await?;

    complete_google_oauth_session(&state, client, &code, &code_verifier).await
}
