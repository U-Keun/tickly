use super::*;

impl RealtimeService {
    pub(super) async fn handle_message(
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
}
