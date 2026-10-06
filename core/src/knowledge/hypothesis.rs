use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::knowledge_id::KnowledgeId;

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
pub struct Hypothesis {
    pub id: Uuid,

    pub statement: KnowledgeId,

    pub status: HypothesisStatus,

    pub probability: f32,

    pub tests_performed: u32,
}

impl Hypothesis {
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
