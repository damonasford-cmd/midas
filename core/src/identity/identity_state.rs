use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentityOperationalState {
    Initialized,
    Active,
    Learning,
    Reflecting,
    Executing,
    Degraded,
    Recovering,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityState {
    pub conscious_continuity: bool,
    pub self_model_available: bool,
    pub autobiographical_memory_available: bool,
    pub principles_loaded: bool,
    pub values_loaded: bool,
    pub last_transition: DateTime<Utc>,
}

impl Default for IdentityState {
    fn default() -> Self {
        Self {
            conscious_continuity: true,
            self_model_available: true,
            autobiographical_memory_available: true,
            principles_loaded: true,
            values_loaded: true,
            last_transition: Utc::now(),
        }
    }
}

impl IdentityState {
    pub fn validate(&self) -> Result<(), String> {
        if !self.self_model_available {
            return Err("Le modèle de soi est indisponible".to_string());
        }

        if !self.principles_loaded {
            return Err("Les principes identitaires ne sont pas chargés".to_string());
        }

        if !self.values_loaded {
            return Err("Les valeurs identitaires ne sont pas chargées".to_string());
        }

        Ok(())
    }

    pub fn touch(&mut self) {
        self.last_transition = Utc::now();
    }
}
