use super::*;

impl SupabaseClient {
    pub async fn fetch_tags(&self, access_token: &str) -> Result<Vec<RemoteTag>, String> {
        let url = format!("{}/tags?select=*", self.rest_url());

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
            return Err(format!("Fetch tags failed: {}", error_text));
        }

        response
            .json::<Vec<RemoteTag>>()
            .await
            .map_err(|e| format!("Failed to parse tags: {}", e))
    }

    pub async fn upsert_tag(&self, access_token: &str, tag: &RemoteTag) -> Result<(), String> {
        let url = format!("{}/tags", self.rest_url());

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates")
            .json(tag)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Upsert tag failed: {}", error_text));
        }

        Ok(())
    }

    pub async fn delete_tag(&self, access_token: &str, sync_id: &str) -> Result<(), String> {
        let url = format!("{}/tags?id=eq.{}", self.rest_url(), sync_id);

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
            return Err(format!("Delete tag failed: {}", error_text));
        }

        Ok(())
    }

    pub async fn fetch_todo_tags(&self, access_token: &str) -> Result<Vec<RemoteTodoTag>, String> {
        let url = format!("{}/todo_tags?select=*", self.rest_url());

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
            return Err(format!("Fetch todo_tags failed: {}", error_text));
        }

        response
            .json::<Vec<RemoteTodoTag>>()
            .await
            .map_err(|e| format!("Failed to parse todo_tags: {}", e))
    }

    pub async fn upsert_todo_tag(
        &self,
        access_token: &str,
        todo_tag: &RemoteTodoTag,
    ) -> Result<(), String> {
        let url = format!("{}/todo_tags", self.rest_url());

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates")
            .json(todo_tag)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Upsert todo_tag failed: {}", error_text));
        }

        Ok(())
    }

    pub async fn delete_todo_tag(&self, access_token: &str, sync_id: &str) -> Result<(), String> {
        let url = format!("{}/todo_tags?id=eq.{}", self.rest_url(), sync_id);

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
            return Err(format!("Delete todo_tag failed: {}", error_text));
        }

        Ok(())
    }
}
