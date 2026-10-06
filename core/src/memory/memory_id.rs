use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
pub struct MemoryId(Uuid);

impl MemoryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for MemoryId {
    fn default() -> Self {
        Self::new()
    }
}
