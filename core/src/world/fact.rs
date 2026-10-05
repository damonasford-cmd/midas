use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    provenance::Provenance,
    temporal::TemporalInterval,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FactStatus {
    Proposed,
    Supported,
    Verified,
    Refuted,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub id: Uuid,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub status: FactStatus,
    pub confidence: f64,
    pub provenance: Vec<Provenance>,
    pub valid_during: Option<TemporalInterval>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Fact {
    pub fn new(
        subject: impl Into<String>,
        predicate: impl Into<String>,
        object: impl Into<String>,
        confidence: f64,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            status: FactStatus::Proposed,
            confidence: confidence.clamp(0.0, 1.0),
            provenance: Vec::new(),
            valid_during: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_provenance(&mut self, provenance: Provenance) {
        self.provenance.push(provenance);
        self.updated_at = Utc::now();
    }

    pub fn set_status(&mut self, status: FactStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn set_temporal_interval(
        &mut self,
        interval: TemporalInterval,
    ) {
        self.valid_during = Some(interval);
        self.updated_at = Utc::now();
    }
}
