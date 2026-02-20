use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
}

impl SupabaseConfig {
    pub fn from_env() -> Option<Self> {
        let url = option_env!("SUPABASE_URL")
            .map(|s| s.to_string())
            .or_else(|| std::env::var("SUPABASE_URL").ok())?;

        let anon_key = option_env!("SUPABASE_ANON_KEY")
            .map(|s| s.to_string())
            .or_else(|| std::env::var("SUPABASE_ANON_KEY").ok())?;

        Some(Self { url, anon_key })
    }
}

#[derive(Debug, Clone)]
pub struct SupabaseClient {
    config: SupabaseConfig,
    client: Arc<Client>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupabaseAuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub expires_at: Option<i64>,
    pub token_type: String,
    pub user: SupabaseUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupabaseUser {
    pub id: String,
    pub email: Option<String>,
    pub user_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteCategory {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub display_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteTodo {
    pub id: String,
    pub user_id: String,
    pub category_id: Option<String>,
    pub text: String,
    pub done: bool,
    pub display_order: i32,
    pub memo: Option<String>,
    pub repeat_type: String,
    pub repeat_detail: Option<String>,
    pub next_due_at: Option<String>,
    pub last_completed_at: Option<String>,
    pub track_streak: bool,
    pub reminder_at: Option<String>,
    pub linked_app: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteCompletionLog {
    pub id: String,
    pub user_id: String,
    pub todo_id: String,
    pub completed_on: String,
    pub completed_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteTag {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteTodoTag {
    pub id: String,
    pub user_id: String,
    pub todo_id: String,
    pub tag_id: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct OAuthSignInRequest {
    id_token: String,
    nonce: Option<String>,
}

#[derive(Debug, Serialize)]
struct RefreshTokenRequest {
    refresh_token: String,
}

mod auth_ops;
mod oauth_ops;
mod sync_ops;

impl SupabaseClient {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            config,
            client: Arc::new(
                Client::builder()
                    .build()
                    .expect("Failed to create HTTP client"),
            ),
        }
    }

    fn rest_url(&self) -> String {
        format!("{}/rest/v1", self.config.url)
    }

    fn auth_url(&self) -> String {
        format!("{}/auth/v1", self.config.url)
    }
}
