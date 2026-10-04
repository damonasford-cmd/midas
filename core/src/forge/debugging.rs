use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugFinding {
    pub id: Uuid,
    pub location: String,
    pub message: String,
    pub severity: DebugSeverity,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Default)]
pub struct Debugger;

impl Debugger {
    pub fn new() -> Self {
        Self
    }

    pub fn finding(
        &self,
        location: impl Into<String>,
        message: impl Into<String>,
        severity: DebugSeverity,
    ) -> DebugFinding {
        DebugFinding {
            id: Uuid::new_v4(),
            location: location.into(),
            message: message.into(),
            severity,
            suggested_fix: None,
        }
    }

    pub fn with_fix(
        &self,
        mut finding: DebugFinding,
        fix: impl Into<String>,
    ) -> DebugFinding {
        finding.suggested_fix = Some(fix.into());
        finding
    }
}
