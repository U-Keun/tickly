use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

use super::*;

impl SupabaseClient {
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
