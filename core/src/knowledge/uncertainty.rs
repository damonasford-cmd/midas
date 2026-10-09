use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UncertaintyKind {
    MissingEvidence,
    ConflictingEvidence,
    AmbiguousMeaning,
    OutdatedInformation,
    SourceReliability,
    MeasurementError,
    Unknown,
}

impl UncertaintyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingEvidence => "missing_evidence",
            Self::ConflictingEvidence => "conflicting_evidence",
            Self::AmbiguousMeaning => "ambiguous_meaning",
            Self::OutdatedInformation => "outdated_information",
            Self::SourceReliability => "source_reliability",
            Self::MeasurementError => "measurement_error",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUncertainty {
    pub kind: UncertaintyKind,
    pub description: String,
    pub severity: f32,
    pub resolution_hint: Option<String>,
}

impl NewUncertainty {
    pub fn validate(&self) -> Result<()> {
        if self.description.trim().is_empty() {
            bail!("uncertainty description cannot be empty");
        }
        if !self.severity.is_finite()
            || !(0.0..=1.0).contains(&self.severity)
        {
            bail!("uncertainty severity must be between 0 and 1");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyRecord {
    pub id: Uuid,
    pub knowledge_id: Uuid,
    pub kind: UncertaintyKind,
    pub description: String,
    pub severity: f32,
    pub resolution_hint: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
