use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::modality::PerceptionModality;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionSource {
    pub id: String,
    pub source_type: String,
    pub display_name: String,
    pub enabled: bool,
    pub metadata: Value,
}

impl PerceptionSource {
    pub fn validate(&self) -> Result<()> {
        if self.id.trim().is_empty() || self.id.len() > 200 {
            bail!("Source ID must contain 1 to 200 bytes");
        }

        if self.source_type.trim().is_empty() || self.source_type.len() > 100 {
            bail!("Source type must contain 1 to 100 bytes");
        }

        if self.display_name.trim().is_empty()
            || self.display_name.len() > 200
        {
            bail!("Source display name must contain 1 to 200 bytes");
        }

        if !self.metadata.is_object() {
            bail!("Source metadata must be a JSON object");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDraft {
    pub source_id: String,
    pub modality: PerceptionModality,
    pub content_type: String,
    pub payload: Value,
    pub text_content: Option<String>,
    pub source_uri: Option<String>,
    pub source_record_id: Option<String>,
    pub deduplication_key: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub source_confidence: Option<f64>,
    pub metadata: Value,
}

impl ObservationDraft {
    pub fn validate(&self) -> Result<()> {
        if self.source_id.trim().is_empty() || self.source_id.len() > 200 {
            bail!("Observation source ID is invalid");
        }

        if self.content_type.trim().is_empty()
            || self.content_type.len() > 200
        {
            bail!("Observation content type is invalid");
        }

        if self.payload.is_null() {
            bail!("Observation payload cannot be null");
        }

        if let Some(text) = &self.text_content {
            if text.len() > 2_000_000 {
                bail!("Observation text exceeds 2 MB");
            }
        }

        if let Some(uri) = &self.source_uri {
            if uri.trim().is_empty() || uri.len() > 8_192 {
                bail!("Observation source URI is invalid");
            }
        }

        if let Some(record_id) = &self.source_record_id {
            if record_id.trim().is_empty() || record_id.len() > 500 {
                bail!("Observation source record ID is invalid");
            }
        }

        if let Some(key) = &self.deduplication_key {
            if key.trim().is_empty() || key.len() > 500 {
                bail!("Observation deduplication key is invalid");
            }
        }

        if let Some(confidence) = self.source_confidence {
            if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
                bail!("Source confidence must be between 0 and 1");
            }
        }

        if !self.metadata.is_object() {
            bail!("Observation metadata must be a JSON object");
        }

        let serialized_size = serde_json::to_vec(&self.payload)?.len();
        if serialized_size > 5 * 1024 * 1024 {
            bail!("Observation payload exceeds 5 MiB");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquiredObservation {
    pub id: Uuid,
    pub draft: ObservationDraft,
    pub acquired_at: DateTime<Utc>,
}

impl AcquiredObservation {
    pub fn new(draft: ObservationDraft) -> Result<Self> {
        draft.validate()?;

        Ok(Self {
            id: Uuid::new_v4(),
            draft,
            acquired_at: Utc::now(),
        })
    }
}

#[async_trait]
pub trait PerceptionConnector: Send + Sync {
    fn source(&self) -> &PerceptionSource;

    /// Acquiert des données auprès de la source réelle.
    /// L'implémentation doit respecter les permissions de cette source.
    async fn acquire(&self) -> Result<Vec<ObservationDraft>>;
}
