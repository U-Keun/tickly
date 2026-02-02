use serde::{Deserialize, Serialize};

/// Realtime connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

impl Default for RealtimeConnectionState {
    fn default() -> Self {
        Self::Disconnected
    }
}

/// Realtime status info returned to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeStatus {
    pub state: RealtimeConnectionState,
    pub reconnect_attempts: u32,
    pub last_error: Option<String>,
}

impl Default for RealtimeStatus {
    fn default() -> Self {
        Self {
            state: RealtimeConnectionState::Disconnected,
            reconnect_attempts: 0,
            last_error: None,
        }
    }
}

/// Event types emitted to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeEventType {
    Connected,
    Disconnected,
    Reconnecting,
    Error,
}

/// Event payload sent to frontend via Tauri events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeEvent {
    pub event_type: RealtimeEventType,
    pub message: Option<String>,
}

/// Data change event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DataChangeType {
    Insert,
    Update,
    Delete,
}

/// Data change event sent to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChangedEvent {
    pub table: String,
    pub change_type: DataChangeType,
    pub sync_id: Option<String>,
}
