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
pub enum DecisionKind {
    Proceed,
    ProceedWithConditions,
    Delay,
    ResearchMore,
    Revise,
    Reject,
    Escalate,
    RequestAuthorization,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DecisionConfidence {
    pub score: f32,
}

impl DecisionConfidence {
    pub fn new(score: f32) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&score) {
            return Err(
                "decision confidence must be between 0 and 1"
                    .into()
            );
        }

        Ok(Self { score })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: Uuid,

    pub kind:
        DecisionKind,

    pub confidence:
        DecisionConfidence,

    pub rationale:
        Vec<String>,

    pub unresolved_risks:
        Vec<String>,

    pub unresolved_unknowns:
        Vec<String>,

    pub authorization_required:
        bool,

    pub reversible:
        bool,

    pub validated:
        bool,
}

impl Decision {
    pub fn new(
        kind: DecisionKind,
        confidence: f32,
    ) -> Result<Self, String> {
        Ok(Self {
            id: Uuid::new_v4(),

            kind,

            confidence:
                DecisionConfidence::new(
                    confidence,
                )?,

            rationale: Vec::new(),

            unresolved_risks: Vec::new(),

            unresolved_unknowns: Vec::new(),

            authorization_required: false,

            reversible: true,

            validated: false,
        })
    }

    pub fn is_executable(&self) -> bool {
        self.validated
            && !self.authorization_required
            && matches!(
                self.kind,
                DecisionKind::Proceed
                    | DecisionKind::ProceedWithConditions
            )
    }
}
