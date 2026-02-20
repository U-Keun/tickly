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

    fn spawn_connection_loop(&self, config: RealtimeConfig, mut shutdown_rx: mpsc::Receiver<()>) {
        let state = self.state.clone();
        let reconnect_attempts = self.reconnect_attempts.clone();
        let last_error = self.last_error.clone();
        let is_running = self.is_running.clone();
        let app_handle = self.app_handle.clone();

        tokio::spawn(async move {
            loop {
                // Check if we should stop
                if !is_running.load(Ordering::SeqCst) {
                    break;
                }

                // Try to connect
                *state.write().await = RealtimeConnectionState::Connecting;

                match Self::run_connection(&config, &app_handle, &mut shutdown_rx, &state).await {
                    Ok(()) => {
                        // Clean disconnect, don't reconnect
                        break;
                    }
                    Err(e) => {
                        log::error!("Realtime connection error: {}", e);
                        *last_error.write().await = Some(e.clone());

                        // Emit error event
                        let _ = app_handle.emit(
                            "realtime-event",
                            RealtimeEvent {
                                event_type: RealtimeEventType::Error,
                                message: Some(e),
                            },
                        );

                        // Check reconnect attempts
                        let attempts = reconnect_attempts.fetch_add(1, Ordering::SeqCst) + 1;
                        if attempts >= MAX_RECONNECT_ATTEMPTS {
                            log::error!(
                                "Max reconnect attempts ({}) reached, giving up",
                                MAX_RECONNECT_ATTEMPTS
                            );
                            is_running.store(false, Ordering::SeqCst);
                            *state.write().await = RealtimeConnectionState::Disconnected;
                            break;
                        }

                        // Emit reconnecting event
                        *state.write().await = RealtimeConnectionState::Reconnecting;
                        let _ = app_handle.emit(
                            "realtime-event",
                            RealtimeEvent {
                                event_type: RealtimeEventType::Reconnecting,
                                message: Some(format!(
                                    "Attempt {} of {}",
                                    attempts, MAX_RECONNECT_ATTEMPTS
                                )),
                            },
                        );

                        // Calculate backoff delay
                        let delay = std::cmp::min(
                            INITIAL_RECONNECT_DELAY_MS * 2u64.pow(attempts - 1),
                            MAX_RECONNECT_DELAY_MS,
                        );

                        log::info!("Reconnecting in {}ms (attempt {})", delay, attempts);

                        // Wait before reconnecting, but check for shutdown
                        tokio::select! {
                            _ = sleep(Duration::from_millis(delay)) => {}
                            _ = shutdown_rx.recv() => {
                                log::info!("Shutdown signal received during reconnect wait");
                                break;
                            }
                        }
                    }
                }
            }

            // Final cleanup
            *state.write().await = RealtimeConnectionState::Disconnected;
            is_running.store(false, Ordering::SeqCst);
        });
    }

    async fn run_connection(
        config: &RealtimeConfig,
        app_handle: &AppHandle,
        shutdown_rx: &mut mpsc::Receiver<()>,
        state: &Arc<RwLock<RealtimeConnectionState>>,
    ) -> Result<(), String> {
        let url = config.websocket_url();
        log::info!("Connecting to Supabase Realtime");

        // Connect to WebSocket
        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| format!("WebSocket connection failed: {}", e))?;

        let (mut write, mut read) = ws_stream.split();

        // Subscribe to channel
        let channel_topic = "realtime:public".to_string();
        let join_ref = "1";
        let msg_ref = "1";

        // Subscribe to postgres_changes for todos, categories, completion_logs
        let join_payload = serde_json::json!({
            "config": {
                "broadcast": {
                    "ack": false,
                    "self": false
                },
                "presence": {
                    "key": "",
                    "enabled": false
                },
                "postgres_changes": [
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "todos",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    },
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "categories",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    },
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "completion_logs",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    },
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "tags",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    },
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "todo_tags",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    }
                ],
                "private": false
            }
        });

        let join_msg = PhoenixMessage::new(
            Some(join_ref.to_string()),
            Some(msg_ref.to_string()),
            &channel_topic,
            "phx_join",
            join_payload,
        );

        // Send join message
        write
            .send(Message::Text(join_msg.to_json()))
            .await
            .map_err(|e| format!("Failed to send join message: {}", e))?;

        // Send access_token event immediately after join
        let access_token_msg = PhoenixMessage::new(
            Some(join_ref.to_string()),
            Some("2".to_string()),
            &channel_topic,
            "access_token",
            serde_json::json!({
                "access_token": config.access_token
            }),
        );

        write
            .send(Message::Text(access_token_msg.to_json()))
            .await
            .map_err(|e| format!("Failed to send access_token: {}", e))?;

        // Start heartbeat interval
        let mut heartbeat_interval = interval(Duration::from_secs(HEARTBEAT_INTERVAL_SECS));
        let mut msg_counter: u32 = 10;

        loop {
            tokio::select! {
                // Handle incoming messages
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            if let Err(e) = Self::handle_message(&text, app_handle, state).await {
                                log::warn!("Failed to handle message: {}", e);
                            }
                        }
                        Some(Ok(Message::Close(_))) => {
                            return Err("Connection closed by server".to_string());
                        }
                        Some(Ok(Message::Ping(data))) => {
                            let _ = write.send(Message::Pong(data)).await;
                        }
                        Some(Err(e)) => {
                            return Err(format!("WebSocket error: {}", e));
                        }
                        None => {
                            return Err("WebSocket stream ended".to_string());
                        }
                        _ => {}
                    }
                }

                // Send heartbeat
                _ = heartbeat_interval.tick() => {
                    msg_counter += 1;
                    let heartbeat = PhoenixMessage::new(
                        None,
                        Some(msg_counter.to_string()),
                        "phoenix",
                        "heartbeat",
                        serde_json::json!({}),
                    );

                    if let Err(e) = write.send(Message::Text(heartbeat.to_json())).await {
                        return Err(format!("Failed to send heartbeat: {}", e));
                    }
                }

                // Handle shutdown signal
                _ = shutdown_rx.recv() => {
                    log::info!("Realtime shutdown signal received");

                    // Send phx_leave
                    let leave_msg = PhoenixMessage::new(
                        Some(join_ref.to_string()),
                        Some("999".to_string()),
                        &channel_topic,
                        "phx_leave",
                        serde_json::json!({}),
                    );
                    let _ = write.send(Message::Text(leave_msg.to_json())).await;
                    let _ = write.send(Message::Close(None)).await;

                    return Ok(());
                }
            }
        }
    }

    async fn handle_message(
        text: &str,
        app_handle: &AppHandle,
        state: &Arc<RwLock<RealtimeConnectionState>>,
    ) -> Result<(), String> {
        let msg = PhoenixMessage::from_json(text)?;

        match msg.event.as_str() {
            "phx_reply" => {
                if let Ok(reply) = serde_json::from_value::<SystemReplyPayload>(msg.payload.clone())
                {
                    if reply.status == "ok" {
                        // Check if this is a join reply (join_ref = "1" and msg_ref = "1")
                        if msg.join_ref.as_deref() == Some("1")
                            && msg.msg_ref.as_deref() == Some("1")
                        {
                            log::info!("Realtime channel join successful");

                            // Update internal state to Connected
                            *state.write().await = RealtimeConnectionState::Connected;

                            // Emit connected event
                            let _ = app_handle.emit(
                                "realtime-event",
                                RealtimeEvent {
                                    event_type: RealtimeEventType::Connected,
                                    message: None,
                                },
                            );
                        }
                    } else {
                        // Emit error for join failure
                        if msg.msg_ref.as_deref() == Some("1") {
                            log::error!("Realtime channel join failed: {:?}", reply.response);
                            let _ = app_handle.emit(
                                "realtime-event",
                                RealtimeEvent {
                                    event_type: RealtimeEventType::Error,
                                    message: Some(format!("Join failed: {:?}", reply.response)),
                                },
                            );
                        }
                    }
                }
            }
            "postgres_changes" => {
                // Payload has nested "data" field
                if let Ok(wrapper) =
                    serde_json::from_value::<PostgresChangesEventPayload>(msg.payload.clone())
                {
                    let change = wrapper.data;
                    let change_type = match change.change_type.as_str() {
                        "INSERT" => DataChangeType::Insert,
                        "UPDATE" => DataChangeType::Update,
                        "DELETE" => DataChangeType::Delete,
                        _ => return Ok(()),
                    };

                    let sync_id = change
                        .record
                        .as_ref()
                        .and_then(|r| r.get("id"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| {
                            change
                                .old_record
                                .as_ref()
                                .and_then(|r| r.get("id"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                        });

                    let event = DataChangedEvent {
                        table: change.table,
                        change_type,
                        sync_id,
                    };

                    log::debug!("Realtime data changed: {:?}", event);
                    let _ = app_handle.emit("data-changed", event);
                }
            }
            "system" | "presence_state" | "presence_diff" => {
                // Ignore these events
            }
            _ => {
                log::trace!("Unhandled realtime event: {}", msg.event);
            }
        }

        Ok(())
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
