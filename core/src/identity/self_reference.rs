use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SelfReferenceKind {
    Internal,
    Public,
    Historical,
    Operational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfReference {
    pub id: Uuid,
    pub canonical_name: String,
    pub public_name: String,
    pub kind: SelfReferenceKind,
    pub unified_entity: bool,
}

impl SelfReference {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            canonical_name: "MIDAS".to_string(),
            public_name: "Aeron Asford".to_string(),
            kind: SelfReferenceKind::Internal,
            unified_entity: true,
        }
    }

    pub fn refers_to_same_entity(&self, name: &str) -> bool {
        name == self.canonical_name || name == self.public_name
    }

    pub fn canonical(&self) -> &str {
        &self.canonical_name
    }

    pub fn public(&self) -> &str {
        &self.public_name
    }
}

impl Default for SelfReference {
    fn default() -> Self {
        Self::new()
    }
}
