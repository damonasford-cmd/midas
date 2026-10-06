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
pub enum KnowledgeRelationKind {
    Supports,
    Contradicts,
    DerivedFrom,
    DependsOn,
    Causes,
    CausedBy,
    SimilarTo,
    Generalizes,
    Specializes,
    Refines,
    Replaces,
    References,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRelation {
    pub id: Uuid,

    pub from: KnowledgeId,
    pub to: KnowledgeId,

    pub kind: KnowledgeRelationKind,

    pub strength: f32,
}

impl KnowledgeRelation {
    pub fn validate(&self) -> Result<(), String> {
        if self.from == self.to {
            return Err(
                "knowledge relation cannot point to itself"
                    .into()
            );
        }

        if !(0.0..=1.0).contains(&self.strength) {
            return Err(
                "relation strength must be between 0 and 1"
                    .into()
            );
        }

        Ok(())
    }
}
