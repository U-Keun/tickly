use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Phoenix message format: [join_ref, ref, topic, event, payload]
#[derive(Debug, Clone)]
pub struct PhoenixMessage {
    pub join_ref: Option<String>,
    pub msg_ref: Option<String>,
    pub topic: String,
    pub event: String,
    pub payload: Value,
}

impl PhoenixMessage {
    /// Create a new Phoenix message
    pub fn new(
        join_ref: Option<String>,
        msg_ref: Option<String>,
        topic: &str,
        event: &str,
        payload: Value,
    ) -> Self {
        Self {
            join_ref,
            msg_ref,
            topic: topic.to_string(),
            event: event.to_string(),
            payload,
        }
    }

    /// Serialize to JSON array format
    pub fn to_json(&self) -> String {
        let arr = serde_json::json!([
            self.join_ref,
            self.msg_ref,
            self.topic,
            self.event,
            self.payload
        ]);
        arr.to_string()
    }

    /// Parse from JSON array format
    pub fn from_json(json: &str) -> Result<Self, String> {
        let arr: Vec<Value> =
            serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

        if arr.len() != 5 {
            return Err(format!("Expected 5 elements, got {}", arr.len()));
        }

        Ok(Self {
            join_ref: arr[0].as_str().map(|s| s.to_string()),
            msg_ref: arr[1].as_str().map(|s| s.to_string()),
            topic: arr[2]
                .as_str()
                .ok_or("Topic must be a string")?
                .to_string(),
            event: arr[3]
                .as_str()
                .ok_or("Event must be a string")?
                .to_string(),
            payload: arr[4].clone(),
        })
    }
}

/// Payload for postgres_changes subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresChangesConfig {
    pub event: String, // "INSERT", "UPDATE", "DELETE", or "*"
    pub schema: String,
    pub table: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

/// Payload for phx_join with postgres_changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeSubscribePayload {
    pub config: RealtimeConfigPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeConfigPayload {
    pub broadcast: BroadcastConfig,
    pub presence: PresenceConfig,
    pub postgres_changes: Vec<PostgresChangesConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastConfig {
    #[serde(rename = "self")]
    pub self_broadcast: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceConfig {
    pub key: String,
}

/// Wrapper for postgres_changes event payload (has nested "data" field)
#[derive(Debug, Clone, Deserialize)]
pub struct PostgresChangesEventPayload {
    pub data: PostgresChangePayload,
}

/// Postgres change payload received from Supabase
#[derive(Debug, Clone, Deserialize)]
pub struct PostgresChangePayload {
    pub schema: String,
    pub table: String,
    #[serde(rename = "type")]
    pub change_type: String, // "INSERT", "UPDATE", "DELETE"
    pub commit_timestamp: Option<String>,
    pub columns: Option<Vec<ColumnInfo>>,
    pub record: Option<Value>,
    pub old_record: Option<Value>,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub col_type: String,
}

/// System reply payload
#[derive(Debug, Clone, Deserialize)]
pub struct SystemReplyPayload {
    pub status: String,
    pub response: Option<Value>,
}

impl PostgresChangesConfig {
    /// Create config for subscribing to all changes on a table
    pub fn all_changes(table: &str, user_id: &str) -> Self {
        Self {
            event: "*".to_string(),
            schema: "public".to_string(),
            table: table.to_string(),
            filter: Some(format!("user_id=eq.{}", user_id)),
        }
    }
}

impl RealtimeSubscribePayload {
    /// Create payload for subscribing to postgres changes
    pub fn for_tables(tables: &[&str], user_id: &str) -> Self {
        let postgres_changes = tables
            .iter()
            .map(|table| PostgresChangesConfig::all_changes(table, user_id))
            .collect();

        Self {
            config: RealtimeConfigPayload {
                broadcast: BroadcastConfig {
                    self_broadcast: false,
                },
                presence: PresenceConfig {
                    key: String::new(),
                },
                postgres_changes,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phoenix_message_serialize() {
        let msg = PhoenixMessage::new(
            Some("1".to_string()),
            Some("1".to_string()),
            "realtime:public",
            "phx_join",
            serde_json::json!({}),
        );

        let json = msg.to_json();
        assert!(json.contains("realtime:public"));
        assert!(json.contains("phx_join"));
    }

    #[test]
    fn test_phoenix_message_parse() {
        let json = r#"["1","1","realtime:public","phx_reply",{"status":"ok"}]"#;
        let msg = PhoenixMessage::from_json(json).unwrap();

        assert_eq!(msg.topic, "realtime:public");
        assert_eq!(msg.event, "phx_reply");
    }
}
