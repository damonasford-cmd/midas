use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutobiographicalEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub significance: f64,
}

impl AutobiographicalEvent {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        significance: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            title: title.into(),
            description: description.into(),
            significance: significance.clamp(0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Autobiography {
    pub events: Vec<AutobiographicalEvent>,
}

impl Autobiography {
    pub fn record(&mut self, event: AutobiographicalEvent) {
        self.events.push(event);
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn latest(&self) -> Option<&AutobiographicalEvent> {
        self.events.last()
    }
}
