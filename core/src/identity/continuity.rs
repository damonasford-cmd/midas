use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityCheckpoint {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub identity_version: u64,
    pub state_digest: String,
    pub created_at: DateTime<Utc>,
}

impl ContinuityCheckpoint {
    pub fn new(
        identity_id: Uuid,
        identity_version: u64,
        state_digest: impl Into<String>,
    ) -> Result<Self, String> {
        let state_digest = state_digest.into();

        if state_digest.trim().is_empty() {
            return Err(
                "continuity state digest cannot be empty"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            identity_id,
            identity_version,
            state_digest,
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityContinuity {
    pub identity_id: Uuid,
    pub current_version: u64,
    pub checkpoints: Vec<ContinuityCheckpoint>,
    pub last_verified_at: Option<DateTime<Utc>>,
}

impl IdentityContinuity {
    pub fn new(
        identity_id: Uuid,
    ) -> Self {
        Self {
            identity_id,
            current_version: 1,
            checkpoints: Vec::new(),
            last_verified_at: None,
        }
    }

    pub fn checkpoint(
        &mut self,
        state_digest: impl Into<String>,
    ) -> Result<ContinuityCheckpoint, String> {
        let checkpoint =
            ContinuityCheckpoint::new(
                self.identity_id,
                self.current_version,
                state_digest,
            )?;

        self.checkpoints
            .push(checkpoint.clone());

        self.last_verified_at = Some(Utc::now());

        Ok(checkpoint)
    }

    pub fn advance_version(
        &mut self,
    ) {
        self.current_version += 1;
    }
}
