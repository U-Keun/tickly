use super::*;

impl SupabaseClient {
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
    pub async fn sign_in_with_google(
        &self,
        id_token: &str,
    ) -> Result<SupabaseAuthResponse, String> {
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
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<SupabaseAuthResponse, String> {
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
}
