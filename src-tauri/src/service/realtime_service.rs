use crate::models::{
    DataChangeType, DataChangedEvent, RealtimeConnectionState, RealtimeEvent, RealtimeEventType,
    RealtimeStatus,
};
use crate::service::realtime_messages::{
    PhoenixMessage, PostgresChangesEventPayload, SystemReplyPayload,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, sleep};
use tokio_tungstenite::{connect_async, tungstenite::Message};

const HEARTBEAT_INTERVAL_SECS: u64 = 25;
const MAX_RECONNECT_ATTEMPTS: u32 = 10;
const INITIAL_RECONNECT_DELAY_MS: u64 = 1000;
const MAX_RECONNECT_DELAY_MS: u64 = 30000;

mod connection;

/// Configuration for realtime connection
#[derive(Debug, Clone)]
pub struct RealtimeConfig {
    pub url: String,
    pub anon_key: String,
    pub access_token: String,
    pub user_id: String,
}

impl RealtimeConfig {
    /// Build WebSocket URL for Supabase Realtime
    pub fn websocket_url(&self) -> String {
        // Convert https://xxx.supabase.co to wss://xxx.supabase.co/realtime/v1/websocket
        let ws_url = self.url.replace("https://", "wss://");
        format!(
            "{}/realtime/v1/websocket?apikey={}&vsn=2.0.0",
            ws_url, self.anon_key
        )
    }
}

/// Realtime service for managing WebSocket connection to Supabase
pub struct RealtimeService {
    config: Arc<RwLock<Option<RealtimeConfig>>>,
    state: Arc<RwLock<RealtimeConnectionState>>,
    reconnect_attempts: Arc<AtomicU32>,
    last_error: Arc<RwLock<Option<String>>>,
    shutdown_tx: Arc<RwLock<Option<mpsc::Sender<()>>>>,
    is_running: Arc<AtomicBool>,
    app_handle: AppHandle,
}

impl RealtimeService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            config: Arc::new(RwLock::new(None)),
            state: Arc::new(RwLock::new(RealtimeConnectionState::Disconnected)),
            reconnect_attempts: Arc::new(AtomicU32::new(0)),
            last_error: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
            is_running: Arc::new(AtomicBool::new(false)),
            app_handle,
        }
    }

    /// Get current status
    pub async fn get_status(&self) -> RealtimeStatus {
        RealtimeStatus {
            state: *self.state.read().await,
            reconnect_attempts: self.reconnect_attempts.load(Ordering::SeqCst),
            last_error: self.last_error.read().await.clone(),
        }
    }

    /// Connect to Supabase Realtime
    pub async fn connect(&self, config: RealtimeConfig) -> Result<(), String> {
        // Check if already running
        if self.is_running.load(Ordering::SeqCst) {
            return Err("Realtime connection already running".to_string());
        }

        // Store config
        *self.config.write().await = Some(config.clone());

        // Reset state
        self.reconnect_attempts.store(0, Ordering::SeqCst);
        *self.last_error.write().await = None;

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);
        *self.shutdown_tx.write().await = Some(shutdown_tx);

        // Mark as running
        self.is_running.store(true, Ordering::SeqCst);

        // Start connection loop in background
        self.spawn_connection_loop(config, shutdown_rx);

        Ok(())
    }

    /// Disconnect from Supabase Realtime
    pub async fn disconnect(&self) -> Result<(), String> {
        if !self.is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        // Send shutdown signal
        if let Some(tx) = self.shutdown_tx.write().await.take() {
            let _ = tx.send(()).await;
        }

        // Mark as not running
        self.is_running.store(false, Ordering::SeqCst);

        // Update state
        self.set_state(RealtimeConnectionState::Disconnected).await;

        // Emit disconnect event
        self.emit_event(RealtimeEventType::Disconnected, None);

        Ok(())
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        *self.state.read().await == RealtimeConnectionState::Connected
    }

    async fn set_state(&self, new_state: RealtimeConnectionState) {
        *self.state.write().await = new_state;
    }

    fn emit_event(&self, event_type: RealtimeEventType, message: Option<String>) {
        let _ = self.app_handle.emit(
            "realtime-event",
            RealtimeEvent {
                event_type,
                message,
            },
        );
    }
}
