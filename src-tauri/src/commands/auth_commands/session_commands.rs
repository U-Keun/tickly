use tauri::State;

use super::with_db;
use super::{basic_user_profile, require_supabase_client, save_session};
use crate::models::{AuthSession, UserProfile};
use crate::service::AuthService;
use crate::AppState;

#[tauri::command]
pub async fn sign_in_with_apple(
    state: State<'_, AppState>,
    id_token: String,
    nonce: Option<String>,
) -> Result<AuthSession, String> {
    let client = require_supabase_client(&state)?;

    // Perform async operation first (no lock held)
    let session = AuthService::sign_in_with_apple(client, &id_token, nonce.as_deref()).await?;

    // Then save to database
    save_session(&state, &session)?;

    Ok(session)
}

#[tauri::command]
pub async fn sign_in_with_google(
    state: State<'_, AppState>,
    id_token: String,
) -> Result<AuthSession, String> {
    let client = require_supabase_client(&state)?;

    // Perform async operation first (no lock held)
    let session = AuthService::sign_in_with_google(client, &id_token).await?;

    // Then save to database
    save_session(&state, &session)?;

    Ok(session)
}

#[tauri::command]
pub fn get_current_session(state: State<'_, AppState>) -> Result<Option<AuthSession>, String> {
    with_db(&state, |conn| AuthService::get_current_session(conn))
}

#[tauri::command]
pub async fn refresh_session(state: State<'_, AppState>) -> Result<AuthSession, String> {
    let client = require_supabase_client(&state)?;

    // Get current session without holding lock across await
    let current_session = with_db(&state, |conn| {
        AuthService::get_current_session(conn)?.ok_or_else(|| "No session found".to_string())
    })?;

    // Perform async operation
    let new_session = AuthService::refresh_token(
        client,
        &current_session.refresh_token,
        current_session.provider,
    )
    .await?;

    // Save new session
    save_session(&state, &new_session)?;

    Ok(new_session)
}

#[tauri::command]
pub async fn sign_out(state: State<'_, AppState>) -> Result<(), String> {
    let client = require_supabase_client(&state)?;

    // Get access token without holding lock across await
    let access_token = with_db(&state, |conn| {
        AuthService::get_current_session(conn).map(|session| session.map(|s| s.access_token))
    })?;

    // Sign out from remote if we have a token
    if let Some(token) = access_token {
        let _ = AuthService::sign_out_remote(client, &token).await;
    }

    // Delete local session
    with_db(&state, |conn| AuthService::delete_session(conn))?;

    Ok(())
}

#[tauri::command]
pub async fn get_user_profile(state: State<'_, AppState>) -> Result<Option<UserProfile>, String> {
    let client = state.supabase.as_ref();

    // Get session without holding lock
    let session = with_db(&state, |conn| AuthService::get_current_session(conn))?;

    match (session, client) {
        (Some(session), Some(client)) => {
            // Fetch user info from Supabase
            match client.get_user(&session.access_token).await {
                Ok(user) => {
                    let full_name = user
                        .user_metadata
                        .as_ref()
                        .and_then(|m| m.get("full_name"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| {
                            // Try to construct from first/last name
                            let first = user
                                .user_metadata
                                .as_ref()
                                .and_then(|m| m.get("name"))
                                .and_then(|v| v.as_str());
                            first.map(|s| s.to_string())
                        });

                    Ok(Some(UserProfile {
                        id: user.id,
                        email: user.email,
                        full_name,
                        avatar_url: user
                            .user_metadata
                            .as_ref()
                            .and_then(|m| m.get("avatar_url"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    }))
                }
                Err(_) => {
                    // Fallback to basic profile from session
                    Ok(Some(basic_user_profile(&session)))
                }
            }
        }
        (Some(session), None) => {
            // No Supabase client, return basic profile
            Ok(Some(basic_user_profile(&session)))
        }
        (None, _) => Ok(None),
    }
}

#[tauri::command]
pub async fn is_logged_in(state: State<'_, AppState>) -> Result<bool, String> {
    let session_opt = with_db(&state, |conn| AuthService::get_current_session(conn))?;
    let client_opt = state.supabase.clone();

    match session_opt {
        Some(session) => {
            if AuthService::is_session_expired(&session) {
                // Try to refresh the session
                if let Some(client) = client_opt {
                    match AuthService::refresh_token(
                        &client,
                        &session.refresh_token,
                        session.provider.clone(),
                    )
                    .await
                    {
                        Ok(new_session) => {
                            // Save the refreshed session
                            save_session(&state, &new_session)?;
                            Ok(true)
                        }
                        Err(_) => {
                            // Refresh failed, session is invalid
                            Ok(false)
                        }
                    }
                } else {
                    Ok(false)
                }
            } else {
                Ok(true)
            }
        }
        None => Ok(false),
    }
}
