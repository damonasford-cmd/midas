use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    cognitive_depth::CognitiveDepth,
    cognitive_phase::CognitivePhase,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum CognitiveStateStatus {
    Idle,
    Processing,
    Waiting,
    Blocked,
    AwaitingAuthorization,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub cycle_id:
        Option<Uuid>,

    pub status:
        CognitiveStateStatus,

    pub phase:
        Option<CognitivePhase>,

    pub depth:
        CognitiveDepth,

    pub progress:
        f32,

    pub blocking_reasons:
        Vec<String>,

    pub unresolved_unknowns:
        Vec<String>,

    pub unresolved_critiques:
        usize,
}

impl Default for CognitiveState {
    fn default() -> Self {
        Self {
            cycle_id: None,
            status: CognitiveStateStatus::Idle,
            phase: None,
            depth: CognitiveDepth::Minimal,
            progress: 0.0,
            blocking_reasons: Vec::new(),
            unresolved_unknowns: Vec::new(),
            unresolved_critiques: 0,
        }
    }
}
