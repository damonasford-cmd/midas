use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityState {
    Registered,
    Available,
    Degraded,
    Unavailable,
    Suspended,
    Retired,
}

impl CapabilityState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Registered => "registered",
            Self::Available => "available",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Suspended => "suspended",
            Self::Retired => "retired",
        }
    }

    pub fn can_attempt_use(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    pub fn validate_transition(
        self,
        next: Self,
        has_current_evaluation: bool,
    ) -> Result<()> {
        if self == next {
            return Ok(());
        }

        let allowed = match self {
            Self::Registered => matches!(
                next,
                Self::Available
                    | Self::Unavailable
                    | Self::Suspended
                    | Self::Retired
            ),
            Self::Available => matches!(
                next,
                Self::Degraded
                    | Self::Unavailable
                    | Self::Suspended
                    | Self::Retired
            ),
            Self::Degraded => matches!(
                next,
                Self::Available
                    | Self::Unavailable
                    | Self::Suspended
                    | Self::Retired
            ),
            Self::Unavailable => matches!(
                next,
                Self::Available
                    | Self::Degraded
                    | Self::Suspended
                    | Self::Retired
            ),
            Self::Suspended => matches!(
                next,
                Self::Registered
                    | Self::Unavailable
                    | Self::Retired
            ),
            Self::Retired => false,
        };

        if !allowed {
            bail!(
                "capability state transition is not allowed: {} -> {}",
                self.as_str(),
                next.as_str()
            );
        }

        if next == Self::Available && !has_current_evaluation {
            bail!("a current successful evaluation is required for availability");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStateChange {
    pub previous: CapabilityState,
    pub next: CapabilityState,
    pub reason: String,
    pub actor: String,
    pub changed_at: DateTime<Utc>,
}

impl CapabilityStateChange {
    pub fn new(
        previous: CapabilityState,
        next: CapabilityState,
        reason: impl Into<String>,
        actor: impl Into<String>,
        has_current_evaluation: bool,
    ) -> Result<Self> {
        let reason = reason.into();
        let actor = actor.into();

        if reason.trim().is_empty() {
            bail!("state change reason cannot be empty");
        }
        if actor.trim().is_empty() {
            bail!("state change actor cannot be empty");
        }

        previous.validate_transition(next, has_current_evaluation)?;

        Ok(Self {
            previous,
            next,
            reason,
            actor,
            changed_at: Utc::now(),
        })
    }
}
