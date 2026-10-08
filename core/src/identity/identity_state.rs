use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum IdentityState {
    Initializing,
    Active,
    Degraded,
    Suspended,
    Recovering,
    Shutdown,
    Corrupted,
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
pub enum IdentityOperationalState {
    Stable,
    Evolving,
    Recovering,
    Restricted,
    IntegrityFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityStateSnapshot {
    pub state: IdentityState,
    pub operational_state: IdentityOperationalState,
    pub changed_at: DateTime<Utc>,
    pub reason: Option<String>,
}

impl IdentityStateSnapshot {
    pub fn new(
        state: IdentityState,
        operational_state: IdentityOperationalState,
        reason: Option<String>,
    ) -> Self {
        Self {
            state,
            operational_state,
            changed_at: Utc::now(),
            reason,
        }
    }
}
