use chrono::{DateTime, Utc};
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
pub enum SelfReferenceKind {
    Internal,
    Public,
    CreatorRelation,
    SystemRelation,
    Historical,
    Operational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfReference {
    pub id: Uuid,
    pub kind: SelfReferenceKind,
    pub name: String,
    pub meaning: String,
    pub canonical: bool,
    pub created_at: DateTime<Utc>,
}

impl SelfReference {
    pub fn new(
        kind: SelfReferenceKind,
        name: impl Into<String>,
        meaning: impl Into<String>,
        canonical: bool,
    ) -> Result<Self, String> {
        let name = name.into();
        let meaning = meaning.into();

        if name.trim().is_empty() {
            return Err(
                "self-reference name cannot be empty"
                    .into(),
            );
        }

        if meaning.trim().is_empty() {
            return Err(
                "self-reference meaning cannot be empty"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            name,
            meaning,
            canonical,
            created_at: Utc::now(),
        })
    }
}
