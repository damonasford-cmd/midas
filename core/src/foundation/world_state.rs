use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde_json::Value;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::contracts::{Event, Observation};

#[derive(Debug, Clone)]
pub struct WorldState {
    pub updated_at: DateTime<Utc>,
    pub observations: HashMap<Uuid, Observation>,
    pub facts: HashMap<String, Value>,
    pub unknowns: Vec<String>,
    pub active_events: Vec<Event>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            updated_at: Utc::now(),
            observations: HashMap::new(),
            facts: HashMap::new(),
            unknowns: Vec::new(),
            active_events: Vec::new(),
        }
    }

    pub fn record_observation(&mut self, observation: Observation) {
        self.updated_at = Utc::now();
        self.observations.insert(observation.id, observation);
    }

    pub fn set_fact(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) {
        self.updated_at = Utc::now();
        self.facts.insert(key.into(), value);
    }

    pub fn register_unknown(&mut self, unknown: impl Into<String>) {
        let unknown = unknown.into();

        if !self.unknowns.contains(&unknown) {
            self.unknowns.push(unknown);
        }

        self.updated_at = Utc::now();
    }

    pub fn register_event(&mut self, event: Event) {
        self.updated_at = Utc::now();
        self.active_events.push(event);
    }

    pub fn observation_count(&self) -> usize {
        self.observations.len()
    }

    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }
}

pub type SharedWorldState = Arc<RwLock<WorldState>>;
