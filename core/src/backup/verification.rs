use crate::backup::snapshot::Snapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupVerification {
    pub snapshot_id: uuid::Uuid,
    pub valid: bool,
    pub expected_hash: String,
    pub actual_hash: String,
}

#[derive(Debug, Clone, Default)]
pub struct BackupVerifier;

impl BackupVerifier {
    pub fn new() -> Self {
        Self
    }

    pub fn verify(
        &self,
        snapshot: &Snapshot,
        actual_hash: impl Into<String>,
    ) -> BackupVerification {
        let actual_hash = actual_hash.into();

        BackupVerification {
            snapshot_id: snapshot.id,
            valid: snapshot.state_hash == actual_hash,
            expected_hash: snapshot.state_hash.clone(),
            actual_hash,
        }
    }
}
