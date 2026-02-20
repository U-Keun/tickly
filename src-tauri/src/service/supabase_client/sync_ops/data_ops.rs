use super::*;

impl SupabaseClient {
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

        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

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

        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

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
            .header(
                "Prefer",
                "resolution=merge-duplicates,return=representation",
            )
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
            .header(
                "Prefer",
                "resolution=merge-duplicates,return=representation",
            )
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
    pub async fn delete_category(&self, access_token: &str, sync_id: &str) -> Result<(), String> {
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
