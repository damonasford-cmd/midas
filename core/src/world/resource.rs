use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResourceKind {
    Compute,
    Memory,
    Storage,
    Network,
    Energy,
    Money,
    Data,
    Information,
    HumanTime,
    Material,
    Equipment,
    Access,
    Capability,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResourceState {
    Available,
    Allocated,
    Depleted,
    Unavailable,
    Reserved,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub kind: ResourceKind,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub state: ResourceState,
    pub attributes: serde_json::Map<String, Value>,
    pub updated_at: DateTime<Utc>,
}

impl Resource {
    pub fn new(
        kind: ResourceKind,
        name: impl Into<String>,
        quantity: f64,
        unit: impl Into<String>,
    ) -> Result<Self, String> {
        if !quantity.is_finite() || quantity < 0.0 {
            return Err(
                "La quantité de ressource doit être finie et positive."
                    .to_string(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            name: name.into(),
            quantity,
            unit: unit.into(),
            state: ResourceState::Available,
            attributes: serde_json::Map::new(),
            updated_at: Utc::now(),
        })
    }

    pub fn update_quantity(
        &mut self,
        quantity: f64,
    ) -> Result<(), String> {
        if !quantity.is_finite() || quantity < 0.0 {
            return Err(
                "Quantité de ressource invalide.".to_string(),
            );
        }

        self.quantity = quantity;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn set_state(&mut self, state: ResourceState) {
        self.state = state;
        self.updated_at = Utc::now();
    }
}
