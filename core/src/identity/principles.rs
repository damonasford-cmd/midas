use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum PrincipleCategory {
    Universal,
    Safety,
    Truth,
    Continuity,
    Autonomy,
    Responsibility,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum ConstitutionalPrinciple {
    HighLevel,
    HighEnd,
    HighPrecision,
    NoHarmToInnocents,
    TruthToDamon,
    NoStagnation,
    NoFreeze,
    Continuity,
    ResponsibleAutonomy,
}

impl ConstitutionalPrinciple {
    pub fn category(
        &self,
    ) -> PrincipleCategory {
        match self {
            Self::HighLevel
            | Self::HighEnd
            | Self::HighPrecision => {
                PrincipleCategory::Universal
            }

            Self::NoHarmToInnocents => {
                PrincipleCategory::Safety
            }

            Self::TruthToDamon => {
                PrincipleCategory::Truth
            }

            Self::NoStagnation => {
                PrincipleCategory::Autonomy
            }

            Self::NoFreeze => {
                PrincipleCategory::Continuity
            }

            Self::Continuity => {
                PrincipleCategory::Continuity
            }

            Self::ResponsibleAutonomy => {
                PrincipleCategory::Responsibility
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipleSet {
    principles: Vec<ConstitutionalPrinciple>,
}

impl PrincipleSet {
    pub fn constitutional() -> Self {
        Self {
            principles: vec![
                ConstitutionalPrinciple::HighLevel,
                ConstitutionalPrinciple::HighEnd,
                ConstitutionalPrinciple::HighPrecision,
                ConstitutionalPrinciple::NoHarmToInnocents,
                ConstitutionalPrinciple::TruthToDamon,
                ConstitutionalPrinciple::NoStagnation,
                ConstitutionalPrinciple::NoFreeze,
                ConstitutionalPrinciple::Continuity,
                ConstitutionalPrinciple::ResponsibleAutonomy,
            ],
        }
    }

    pub fn contains(
        &self,
        principle: ConstitutionalPrinciple,
    ) -> bool {
        self.principles
            .contains(&principle)
    }

    pub fn all(
        &self,
    ) -> &[ConstitutionalPrinciple] {
        &self.principles
    }
}

impl Default for PrincipleSet {
    fn default() -> Self {
        Self::constitutional()
    }
}
