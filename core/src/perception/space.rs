use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceObservation {
    pub id: Uuid,
    pub object: String,
    pub observation: String,
    pub source: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SpacePerception;

impl SpacePerception {
    pub fn new() -> Self {
        Self
    }

    pub fn observe(
        &self,
        object: impl Into<String>,
        observation: impl Into<String>,
        source: impl Into<String>,
    ) -> SpaceObservation {
        SpaceObservation {
            id: Uuid::new_v4(),
            object: object.into(),
            observation: observation.into(),
            source: source.into(),
            confidence: 0.0,
        }
    }
}
