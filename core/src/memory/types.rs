use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryModality {
    Text,
    Image,
    Audio,
    Video,
    File,
    StructuredData,
    WorldObservation,
    ActionResult,
    Other,
}

impl MemoryModality {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Video => "video",
            Self::File => "file",
            Self::StructuredData => "structured_data",
            Self::WorldObservation => "world_observation",
            Self::ActionResult => "action_result",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryVerification {
    Unverified,
    Observed,
    Corroborated,
    Verified,
    Disputed,
}

impl MemoryVerification {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unverified => "unverified",
            Self::Observed => "observed",
            Self::Corroborated => "corroborated",
            Self::Verified => "verified",
            Self::Disputed => "disputed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemorySensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
}

impl MemorySensitivity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Internal => "internal",
            Self::Confidential => "confidential",
            Self::Restricted => "restricted",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMemory {
    pub modality: MemoryModality,
    pub content: Value,
    pub searchable_text: Option<String>,
    pub source: String,
    pub source_reference: Option<String>,
    pub occurred_at: Option<DateTime<Utc>>,
    pub verification: MemoryVerification,
    pub sensitivity: MemorySensitivity,
    pub confidence: Option<f32>,
    pub embedding: Option<Vec<f32>>,
    pub embedding_model: Option<String>,
    pub idempotency_key: Option<String>,
    pub metadata: Value,
}

impl NewMemory {
    pub fn validate(&self) -> Result<()> {
        if self.source.trim().is_empty() {
            bail!("memory source cannot be empty");
        }

        if let Some(text) = &self.searchable_text {
            if text.len() > 2_000_000 {
                bail!("searchable text exceeds the 2 MB limit");
            }
        }

        if let Some(confidence) = self.confidence {
            if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
                bail!("confidence must be finite and between 0 and 1");
            }
        }

        match (&self.embedding, &self.embedding_model) {
            (Some(vector), Some(model)) => {
                if vector.is_empty() {
                    bail!("embedding cannot be empty");
                }

                if model.trim().is_empty() {
                    bail!("embedding model cannot be empty");
                }

                if vector.iter().any(|value| !value.is_finite()) {
                    bail!("embedding contains a non-finite value");
                }
            }
            (Some(_), None) => {
                bail!("embedding_model is required when an embedding is supplied");
            }
            (None, Some(_)) => {
                bail!("embedding_model cannot be supplied without an embedding");
            }
            (None, None) => {}
        }

        if self
            .idempotency_key
            .as_ref()
            .is_some_and(|key| key.trim().is_empty())
        {
            bail!("idempotency key cannot be empty");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: Uuid,
    pub modality: MemoryModality,
    pub content: Value,
    pub searchable_text: Option<String>,
    pub source: String,
    pub source_reference: Option<String>,
    pub occurred_at: Option<DateTime<Utc>>,
    pub recorded_at: DateTime<Utc>,
    pub verification: MemoryVerification,
    pub sensitivity: MemorySensitivity,
    pub confidence: Option<f32>,
    pub embedding: Option<Vec<f32>>,
    pub embedding_model: Option<String>,
    pub metadata: Value,
    pub content_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMatch {
    pub memory: MemoryRecord,
    /// Distance cosinus : plus petite = plus proche.
    pub distance: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRelation {
    pub id: Uuid,
    pub from_memory: Uuid,
    pub to_memory: Uuid,
    pub relation_type: String,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingMemoryEvent {
    pub id: Uuid,
    pub event_type: String,
    pub payload: Value,
    pub created_at: DateTime<Utc>,
}
