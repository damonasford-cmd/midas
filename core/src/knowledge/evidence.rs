use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePolarity {
    Supports,
    Contradicts,
    Neutral,
}

impl EvidencePolarity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supports => "supports",
            Self::Contradicts => "contradicts",
            Self::Neutral => "neutral",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEvidence {
    pub polarity: EvidencePolarity,
    pub description: String,
    pub source: String,
    pub source_reference: Option<String>,
    pub reliability: f32,
    pub observed_at: Option<DateTime<Utc>>,
    pub metadata: Value,
    pub idempotency_key: Option<String>,
}

impl NewEvidence {
    pub fn validate(&self) -> Result<()> {
        if self.description.trim().is_empty() {
            bail!("evidence description cannot be empty");
        }
        if self.source.trim().is_empty() {
            bail!("evidence source cannot be empty");
        }
        if !self.reliability.is_finite()
            || !(0.0..=1.0).contains(&self.reliability)
        {
            bail!("evidence reliability must be between 0 and 1");
        }
        if let Some(key) = &self.idempotency_key {
            if key.trim().is_empty() {
                bail!("evidence idempotency_key cannot be empty");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub knowledge_id: Uuid,
    pub polarity: EvidencePolarity,
    pub description: String,
    pub source: String,
    pub source_reference: Option<String>,
    pub reliability: f32,
    pub observed_at: Option<DateTime<Utc>>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}
