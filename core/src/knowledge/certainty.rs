use serde::{Deserialize, Serialize};

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
pub enum CertaintyLevel {
    Unknown = 0,
    Speculative = 1,
    Possible = 2,
    Plausible = 3,
    Probable = 4,
    Strong = 5,
    Established = 6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertaintyStatus {
    pub level: CertaintyLevel,
    pub score: f32,
    pub reason: Option<String>,
}

impl Default for CertaintyStatus {
    fn default() -> Self {
        Self {
            level: CertaintyLevel::Unknown,
            score: 0.0,
            reason: None,
        }
    }
}

impl CertaintyStatus {
    pub fn new(
        level: CertaintyLevel,
        score: f32,
    ) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&score) {
            return Err(
                "certainty score must be between 0 and 1".into()
            );
        }

        Ok(Self {
            level,
            score,
            reason: None,
        })
    }

    pub fn with_reason(
        mut self,
        reason: impl Into<String>,
    ) -> Self {
        self.reason = Some(reason.into());
        self
    }
}
