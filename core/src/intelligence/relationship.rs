use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub name: String,
    pub organization: Option<String>,
    pub role: Option<String>,
    pub communication_channels: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RelationshipEngine;

impl RelationshipEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        name: impl Into<String>,
    ) -> Relationship {
        Relationship {
            id: Uuid::new_v4(),
            name: name.into(),
            organization: None,
            role: None,
            communication_channels: Vec::new(),
            notes: Vec::new(),
        }
    }
}
