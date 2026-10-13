use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityState {
    Declared,
    Available,
    Degraded,
    Unavailable,
    Suspended,
    Retired,
}

impl CapabilityState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Declared => "declared",
            Self::Available => "available",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Suspended => "suspended",
            Self::Retired => "retired",
        }
    }

    pub fn can_transition_to(self, next: Self) -> bool {
        use CapabilityState::*;

        if self == next {
            return true;
        }

        match self {
            Declared => matches!(
                next,
                Available | Degraded | Unavailable | Suspended | Retired
            ),
            Available => matches!(
                next,
                Degraded | Unavailable | Suspended | Retired
            ),
            Degraded => matches!(
                next,
                Available | Unavailable | Suspended | Retired
            ),
            Unavailable => matches!(
                next,
                Available | Degraded | Suspended | Retired
            ),
            Suspended => matches!(
                next,
                Declared | Available | Degraded | Unavailable | Retired
            ),
            Retired => false,
        }
    }

    pub fn can_perform_actions(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }
}

impl std::fmt::Display for CapabilityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for CapabilityState {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "declared" => Ok(Self::Declared),
            "available" => Ok(Self::Available),
            "degraded" => Ok(Self::Degraded),
            "unavailable" => Ok(Self::Unavailable),
            "suspended" => Ok(Self::Suspended),
            "retired" => Ok(Self::Retired),
            other => Err(format!("Unknown capability state: {other}")),
        }
    }
}
