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
pub enum MemoryLifecycleState {
    Active,
    Consolidating,
    Archived,
    Forgotten,
    Restored,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLifecycleStatus {
    pub state: MemoryLifecycleState,
    pub reason: Option<String>,
}

impl Default for MemoryLifecycleStatus {
    fn default() -> Self {
        Self {
            state: MemoryLifecycleState::Active,
            reason: None,
        }
    }
}
