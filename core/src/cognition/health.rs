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
pub enum CognitionHealthState {
    Healthy,
    Degraded,
    Blocked,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionHealth {
    pub state:
        CognitionHealthState,

    pub active:
        bool,

    pub cycles_started:
        u64,

    pub cycles_completed:
        u64,

    pub cycles_failed:
        u64,

    pub blocked_cycles:
        u64,

    pub unresolved_unknowns:
        usize,

    pub unresolved_critiques:
        usize,

    pub decision_quality:
        f32,

    pub reasoning_available:
        bool,

    pub verification_available:
        bool,

    pub simulation_available:
        bool,
}
