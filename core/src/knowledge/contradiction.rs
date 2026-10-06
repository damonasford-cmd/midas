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
pub enum ContradictionResolution {
    Unresolved,
    PreferHigherConfidence,
    PreferHigherAuthority,
    PreferMoreRecent,
    ContextDependent,
    BothMayBeValid,
    RejectedFirst,
    RejectedSecond,
    RequiresResearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    pub id: Uuid,

    pub first: KnowledgeId,
    pub second: KnowledgeId,

    pub detected_at: chrono::DateTime<chrono::Utc>,

    pub resolution: ContradictionResolution,

    pub explanation: Option<String>,
}

impl Contradiction {
    pub fn new(
        first: KnowledgeId,
        second: KnowledgeId,
    ) -> Result<Self, String> {
        if first == second {
            return Err(
                "knowledge cannot contradict itself".into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            first,
            second,
            detected_at: chrono::Utc::now(),
            resolution:
                ContradictionResolution::Unresolved,
            explanation: None,
        })
    }
}

#[derive(Debug, Default)]
pub struct ContradictionSet {
    contradictions: Vec<Contradiction>,
}

impl ContradictionSet {
    pub fn add(
        &mut self,
        contradiction: Contradiction,
    ) {
        self.contradictions.push(contradiction);
    }

    pub fn all(&self) -> &[Contradiction] {
        &self.contradictions
    }

    pub fn unresolved(&self) -> Vec<Contradiction> {
        self.contradictions
            .iter()
            .filter(|item| {
                item.resolution
                    == ContradictionResolution::Unresolved
            })
            .cloned()
            .collect()
    }
}
