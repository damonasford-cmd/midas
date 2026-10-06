use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
pub enum CritiqueSeverity {
    Informational,
    Minor,
    Significant,
    Major,
    Critical,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum CritiqueType {
    Logical,
    Factual,
    Causal,
    Temporal,
    Statistical,
    Resource,
    Security,
    Safety,
    Authorization,
    Requirement,
    Consistency,
    Completeness,
    Reversibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Critique {
    pub id: Uuid,

    pub critique_type:
        CritiqueType,

    pub severity:
        CritiqueSeverity,

    pub description: String,

    pub affected_element:
        Option<String>,

    pub correction_required:
        bool,

    pub resolved:
        bool,
}

impl Critique {
    pub fn new(
        critique_type: CritiqueType,
        severity: CritiqueSeverity,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                "critique description cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            critique_type,
            severity,
            description,
            affected_element: None,
            correction_required: severity
                >= CritiqueSeverity::Significant,
            resolved: false,
        })
    }

    pub fn blocks_decision(&self) -> bool {
        matches!(
            self.severity,
            CritiqueSeverity::Major
                | CritiqueSeverity::Critical
        )
    }
}
