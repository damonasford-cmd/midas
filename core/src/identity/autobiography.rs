use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutobiographicalEvent {
    pub id: Uuid,
    pub event_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub source_event_id: Option<Uuid>,
}

impl AutobiographicalEvent {
    pub fn new(
        event_type: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let event_type = event_type.into();
        let description = description.into();

        if event_type.trim().is_empty()
            || description.trim().is_empty()
        {
            return Err(
                "autobiographical event fields cannot be empty"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            event_type,
            description,
            timestamp: Utc::now(),
            source_event_id: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Autobiography {
    events: Vec<AutobiographicalEvent>,
}

impl Autobiography {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn record(
        &mut self,
        event: AutobiographicalEvent,
    ) {
        self.events.push(event);
    }

    pub fn events(
        &self,
    ) -> &[AutobiographicalEvent] {
        &self.events
    }

    pub fn len(
        &self,
    ) -> usize {
        self.events.len()
    }
}

impl Default for Autobiography {
    fn default() -> Self {
        Self::new()
    }
}
