use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    pub id: Uuid,
    pub title: String,
    pub participants: Vec<String>,
    pub scheduled_at: DateTime<Utc>,
    pub agenda: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct MeetingEngine;

impl MeetingEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        title: impl Into<String>,
        scheduled_at: DateTime<Utc>,
    ) -> Meeting {
        Meeting {
            id: Uuid::new_v4(),
            title: title.into(),
            participants: Vec::new(),
            scheduled_at,
            agenda: Vec::new(),
            notes: Vec::new(),
        }
    }
}
