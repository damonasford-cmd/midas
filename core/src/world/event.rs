use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorldEventKind {
    Observation,
    Creation,
    Modification,
    Destruction,
    Connection,
    Disconnection,
    Transaction,
    Movement,
    Communication,
    Discovery,
    Failure,
    Recovery,
    StateChange,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: Uuid,
    pub kind: WorldEventKind,
    pub timestamp: DateTime<Utc>,
    pub subject_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub description: String,
    pub data: serde_json::Map<String, Value>,
}

impl WorldEvent {
    pub fn new(
        kind: WorldEventKind,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            timestamp: Utc::now(),
            subject_id: None,
            location_id: None,
            description: description.into(),
            data: serde_json::Map::new(),
        }
    }

    pub fn attach_subject(&mut self, id: Uuid) {
        self.subject_id = Some(id);
    }

    pub fn attach_location(&mut self, id: Uuid) {
        self.location_id = Some(id);
    }

    pub fn add_data(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) {
        self.data.insert(key.into(), value);
    }
}
