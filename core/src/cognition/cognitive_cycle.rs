use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    cognitive_depth::CognitiveDepth,
    cognitive_phase::{
        CognitivePhase,
        CognitivePhaseResult,
    },
};

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
pub struct CognitiveCycleId(Uuid);

impl CognitiveCycleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for CognitiveCycleId {
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
pub enum CognitiveCycleStatus {
    Created,
    Running,
    Paused,
    Blocked,
    AwaitingAuthorization,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveCycle {
    pub id:
        CognitiveCycleId,

    pub status:
        CognitiveCycleStatus,

    pub depth:
        CognitiveDepth,

    pub current_phase:
        CognitivePhase,

    pub completed_phases:
        Vec<CognitivePhaseResult>,

    pub started_at:
        chrono::DateTime<chrono::Utc>,

    pub updated_at:
        chrono::DateTime<chrono::Utc>,

    pub iteration:
        u64,
}

impl CognitiveCycle {
    pub fn new(
        depth: CognitiveDepth,
    ) -> Self {
        let now = chrono::Utc::now();

        Self {
            id: CognitiveCycleId::new(),
            status: CognitiveCycleStatus::Created,
            depth,
            current_phase:
                CognitivePhase::Objective,
            completed_phases: Vec::new(),
            started_at: now,
            updated_at: now,
            iteration: 0,
        }
    }

    pub fn start(&mut self) {
        self.status =
            CognitiveCycleStatus::Running;

        self.updated_at =
            chrono::Utc::now();
    }

    pub fn record_phase(
        &mut self,
        result: CognitivePhaseResult,
    ) {
        self.completed_phases
            .push(result);

        self.updated_at =
            chrono::Utc::now();

        self.iteration += 1;
    }

    pub fn set_phase(
        &mut self,
        phase: CognitivePhase,
    ) {
        self.current_phase = phase;

        self.updated_at =
            chrono::Utc::now();
    }

    pub fn complete(&mut self) {
        self.status =
            CognitiveCycleStatus::Completed;

        self.updated_at =
            chrono::Utc::now();
    }

    pub fn block(&mut self) {
        self.status =
            CognitiveCycleStatus::Blocked;

        self.updated_at =
            chrono::Utc::now();
    }

    pub fn fail(&mut self) {
        self.status =
            CognitiveCycleStatus::Failed;

        self.updated_at =
            chrono::Utc::now();
    }
}
