use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeStatus {
    Proposed,
    Supported,
    Verified,
    Disputed,
    Rejected,
    Superseded,
}

impl KnowledgeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Supported => "supported",
            Self::Verified => "verified",
            Self::Disputed => "disputed",
            Self::Rejected => "rejected",
            Self::Superseded => "superseded",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewKnowledgeItem {
    pub subject: String,
    pub predicate: String,
    pub object: Value,
    pub statement: String,
    pub source: String,
    pub source_reference: Option<String>,
    pub status: KnowledgeStatus,
    pub confidence: f32,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub metadata: Value,
    pub idempotency_key: Option<String>,
}

impl NewKnowledgeItem {
    pub fn validate(&self) -> Result<()> {
        if self.subject.trim().is_empty() {
            bail!("knowledge subject cannot be empty");
        }
        if self.predicate.trim().is_empty() {
            bail!("knowledge predicate cannot be empty");
        }
        if self.statement.trim().is_empty() {
            bail!("knowledge statement cannot be empty");
        }
        if self.source.trim().is_empty() {
            bail!("knowledge source cannot be empty");
        }
        if !self.confidence.is_finite()
            || !(0.0..=1.0).contains(&self.confidence)
        {
            bail!("confidence must be finite and between 0 and 1");
        }
        if let (Some(from), Some(until)) = (self.valid_from, self.valid_until) {
            if until < from {
                bail!("valid_until cannot precede valid_from");
            }
        }
        if let Some(key) = &self.idempotency_key {
            if key.trim().is_empty() {
                bail!("idempotency_key cannot be empty");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: Uuid,
    pub subject: String,
    pub predicate: String,
    pub object: Value,
    pub statement: String,
    pub source: String,
    pub source_reference: Option<String>,
    pub status: KnowledgeStatus,
    pub confidence: f32,
    pub version: i64,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub metadata: Value,
    pub content_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeVersion {
    pub knowledge_id: Uuid,
    pub version: i64,
    pub snapshot: Value,
    pub content_hash: String,
    pub changed_at: DateTime<Utc>,
    pub change_reason: String,
}
