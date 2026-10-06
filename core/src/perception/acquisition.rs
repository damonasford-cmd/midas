use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum AcquisitionStatus {
    Requested,
    Acquiring,
    Acquired,
    Partial,
    Failed,
    TimedOut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionContext {
    pub id: Uuid,

    pub status: AcquisitionStatus,

    pub started_at:
        chrono::DateTime<chrono::Utc>,

    pub completed_at:
        Option<chrono::DateTime<chrono::Utc>>,

    pub timeout_ms:
        Option<u64>,

    pub attempts:
        u32,

    pub errors:
        Vec<String>,
}

impl AcquisitionContext {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            status: AcquisitionStatus::Requested,
            started_at: chrono::Utc::now(),
            completed_at: None,
            timeout_ms: None,
            attempts: 0,
            errors: Vec::new(),
        }
    }
}

impl Default for AcquisitionContext {
    fn default() -> Self {
        Self::new()
    }
}
