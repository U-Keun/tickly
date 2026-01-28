use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteCompletionLog {
    pub id: String,
    pub user_id: String,
    pub todo_id: String,
    pub completed_at: String,
}

#[derive(Debug, Serialize)]
struct OAuthSignInRequest {
    id_token: String,
    nonce: Option<String>,
}

#[derive(Debug, Serialize)]
struct RefreshTokenRequest {
    refresh_token: String,
}

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

    // Sign in with Apple ID token
    pub async fn sign_in_with_apple(
        &self,
        id_token: &str,
        nonce: Option<&str>,
    ) -> Result<SupabaseAuthResponse, String> {
        let url = format!("{}/token?grant_type=id_token", self.auth_url());

        let body = serde_json::json!({
            "provider": "apple",
            "id_token": id_token,
            "nonce": nonce
        });

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Apple sign in failed: {}", error_text));
        }

        response
            .json::<SupabaseAuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    // Sign in with Google ID token
    pub async fn sign_in_with_google(&self, id_token: &str) -> Result<SupabaseAuthResponse, String> {
        let url = format!("{}/token?grant_type=id_token", self.auth_url());

        let body = serde_json::json!({
            "provider": "google",
            "id_token": id_token
        });

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Google sign in failed: {}", error_text));
        }

        response
            .json::<SupabaseAuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    // Refresh access token
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<SupabaseAuthResponse, String> {
        let url = format!("{}/token?grant_type=refresh_token", self.auth_url());

        let body = RefreshTokenRequest {
            refresh_token: refresh_token.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Token refresh failed: {}", error_text));
        }

        response
            .json::<SupabaseAuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    // Sign out
    pub async fn sign_out(&self, access_token: &str) -> Result<(), String> {
        let url = format!("{}/logout", self.auth_url());

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Sign out failed: {}", error_text));
        }

        Ok(())
    }

    // Get current user info
    pub async fn get_user(&self, access_token: &str) -> Result<SupabaseUser, String> {
        let url = format!("{}/user", self.auth_url());

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Get user failed: {}", error_text));
        }

        response
            .json::<SupabaseUser>()
            .await
            .map_err(|e| format!("Failed to parse user response: {}", e))
    }

    // Fetch all categories for the user
    pub async fn fetch_categories(
        &self,
        access_token: &str,
    ) -> Result<Vec<RemoteCategory>, String> {
        let url = format!("{}/categories?select=*", self.rest_url());

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Fetch categories failed: {}", error_text));
        }

        let text = response.text().await.map_err(|e| format!("Failed to read response: {}", e))?;

        // Debug: return error with response content if empty
        if text == "[]" {
            return Err(format!("Categories API returned empty array. URL: {}", url));
        }

        serde_json::from_str::<Vec<RemoteCategory>>(&text)
            .map_err(|e| format!("Failed to parse categories: {} - Response was: {}", e, text))
    }

    // Fetch all todos for the user
    pub async fn fetch_todos(&self, access_token: &str) -> Result<Vec<RemoteTodo>, String> {
        let url = format!("{}/todos?select=*", self.rest_url());

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Fetch todos failed: {}", error_text));
        }

        let text = response.text().await.map_err(|e| format!("Failed to read response: {}", e))?;

        // Debug: return error with response content if empty
        if text == "[]" {
            return Err(format!("Todos API returned empty array. URL: {}", url));
        }

        serde_json::from_str::<Vec<RemoteTodo>>(&text)
            .map_err(|e| format!("Failed to parse todos: {} - Response was: {}", e, text))
    }

    // Upsert a category
    pub async fn upsert_category(
        &self,
        access_token: &str,
        category: &RemoteCategory,
    ) -> Result<RemoteCategory, String> {
        let url = format!("{}/categories", self.rest_url());

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates,return=representation")
            .json(category)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Upsert category failed: {}", error_text));
        }

        let mut categories: Vec<RemoteCategory> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        categories
            .pop()
            .ok_or_else(|| "No category returned".to_string())
    }

    // Upsert a todo
    pub async fn upsert_todo(
        &self,
        access_token: &str,
        todo: &RemoteTodo,
    ) -> Result<RemoteTodo, String> {
        let url = format!("{}/todos", self.rest_url());

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates,return=representation")
            .json(todo)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Upsert todo failed: {}", error_text));
        }

        let mut todos: Vec<RemoteTodo> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        todos.pop().ok_or_else(|| "No todo returned".to_string())
    }

    // Delete a category
    pub async fn delete_category(
        &self,
        access_token: &str,
        sync_id: &str,
    ) -> Result<(), String> {
        let url = format!("{}/categories?id=eq.{}", self.rest_url(), sync_id);

        let response = self
            .client
            .delete(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Delete category failed: {}", error_text));
        }

        Ok(())
    }

    // Delete a todo
    pub async fn delete_todo(&self, access_token: &str, sync_id: &str) -> Result<(), String> {
        let url = format!("{}/todos?id=eq.{}", self.rest_url(), sync_id);

        let response = self
            .client
            .delete(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Delete todo failed: {}", error_text));
        }

        Ok(())
    }

    // Fetch completion logs for a todo
    pub async fn fetch_completion_logs(
        &self,
        access_token: &str,
        todo_id: &str,
    ) -> Result<Vec<RemoteCompletionLog>, String> {
        let url = format!(
            "{}/completion_logs?todo_id=eq.{}&select=*",
            self.rest_url(),
            todo_id
        );

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Fetch completion logs failed: {}", error_text));
        }

        response
            .json::<Vec<RemoteCompletionLog>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    // Upsert a completion log
    pub async fn upsert_completion_log(
        &self,
        access_token: &str,
        log: &RemoteCompletionLog,
    ) -> Result<(), String> {
        let url = format!("{}/completion_logs", self.rest_url());

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates")
            .json(log)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Upsert completion log failed: {}", error_text));
        }

        Ok(())
    }

    // ===== OAuth PKCE Flow for Desktop =====

    /// Generate a PKCE code verifier (43-128 characters)
    pub fn generate_code_verifier() -> String {
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        URL_SAFE_NO_PAD.encode(&bytes)
    }

    /// Generate code challenge from verifier (SHA256 + base64url)
    pub fn generate_code_challenge(verifier: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        URL_SAFE_NO_PAD.encode(&hash)
    }

    /// Get the OAuth URL for Google sign-in with PKCE
    pub fn get_google_oauth_url(&self, redirect_url: &str, code_challenge: &str) -> String {
        let base_url = format!("{}/authorize", self.auth_url());

        format!(
            "{}?provider=google&redirect_to={}&code_challenge={}&code_challenge_method=S256",
            base_url,
            urlencoding::encode(redirect_url),
            urlencoding::encode(code_challenge)
        )
    }

    /// Exchange authorization code for tokens using PKCE
    pub async fn exchange_code_for_token(
        &self,
        code: &str,
        code_verifier: &str,
    ) -> Result<SupabaseAuthResponse, String> {
        let url = format!("{}/token?grant_type=pkce", self.auth_url());

        let body = serde_json::json!({
            "auth_code": code,
            "code_verifier": code_verifier
        });

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Code exchange failed: {}", error_text));
        }

        response
            .json::<SupabaseAuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }
}

// URL encoding helper
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
