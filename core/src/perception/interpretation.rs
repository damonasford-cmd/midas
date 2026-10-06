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
pub enum InterpretationKind {
    Semantic,
    Visual,
    Auditory,
    Linguistic,
    Spatial,
    Temporal,
    Behavioral,
    Causal,
    Environmental,
    Operational,
    Anomaly,
    Intent,
    Context,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interpretation {
    pub id: Uuid,

    pub kind:
        InterpretationKind,

    pub statement:
        String,

    pub confidence:
        f32,

    pub supporting_observations:
        Vec<Uuid>,

    pub assumptions:
        Vec<String>,

    pub uncertainty:
        Vec<String>,

    pub verified:
        bool,
}

impl Interpretation {
    pub fn new(
        kind: InterpretationKind,
        statement: impl Into<String>,
        confidence: f32,
    ) -> Result<Self, String> {
        let statement = statement.into();

        if statement.trim().is_empty() {
            return Err(
                "interpretation statement cannot be empty"
                    .into(),
            );
        }

        if !(0.0..=1.0).contains(&confidence) {
            return Err(
                "interpretation confidence must be between 0 and 1"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            statement,
            confidence,
            supporting_observations: Vec::new(),
            assumptions: Vec::new(),
            uncertainty: Vec::new(),
            verified: false,
        })
    }
}
