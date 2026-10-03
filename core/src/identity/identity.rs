use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidasIdentity {
    pub system_name: String,
    pub public_name: String,
    pub creator: String,
    pub version: String,
    pub state: IdentityState,

    pub unified: bool,
    pub innate_capabilities: bool,

    pub principles: Principles,

    pub mission: String,

    pub last_state_change: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityState {
    Initializing,
    Operational,
    Degraded,
    Maintenance,
    Emergency,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principles {
    pub high_level: bool,
    pub high_end: bool,
    pub high_precision: bool,
}

impl Default for MidasIdentity {
    fn default() -> Self {
        Self {
            system_name: "MIDAS".to_string(),
            public_name: "Aeron Asford".to_string(),
            creator: "Damon".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            state: IdentityState::Initializing,

            unified: true,
            innate_capabilities: true,

            principles: Principles {
                high_level: true,
                high_end: true,
                high_precision: true,
            },

            mission: "Comprendre, créer, construire, agir, apprendre et évoluer dans le monde réel afin de produire une valeur réelle et mesurable.".to_string(),

            last_state_change: Utc::now(),
        }
    }
}

impl MidasIdentity {
    pub fn set_state(&mut self, state: IdentityState) {
        self.state = state;
        self.last_state_change = Utc::now();
    }

    pub fn is_operational(&self) -> bool {
        matches!(self.state, IdentityState::Operational)
    }

    pub fn summary(&self) -> String {
        format!(
            "{} / {} / version {} / {:?}",
            self.system_name,
            self.public_name,
            self.version,
            self.state
        )
    }
}
