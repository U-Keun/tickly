use chrono::{Duration, Utc};
use rusqlite::Connection;

use crate::models::{AuthProvider, AuthSession, UserProfile};
use crate::repository::AuthRepository;

use super::supabase_client::{SupabaseAuthResponse, SupabaseClient};

pub struct AuthService;

impl AuthService {
    /// Create a session from Supabase auth response (public for OAuth flow)
    pub fn create_session_from_response(
        response: SupabaseAuthResponse,
        provider: AuthProvider,
    ) -> AuthSession {
        Self::auth_response_to_session(response, provider)
    }

    fn auth_response_to_session(
        response: SupabaseAuthResponse,
        provider: AuthProvider,
    ) -> AuthSession {
        let expires_at = if let Some(expires_at) = response.expires_at {
            chrono::DateTime::from_timestamp(expires_at, 0)
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
                .unwrap_or_else(|| {
                    (Utc::now() + Duration::seconds(response.expires_in))
                        .format("%Y-%m-%dT%H:%M:%SZ")
                        .to_string()
                })
        } else {
            (Utc::now() + Duration::seconds(response.expires_in))
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string()
        };

        AuthSession {
            user_id: response.user.id,
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at,
            provider,
        }
    }

    pub async fn sign_in_with_apple(
        client: &SupabaseClient,
        id_token: &str,
        nonce: Option<&str>,
    ) -> Result<AuthSession, String> {
        let response = client.sign_in_with_apple(id_token, nonce).await?;
        let session = Self::auth_response_to_session(response, AuthProvider::Apple);
        Ok(session)
    }

    pub async fn sign_in_with_google(
        client: &SupabaseClient,
        id_token: &str,
    ) -> Result<AuthSession, String> {
        let response = client.sign_in_with_google(id_token).await?;
        let session = Self::auth_response_to_session(response, AuthProvider::Google);
        Ok(session)
    }

    pub fn save_session(conn: &Connection, session: &AuthSession) -> Result<(), String> {
        AuthRepository::save_session(conn, session).map_err(|e| e.to_string())
    }

    pub fn get_current_session(conn: &Connection) -> Result<Option<AuthSession>, String> {
        AuthRepository::get_session(conn).map_err(|e| e.to_string())
    }

    pub fn is_session_expired(session: &AuthSession) -> bool {
        if let Ok(expires_at) = chrono::DateTime::parse_from_rfc3339(&session.expires_at) {
            expires_at < Utc::now()
        } else {
            true
        }
    }

    pub async fn refresh_token(
        client: &SupabaseClient,
        refresh_token: &str,
        provider: AuthProvider,
    ) -> Result<AuthSession, String> {
        let response = client.refresh_token(refresh_token).await?;
        let session = Self::auth_response_to_session(response, provider);
        Ok(session)
    }

    pub async fn sign_out_remote(client: &SupabaseClient, access_token: &str) -> Result<(), String> {
        // Try to sign out from Supabase, but don't fail if it doesn't work
        let _ = client.sign_out(access_token).await;
        Ok(())
    }

    pub fn delete_session(conn: &Connection) -> Result<(), String> {
        AuthRepository::delete_session(conn).map_err(|e| e.to_string())
    }

    pub fn get_user_profile(session: &AuthSession) -> UserProfile {
        UserProfile {
            id: session.user_id.clone(),
            email: None,
            full_name: None,
            avatar_url: None,
        }
    }
}
