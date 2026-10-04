use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAction {
    pub id: Uuid,
    pub failure: String,
    pub action: String,
    pub reversible: bool,
}

#[derive(Debug, Clone, Default)]
pub struct RecoveryEngine;

impl RecoveryEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn propose(
        &self,
        failure: impl Into<String>,
        action: impl Into<String>,
        reversible: bool,
    ) -> RecoveryAction {
        RecoveryAction {
            id: Uuid::new_v4(),
            failure: failure.into(),
            action: action.into(),
            reversible,
        }
    }
}
