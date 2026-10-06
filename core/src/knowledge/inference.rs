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
pub enum InferenceStatus {
    Proposed,
    Validated,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InferenceRule {
    Deduction,
    Induction,
    Abduction,
    Analogy,
    Statistical,
    Causal,
    Temporal,
    Computational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inference {
    pub id: Uuid,

    pub conclusion: KnowledgeId,

    pub premises: Vec<KnowledgeId>,

    pub rule: InferenceRule,

    pub confidence: f32,

    pub status: InferenceStatus,

    pub explanation: Option<String>,
}

impl Inference {
    pub fn validate(&self) -> Result<(), String> {
        if self.premises.is_empty() {
            return Err(
                "inference must contain at least one premise"
                    .into()
            );
        }

        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(
                "inference confidence must be between 0 and 1"
                    .into()
            );
        }

        Ok(())
    }
}
