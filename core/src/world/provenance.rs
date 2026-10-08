
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceKind {
    DirectObservation,
    UserProvided,
    OfficialDocument,
    TrustedDatabase,
    WebPage,
    Api,
    Sensor,
    Device,
    HumanReport,
    InternalCalculation,
    ExternalModel,
    Inference,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Unverified,
    PartiallyVerified,
    Verified,
    Contradicted,
    Stale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub id: Uuid,
    pub source_kind: SourceKind,
    pub source_identifier: Option<String>,
    pub source_uri: Option<String>,
    pub collected_at: DateTime<Utc>,
    pub source_timestamp: Option<DateTime<Utc>>,
    pub verification: VerificationStatus,
    pub verification_notes: Vec<String>,
    pub content_digest: Option<String>,
}

impl Provenance {
    pub fn new(source_kind: SourceKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_kind,
            source_identifier: None,
            source_uri: None,
            collected_at: Utc::now(),
            source_timestamp: None,
            verification: VerificationStatus::Unverified,
            verification_notes: Vec::new(),
            content_digest: None,
        }
    }

    pub fn record_verification(
        &mut self,
        status: VerificationStatus,
        note: impl Into<String>,
    ) -> Result<(), String> {
        let note = note.into();

        if note.trim().is_empty() {
            return Err("verification note cannot be empty".into());
        }

        self.verification = status;
        self.verification_notes.push(note);

        Ok(())
    }

    pub fn is_verified(&self) -> bool {
        self.verification == VerificationStatus::Verified
    }
}
