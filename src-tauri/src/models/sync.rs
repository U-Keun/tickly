use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum SyncStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "synced")]
    Synced,
    #[serde(rename = "deleted")]
    Deleted,
}

impl SyncStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "synced" => SyncStatus::Synced,
            "deleted" => SyncStatus::Deleted,
            _ => SyncStatus::Pending,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            SyncStatus::Pending => "pending",
            SyncStatus::Synced => "synced",
            SyncStatus::Deleted => "deleted",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: String,
    pub provider: AuthProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthProvider {
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "google")]
    Google,
}

impl AuthProvider {
    pub fn from_str(s: &str) -> Self {
        match s {
            "google" => AuthProvider::Google,
            _ => AuthProvider::Apple,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            AuthProvider::Apple => "apple",
            AuthProvider::Google => "google",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncResult {
    pub pushed: usize,
    pub pulled: usize,
    pub last_synced_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatusInfo {
    pub is_enabled: bool,
    pub is_syncing: bool,
    pub is_online: bool,
    pub pending_count: usize,
    pub last_synced_at: Option<String>,
}
