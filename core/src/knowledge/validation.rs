use serde::{Deserialize, Serialize};

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
pub enum ValidationStatus {
    NotValidated,
    InProgress,
    Validated,
    Failed,
    NeedsReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeValidation {
    pub knowledge_id: KnowledgeId,

    pub status: ValidationStatus,

    pub checks_performed: Vec<String>,

    pub validator: Option<String>,

    pub notes: Option<String>,
}

impl KnowledgeValidation {
    pub fn new(
        knowledge_id: KnowledgeId,
    ) -> Self {
        Self {
            knowledge_id,
            status: ValidationStatus::NotValidated,
            checks_performed: Vec::new(),
            validator: None,
            notes: None,
        }
    }

    pub fn add_check(
        &mut self,
        check: impl Into<String>,
    ) {
        self.checks_performed.push(check.into());
    }
}
