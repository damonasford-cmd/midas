use crate::foundation::contracts::{Observation, ObservationModality};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GlobalPerceptionEngine;

impl GlobalPerceptionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn observation(
        &self,
        source: impl Into<String>,
        modality: ObservationModality,
        content: impl Into<String>,
        confidence: f64,
        provenance: impl Into<String>,
    ) -> Observation {
        Observation {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: source.into(),
            modality,
            content: content.into(),
            confidence: confidence.clamp(0.0, 1.0),
            provenance: provenance.into(),
        }
    }
}
