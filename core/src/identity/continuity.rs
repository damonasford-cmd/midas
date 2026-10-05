use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityCheckpoint {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub identity_version: u64,
    pub continuity_hash: String,
}

impl ContinuityCheckpoint {
    pub fn new(identity_version: u64, continuity_hash: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            identity_version,
            continuity_hash: continuity_hash.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityContinuity {
    pub origin_timestamp: DateTime<Utc>,
    pub last_checkpoint: Option<ContinuityCheckpoint>,
    pub generation: u64,
    pub uninterrupted: bool,
}

impl IdentityContinuity {
    pub fn new(origin_timestamp: DateTime<Utc>) -> Self {
        Self {
            origin_timestamp,
            last_checkpoint: None,
            generation: 1,
            uninterrupted: true,
        }
    }

    pub fn checkpoint(
        &mut self,
        identity_version: u64,
        continuity_hash: impl Into<String>,
    ) {
        self.last_checkpoint =
            Some(ContinuityCheckpoint::new(identity_version, continuity_hash));

        self.generation = self.generation.saturating_add(1);
    }

    pub fn mark_interrupted(&mut self) {
        self.uninterrupted = false;
    }

    pub fn mark_restored(&mut self) {
        self.uninterrupted = true;
    }
}
