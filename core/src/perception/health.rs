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
pub enum PerceptionHealthState {
    Healthy,
    Degraded,
    Partial,
    Blocked,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionHealth {
    pub state:
        PerceptionHealthState,

    pub active:
        bool,

    pub inputs_received:
        u64,

    pub inputs_processed:
        u64,

    pub inputs_failed:
        u64,

    pub observations_created:
        u64,

    pub anomalies_detected:
        u64,

    pub degraded_sources:
        u64,

    pub fusion_available:
        bool,

    pub multimodal_available:
        bool,

    pub provenance_available:
        bool,

    pub uncertainty_available:
        bool,
}
