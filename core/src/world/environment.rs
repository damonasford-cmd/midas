use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnvironmentKind {
    Physical,
    Digital,
    Financial,
    Economic,
    Social,
    Political,
    Scientific,
    Industrial,
    Natural,
    Space,
    Mixed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: Uuid,
    pub kind: EnvironmentKind,
    pub name: String,
    pub description: Option<String>,
    pub state: serde_json::Map<String, Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Environment {
    pub fn new(
        kind: EnvironmentKind,
        name: impl Into<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            kind,
            name: name.into(),
            description: None,
            state: serde_json::Map::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_state(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) {
        self.state.insert(key.into(), value);
        self.updated_at = Utc::now();
    }

    pub fn get_state(&self, key: &str) -> Option<&Value> {
        self.state.get(key)
    }
}
