use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeopoliticalEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub region: String,
    pub actors: Vec<String>,
    pub event: String,
    pub sources: Vec<String>,
    pub confidence: f64,
    pub verified: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GeopoliticalPerception;

impl GeopoliticalPerception {
    pub fn new() -> Self {
        Self
    }

    pub fn event(
        &self,
        region: impl Into<String>,
        event: impl Into<String>,
    ) -> GeopoliticalEvent {
        GeopoliticalEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            region: region.into(),
            actors: Vec::new(),
            event: event.into(),
            sources: Vec::new(),
            confidence: 0.0,
            verified: false,
        }
    }
}
