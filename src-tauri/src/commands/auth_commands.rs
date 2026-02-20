mod oauth_commands;
mod session_commands;

use std::sync::Mutex;
use tauri::State;

use super::with_db;
use crate::models::{AuthProvider, AuthSession, UserProfile};
use crate::service::{AuthService, OAuthService};
use crate::AppState;

pub use oauth_commands::*;
pub use session_commands::*;

// Store for OAuth state (code verifier) during the flow
pub struct OAuthStateStore {
    pub code_verifier: Mutex<Option<String>>,
    pub port: Mutex<Option<u16>>,
}

pub(super) fn require_supabase_client<'a>(
    state: &'a State<'_, AppState>,
) -> Result<&'a crate::service::SupabaseClient, String> {
    state
        .supabase
        .as_ref()
        .ok_or_else(|| "Supabase not configured".to_string())
}

pub(super) fn save_session(
    state: &State<'_, AppState>,
    session: &AuthSession,
) -> Result<(), String> {
    with_db(state, |conn| AuthService::save_session(conn, session))
}

pub(super) fn basic_user_profile(session: &AuthSession) -> UserProfile {
    UserProfile {
        id: session.user_id.clone(),
        email: None,
        full_name: None,
        avatar_url: None,
    }
}

pub(super) fn take_code_verifier(
    oauth_state: &State<'_, OAuthStateStore>,
) -> Result<String, String> {
    oauth_state
        .code_verifier
        .lock()
        .map_err(|e| e.to_string())?
        .take()
        .ok_or("No OAuth flow in progress".to_string())
}

pub(super) async fn complete_google_oauth_session(
    state: &State<'_, AppState>,
    client: &crate::service::SupabaseClient,
    code: &str,
    code_verifier: &str,
) -> Result<AuthSession, String> {
    let response = OAuthService::complete_oauth(client, code, code_verifier).await?;
    let session = AuthService::create_session_from_response(response, AuthProvider::Google);
    save_session(state, &session)?;
    Ok(session)
}
