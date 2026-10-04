use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSession {
    pub id: Uuid,
    pub participant: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub transcript: Vec<String>,
    pub status: CallStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallStatus {
    Planned,
    Ringing,
    Active,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Default)]
pub struct CallEngine;

impl CallEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn start(
        &self,
        participant: impl Into<String>,
    ) -> CallSession {
        CallSession {
            id: Uuid::new_v4(),
            participant: participant.into(),
            started_at: Utc::now(),
            ended_at: None,
            transcript: Vec::new(),
            status: CallStatus::Active,
        }
    }
}
