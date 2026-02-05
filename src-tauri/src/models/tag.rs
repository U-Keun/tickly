use serde::{Deserialize, Serialize};

use super::SyncStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub sync_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub sync_status: SyncStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoTag {
    pub todo_id: i64,
    pub tag_id: i64,
    pub sync_id: Option<String>,
    pub created_at: Option<String>,
    pub sync_status: SyncStatus,
}
