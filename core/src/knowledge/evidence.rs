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
pub enum EvidenceKind {
    Direct,
    Experimental,
    Observational,
    Documentary,
    Statistical,
    Computational,
    Testimonial,
    Corroborating,
    Contradicting,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum EvidenceStrength {
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,

    pub kind: EvidenceKind,

    pub strength: EvidenceStrength,

    pub supports: Option<KnowledgeId>,

    pub contradicts: Option<KnowledgeId>,

    pub description: String,

    pub reliability: f32,
}

impl Evidence {
    pub fn new(
        kind: EvidenceKind,
        strength: EvidenceStrength,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                "evidence description cannot be empty".into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            strength,
            supports: None,
            contradicts: None,
            description,
            reliability: 0.5,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.reliability) {
            return Err(
                "evidence reliability must be between 0 and 1"
                    .into(),
            );
        }

        if self.supports.is_some()
            && self.contradicts.is_some()
        {
            return Err(
                "evidence cannot simultaneously support and contradict the same knowledge item"
                    .into(),
            );
        }

        Ok(())
    }
}
