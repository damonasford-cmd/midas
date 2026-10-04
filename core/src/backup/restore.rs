use crate::backup::snapshot::Snapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestoreStatus {
    Prepared,
    Restoring,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePlan {
    pub snapshot_id: uuid::Uuid,
    pub target: String,
    pub status: RestoreStatus,
}

#[derive(Debug, Clone, Default)]
pub struct RestoreManager;

impl RestoreManager {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare(
        &self,
        snapshot: &Snapshot,
        target: impl Into<String>,
    ) -> RestorePlan {
        RestorePlan {
            snapshot_id: snapshot.id,
            target: target.into(),
            status: RestoreStatus::Prepared,
        }
    }
}
