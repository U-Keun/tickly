use crate::models::RealtimeStatus;
use crate::service::{RealtimeConfig, RealtimeService};
use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::RwLock;

pub struct RealtimeState {
    pub service: Arc<RwLock<Option<RealtimeService>>>,
}

impl RealtimeState {
    pub fn new() -> Self {
        Self {
            service: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for RealtimeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Connect to Supabase Realtime
#[tauri::command]
pub async fn connect_realtime(
    app_handle: AppHandle,
    realtime_state: State<'_, RealtimeState>,
    supabase_url: String,
    anon_key: String,
    access_token: String,
    user_id: String,
) -> Result<(), String> {
    let config = RealtimeConfig {
        url: supabase_url,
        anon_key,
        access_token,
        user_id,
    };

    let mut service_guard = realtime_state.service.write().await;

    // Create new service if none exists
    if service_guard.is_none() {
        *service_guard = Some(RealtimeService::new(app_handle));
    }

    // Connect
    if let Some(service) = service_guard.as_ref() {
        service.connect(config).await
    } else {
        Err("Failed to create realtime service".to_string())
    }
}

/// Disconnect from Supabase Realtime
#[tauri::command]
pub async fn disconnect_realtime(
    realtime_state: State<'_, RealtimeState>,
) -> Result<(), String> {
    let service_guard = realtime_state.service.read().await;

    if let Some(service) = service_guard.as_ref() {
        service.disconnect().await
    } else {
        Ok(()) // Already disconnected
    }
}

/// Get realtime connection status
#[tauri::command]
pub async fn get_realtime_status(
    realtime_state: State<'_, RealtimeState>,
) -> Result<RealtimeStatus, String> {
    let service_guard = realtime_state.service.read().await;

    if let Some(service) = service_guard.as_ref() {
        Ok(service.get_status().await)
    } else {
        Ok(RealtimeStatus::default())
    }
}

/// Check if realtime is connected
#[tauri::command]
pub async fn is_realtime_connected(
    realtime_state: State<'_, RealtimeState>,
) -> Result<bool, String> {
    let service_guard = realtime_state.service.read().await;

    if let Some(service) = service_guard.as_ref() {
        Ok(service.is_connected().await)
    } else {
        Ok(false)
    }
}
