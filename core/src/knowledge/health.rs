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
pub enum KnowledgeHealthState {
    Healthy,
    Degraded,
    Contradictory,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeHealth {
    pub state: KnowledgeHealthState,

    pub total_items: usize,

    pub facts: usize,
    pub hypotheses: usize,
    pub unknowns: usize,

    pub unresolved_contradictions: usize,

    pub validated_items: usize,

    pub integrity_errors: usize,

    pub retrieval_available: bool,
}
