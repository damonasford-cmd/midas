use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct PerceptionCycleId(Uuid);

impl PerceptionCycleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for PerceptionCycleId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum PerceptionCycleStatus {
    Created,
    Acquiring,
    Processing,
    Fusing,
    Interpreting,
    Completed,
    Partial,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionCycle {
    pub id:
        PerceptionCycleId,

    pub status:
        PerceptionCycleStatus,

    pub input_count:
        usize,

    pub observation_count:
        usize,

    pub started_at:
        chrono::DateTime<chrono::Utc>,

    pub completed_at:
        Option<chrono::DateTime<chrono::Utc>>,

    pub errors:
        Vec<String>,
}

impl PerceptionCycle {
    pub fn new() -> Self {
        Self {
            id:
                PerceptionCycleId::new(),

            status:
                PerceptionCycleStatus::Created,

            input_count: 0,

            observation_count: 0,

            started_at:
                chrono::Utc::now(),

            completed_at: None,

            errors: Vec::new(),
        }
    }

    pub fn complete(
        &mut self,
        partial: bool,
    ) {
        self.status =
            if partial {
                PerceptionCycleStatus::Partial
            } else {
                PerceptionCycleStatus::Completed
            };

        self.completed_at =
            Some(chrono::Utc::now());
    }
}

impl Default for PerceptionCycle {
    fn default() -> Self {
        Self::new()
    }
}
