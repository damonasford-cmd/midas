use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub version: String,
    pub description: String,
    pub state_hash: String,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SnapshotManager;

impl SnapshotManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        version: impl Into<String>,
        description: impl Into<String>,
        state_hash: impl Into<String>,
    ) -> Snapshot {
        Snapshot {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            version: version.into(),
            description: description.into(),
            state_hash: state_hash.into(),
            location: None,
        }
    }
}
