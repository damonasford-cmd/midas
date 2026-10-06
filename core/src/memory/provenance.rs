use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemorySourceKind {
    User,
    SelfObservation,
    WorldObservation,
    Event,
    Tool,
    ExternalModel,
    File,
    Database,
    Human,
    Derived,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySource {
    pub kind: MemorySourceKind,

    pub source_id: Option<String>,

    pub event_id: Option<Uuid>,

    pub observed_at: DateTime<Utc>,

    pub reliability: f32,

    pub description: Option<String>,
}

impl Default for MemorySource {
    fn default() -> Self {
        Self {
            kind: MemorySourceKind::Unknown,
            source_id: None,
            event_id: None,
            observed_at: Utc::now(),
            reliability: 0.5,
            description: None,
        }
    }
}

impl MemorySource {
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.reliability) {
            return Err(
                "memory source reliability must be between 0 and 1"
                    .into(),
            );
        }

        Ok(())
    }
}
