use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type EventId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricEvent {
    pub id: EventId,
    pub event_type: String,
    pub version: u32,
    pub occurred_at: DateTime<Utc>,
    pub payload: serde_json::Value,
}

impl FabricEvent {
    pub fn new(
        event_type: impl Into<String>,
        version: u32,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: event_type.into(),
            version,
            occurred_at: Utc::now(),
            payload,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.event_type.trim().is_empty() {
            return Err("event_type cannot be empty".into());
        }

        if self.version == 0 {
            return Err("event version must be greater than zero".into());
        }

        Ok(())
    }
}
