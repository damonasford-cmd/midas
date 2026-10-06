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
pub enum HypothesisStatus {
    Proposed,
    Testing,
    Supported,
    Weakened,
    Rejected,
    Confirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveHypothesis {
    pub id: Uuid,

    pub statement: String,

    pub probability: f32,

    pub status: HypothesisStatus,

    pub supporting_evidence: Vec<String>,

    pub contradicting_evidence: Vec<String>,

    pub required_tests: Vec<String>,
}

impl CognitiveHypothesis {
    pub fn new(
        statement: impl Into<String>,
    ) -> Result<Self, String> {
        let statement = statement.into();

        if statement.trim().is_empty() {
            return Err(
                "hypothesis statement cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            statement,
            probability: 0.5,
            status: HypothesisStatus::Proposed,
            supporting_evidence: Vec::new(),
            contradicting_evidence: Vec::new(),
            required_tests: Vec::new(),
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.probability) {
            return Err(
                "hypothesis probability must be between 0 and 1"
                    .into()
            );
        }

        Ok(())
    }
}
