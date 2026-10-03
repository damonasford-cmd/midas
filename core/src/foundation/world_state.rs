use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::contracts::Observation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub updated_at: DateTime<Utc>,
    pub observations: HashMap<String, Observation>,
    pub facts: HashMap<String, Value>,
    pub unknowns: Vec<String>,
    pub active_events: Vec<String>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            updated_at: Utc::now(),
            observations: HashMap::new(),
            facts: HashMap::new(),
            unknowns: Vec::new(),
            active_events: Vec::new(),
        }
    }
}

impl WorldState {
    pub fn record_observation(&mut self, observation: Observation) {
        self.updated_at = Utc::now();

        self.observations.insert(
            observation.id.to_string(),
            observation,
        );
    }

    pub fn set_fact(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) {
        self.updated_at = Utc::now();
        self.facts.insert(key.into(), value);
    }

    pub fn register_unknown(
        &mut self,
        unknown: impl Into<String>,
    ) {
        let unknown = unknown.into();

        if !self.unknowns.contains(&unknown) {
            self.unknowns.push(unknown);
        }

        self.updated_at = Utc::now();
    }

    pub fn register_event(
        &mut self,
        event: impl Into<String>,
    ) {
        self.active_events.push(event.into());
        self.updated_at = Utc::now();
    }
}
