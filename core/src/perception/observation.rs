use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::modality::{ObservationStatus, PerceptionModality};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Uuid,
    pub source_id: String,
    pub modality: PerceptionModality,
    pub content_type: String,
    pub payload: Value,
    pub text_content: Option<String>,
    pub source_uri: Option<String>,
    pub source_record_id: Option<String>,
    pub deduplication_key: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub acquired_at: DateTime<Utc>,
    pub source_confidence: Option<f64>,
    pub metadata: Value,
    pub status: ObservationStatus,
}

impl Observation {
    pub fn validate(&self) -> Result<()> {
        if self.source_id.trim().is_empty() {
            bail!("Observation source ID cannot be empty");
        }

        if self.payload.is_null() || !self.metadata.is_object() {
            bail!("Observation payload or metadata is invalid");
        }

        if let Some(confidence) = self.source_confidence {
            if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
                bail!("Invalid source confidence");
            }
        }

        if self.acquired_at < self.occurred_at {
            // Des sources externes peuvent avoir des horloges décalées.
            // On ne rejette donc pas l'observation sur ce seul critère.
        }

        Ok(())
    }
}
