use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::contracts::{
    Event,
    Observation,
};

#[derive(Debug, Clone)]
pub struct WorldState {
    pub updated_at: DateTime<Utc>,
    pub observations: Vec<Observation>,
    pub facts: Vec<String>,
    pub unknowns: Vec<String>,
    pub active_events: Vec<Event>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            updated_at: Utc::now(),
            observations: Vec::new(),
            facts: Vec::new(),
            unknowns: Vec::new(),
            active_events: Vec::new(),
        }
    }

    pub fn add_observation(
        &mut self,
        observation: Observation,
    ) {
        self.observations.push(observation);
        self.updated_at = Utc::now();
    }

    pub fn add_fact(
        &mut self,
        fact: impl Into<String>,
    ) {
        let fact = fact.into();

        if !fact.trim().is_empty() {
            self.facts.push(fact);
            self.updated_at = Utc::now();
        }
    }

    pub fn add_unknown(
        &mut self,
        unknown: impl Into<String>,
    ) {
        let unknown = unknown.into();

        if !unknown.trim().is_empty() {
            self.unknowns.push(unknown);
            self.updated_at = Utc::now();
        }
    }

    pub fn register_event(
        &mut self,
        event: Event,
    ) {
        self.active_events.push(event);
        self.updated_at = Utc::now();
    }

    pub fn observation_count(&self) -> usize {
        self.observations.len()
    }

    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }

    pub fn unknown_count(&self) -> usize {
        self.unknowns.len()
    }

    pub fn has_event(
        &self,
        event_id: Uuid,
    ) -> bool {
        self.active_events
            .iter()
            .any(|event| event.id == event_id)
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

pub type SharedWorldState =
    Arc<RwLock<WorldState>>;
