use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    modality::PerceptionModality,
    payload::PerceptionPayload,
    source::PerceptionSource,
};

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
pub struct PerceptionInputId(Uuid);

impl PerceptionInputId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for PerceptionInputId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum PerceptionInputState {
    Received,
    Accepted,
    Processing,
    Processed,
    Rejected,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionInput {
    pub id: PerceptionInputId,

    pub modality: PerceptionModality,

    pub source: PerceptionSource,

    pub payload: PerceptionPayload,

    pub state: PerceptionInputState,

    pub received_at:
        chrono::DateTime<chrono::Utc>,
}

impl PerceptionInput {
    pub fn new(
        modality: PerceptionModality,
        source: PerceptionSource,
        payload: PerceptionPayload,
    ) -> Self {
        Self {
            id: PerceptionInputId::new(),
            modality,
            source,
            payload,
            state: PerceptionInputState::Received,
            received_at: chrono::Utc::now(),
        }
    }
}
