use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub type EntityId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityKind {
    Human,
    Organization,
    Company,
    Government,
    Institution,
    Machine,
    Device,
    Software,
    ArtificialIntelligence,
    Vehicle,
    Infrastructure,
    Location,
    NaturalObject,
    Resource,
    Event,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityState {
    Unknown,
    Active,
    Inactive,
    Available,
    Unavailable,
    Destroyed,
    Lost,
    Disconnected,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub kind: EntityKind,
    pub name: String,
    pub description: Option<String>,
    pub state: EntityState,
    pub attributes: serde_json::Map<String, Value>,
    pub first_observed_at: DateTime<Utc>,
    pub last_observed_at: DateTime<Utc>,
}

impl Entity {
    pub fn new(
        kind: EntityKind,
        name: impl Into<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            kind,
            name: name.into(),
            description: None,
            state: EntityState::Unknown,
            attributes: serde_json::Map::new(),
            first_observed_at: now,
            last_observed_at: now,
        }
    }

    pub fn update_observation_time(&mut self) {
        self.last_observed_at = Utc::now();
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = Some(description.into());
        self.update_observation_time();
    }

    pub fn set_state(&mut self, state: EntityState) {
        self.state = state;
        self.update_observation_time();
    }

    pub fn set_attribute(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) {
        self.attributes.insert(key.into(), value);
        self.update_observation_time();
    }

    pub fn get_attribute(&self, key: &str) -> Option<&Value> {
        self.attributes.get(key)
    }
}
