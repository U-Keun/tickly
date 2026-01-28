use serde::{Deserialize, Serialize};

use super::SyncStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub display_order: i64,
    // Sync fields
    pub sync_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub sync_status: SyncStatus,
}
