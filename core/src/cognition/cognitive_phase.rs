use serde::{Deserialize, Serialize};

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
pub enum CognitivePhase {
    Objective,
    Perceive,
    Understand,
    Decompose,
    IdentifyMissingCapabilities,
    Research,
    Hypotheses,
    Reason,
    Simulate,
    Critique,
    Verify,
    Decide,
    Act,
    Observe,
    Measure,
    Learn,
    Correct,
    Improve,
    Evolve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitivePhaseResult {
    pub phase: CognitivePhase,
    pub completed: bool,
    pub success: bool,
    pub summary: Option<String>,
    pub blocking: bool,
}

impl CognitivePhaseResult {
    pub fn success(
        phase: CognitivePhase,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            completed: true,
            success: true,
            summary: Some(summary.into()),
            blocking: false,
        }
    }

    pub fn blocked(
        phase: CognitivePhase,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            completed: true,
            success: false,
            summary: Some(summary.into()),
            blocking: true,
        }
    }
}
