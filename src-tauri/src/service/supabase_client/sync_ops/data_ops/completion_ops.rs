use super::super::*;

impl SupabaseClient {
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

    // Fetch all completion logs for the current user
    pub async fn fetch_all_completion_logs(
        &self,
        access_token: &str,
    ) -> Result<Vec<RemoteCompletionLog>, String> {
        let url = format!("{}/completion_logs?select=*", self.rest_url());

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
            return Err(format!("Fetch all completion logs failed: {}", error_text));
        }

        response
            .json::<Vec<RemoteCompletionLog>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    // Delete a completion log
    pub async fn delete_completion_log(
        &self,
        access_token: &str,
        log_id: &str,
    ) -> Result<(), String> {
        let url = format!("{}/completion_logs?id=eq.{}", self.rest_url(), log_id);

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
            return Err(format!("Delete completion log failed: {}", error_text));
        }

        Ok(())
    }
}
