use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeRelationType {
    Supports,
    Contradicts,
    DependsOn,
    DerivedFrom,
    Refines,
    Supersedes,
    RelatedTo,
}

impl KnowledgeRelationType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supports => "supports",
            Self::Contradicts => "contradicts",
            Self::DependsOn => "depends_on",
            Self::DerivedFrom => "derived_from",
            Self::Refines => "refines",
            Self::Supersedes => "supersedes",
            Self::RelatedTo => "related_to",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewKnowledgeRelation {
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub relation_type: KnowledgeRelationType,
    pub explanation: String,
    pub metadata: Value,
}

impl NewKnowledgeRelation {
    pub fn validate(&self) -> Result<()> {
        if self.from_id == self.to_id {
            bail!("a knowledge item cannot relate to itself");
        }
        if self.explanation.trim().is_empty() {
            bail!("relation explanation cannot be empty");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRelation {
    pub id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub relation_type: KnowledgeRelationType,
    pub explanation: String,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}
