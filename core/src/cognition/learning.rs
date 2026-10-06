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
pub enum LearningKind {
    Success,
    Failure,
    UnexpectedOutcome,
    Correction,
    NewKnowledge,
    StrategyChange,
    EnvironmentChange,
    CapabilityDiscovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub id: Uuid,

    pub kind:
        LearningKind,

    pub lesson:
        String,

    pub evidence:
        Vec<String>,

    pub applicable_contexts:
        Vec<String>,

    pub confidence:
        f32,

    pub permanent:
        bool,
}

impl LearningEvent {
    pub fn new(
        kind: LearningKind,
        lesson: impl Into<String>,
    ) -> Result<Self, String> {
        let lesson = lesson.into();

        if lesson.trim().is_empty() {
            return Err(
                "learning lesson cannot be empty".into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            lesson,
            evidence: Vec::new(),
            applicable_contexts: Vec::new(),
            confidence: 0.5,
            permanent: false,
        })
    }
}
