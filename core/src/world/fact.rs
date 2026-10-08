
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::provenance::VerificationStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactStatus {
    Proposed,
    Supported,
    Verified,
    Disputed,
    Refuted,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub id: Uuid,
    pub subject_entity_id: Option<Uuid>,
    pub predicate: String,
    pub object: serde_json::Value,
    pub status: FactStatus,
    pub confidence: f32,
    pub provenance_ids: Vec<Uuid>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub observed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Fact {
    pub fn new(
        subject_entity_id: Option<Uuid>,
        predicate: impl Into<String>,
        object: serde_json::Value,
        confidence: f32,
    ) -> Result<Self, String> {
        let predicate = predicate.into();

        if predicate.trim().is_empty() {
            return Err("fact predicate cannot be empty".into());
        }

        if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
            return Err("fact confidence must be between 0 and 1".into());
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            subject_entity_id,
            predicate,
            object,
            status: FactStatus::Proposed,
            confidence,
            provenance_ids: Vec::new(),
            valid_from: None,
            valid_until: None,
            observed_at: now,
            updated_at: now,
        })
    }

    pub fn add_provenance(
        &mut self,
        provenance_id: Uuid,
    ) {
        if !self.provenance_ids.contains(&provenance_id) {
            self.provenance_ids.push(provenance_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn apply_verification(
        &mut self,
        verification: VerificationStatus,
    ) {
        self.status = match verification {
            VerificationStatus::Unverified => FactStatus::Proposed,
            VerificationStatus::PartiallyVerified => FactStatus::Supported,
            VerificationStatus::Verified => FactStatus::Verified,
            VerificationStatus::Contradicted => FactStatus::Disputed,
            VerificationStatus::Stale => FactStatus::Expired,
        };

        self.updated_at = Utc::now();
    }

    pub fn is_current_at(&self, time: DateTime<Utc>) -> bool {
        self.valid_from.map_or(true, |start| time >= start)
            && self.valid_until.map_or(true, |end| time <= end)
            && self.status != FactStatus::Expired
            && self.status != FactStatus::Refuted
    }
}
