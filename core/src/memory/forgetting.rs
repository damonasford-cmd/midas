use serde::{Deserialize, Serialize};

use super::memory_record::MemoryRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForgettingReason {
    UserRequest,
    RetentionPolicy,
    ObsoleteInformation,
    Duplicate,
    Corrupted,
    LowValue,
    Privacy,
    StoragePressure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgettingPolicy {
    pub minimum_importance_to_keep: f32,
    pub minimum_confidence_to_keep: f32,
    pub preserve_user_requested_memories: bool,
}

impl Default for ForgettingPolicy {
    fn default() -> Self {
        Self {
            minimum_importance_to_keep: 0.1,
            minimum_confidence_to_keep: 0.05,
            preserve_user_requested_memories: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForgettingDecision {
    Keep,
    Archive,
    Forget,
}

impl ForgettingPolicy {
    pub fn evaluate(
        &self,
        record: &MemoryRecord,
    ) -> ForgettingDecision {
        if record.metadata.importance
            >= self.minimum_importance_to_keep
        {
            return ForgettingDecision::Keep;
        }

        if record.metadata.confidence
            < self.minimum_confidence_to_keep
        {
            return ForgettingDecision::Archive;
        }

        ForgettingDecision::Keep
    }
}
