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
pub enum VerificationLevel {
    Basic,
    Standard,
    Deep,
    Independent,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    pub id: Uuid,

    pub level:
        VerificationLevel,

    pub target: String,

    pub checks: Vec<String>,

    pub passed: bool,

    pub independent:
        bool,

    pub evidence:
        Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub verification_id: Uuid,

    pub passed: bool,

    pub confidence: f32,

    pub failures: Vec<String>,

    pub recommendations:
        Vec<String>,
}

impl Verification {
    pub fn new(
        target: impl Into<String>,
        level: VerificationLevel,
    ) -> Result<Self, String> {
        let target = target.into();

        if target.trim().is_empty() {
            return Err(
                "verification target cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            level,
            target,
            checks: Vec::new(),
            passed: false,
            independent: matches!(
                level,
                VerificationLevel::Independent
                    | VerificationLevel::Critical
            ),
            evidence: Vec::new(),
        })
    }
}

impl VerificationResult {
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(
                "verification confidence must be between 0 and 1"
                    .into()
            );
        }

        Ok(())
    }
}
