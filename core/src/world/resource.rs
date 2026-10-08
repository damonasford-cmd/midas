
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceKind {
    Money,
    Compute,
    Memory,
    Storage,
    Network,
    Energy,
    Material,
    Equipment,
    Software,
    Data,
    HumanTime,
    Service,
    Infrastructure,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceStatus {
    Available,
    InUse,
    Limited,
    Unavailable,
    Unknown,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
    pub kind: ResourceKind,
    pub status: ResourceStatus,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub owner_entity_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub source_provenance_id: Option<Uuid>,
    pub attributes: serde_json::Value,
    pub observed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Resource {
    pub fn new(
        name: impl Into<String>,
        kind: ResourceKind,
    ) -> Result<Self, String> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err("resource name cannot be empty".into());
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            kind,
            status: ResourceStatus::Unknown,
            quantity: None,
            unit: None,
            owner_entity_id: None,
            location_id: None,
            source_provenance_id: None,
            attributes: serde_json::json!({}),
            observed_at: now,
            updated_at: now,
        })
    }

    pub fn set_quantity(
        &mut self,
        quantity: f64,
        unit: impl Into<String>,
    ) -> Result<(), String> {
        if !quantity.is_finite() || quantity < 0.0 {
            return Err("resource quantity must be finite and non-negative".into());
        }

        let unit = unit.into();

        if unit.trim().is_empty() {
            return Err("resource unit cannot be empty".into());
        }

        self.quantity = Some(quantity);
        self.unit = Some(unit);
        self.observed_at = Utc::now();
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn mark_observed(&mut self, status: ResourceStatus) {
        self.status = status;
        self.observed_at = Utc::now();
        self.updated_at = self.observed_at;
    }
}
