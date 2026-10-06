use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum MemoryHealthState {
    Healthy,
    Degraded,
    Corrupted,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHealth {
    pub state: MemoryHealthState,

    pub total_records: usize,
    pub active_records: usize,
    pub archived_records: usize,

    pub indexed_records: usize,

    pub integrity_errors: usize,

    pub retrieval_available: bool,
    pub persistence_available: bool,
}

impl Default for MemoryHealth {
    fn default() -> Self {
        Self {
            state: MemoryHealthState::Healthy,
            total_records: 0,
            active_records: 0,
            archived_records: 0,
            indexed_records: 0,
            integrity_errors: 0,
            retrieval_available: true,
            persistence_available: true,
        }
    }
}
