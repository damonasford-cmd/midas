
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationKind {
    Address,
    Building,
    Facility,
    City,
    Region,
    Country,
    GeographicArea,
    Online,
    NetworkEndpoint,
    Orbit,
    Ocean,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Uuid,
    pub name: String,
    pub kind: LocationKind,
    pub parent_id: Option<Uuid>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude_meters: Option<f64>,
    pub address: Option<String>,
    pub external_reference: Option<String>,
    pub attributes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Location {
    pub fn new(
        name: impl Into<String>,
        kind: LocationKind,
    ) -> Result<Self, String> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err("location name cannot be empty".into());
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            kind,
            parent_id: None,
            latitude: None,
            longitude: None,
            altitude_meters: None,
            address: None,
            external_reference: None,
            attributes: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn set_coordinates(
        &mut self,
        latitude: f64,
        longitude: f64,
    ) -> Result<(), String> {
        if !latitude.is_finite()
            || !longitude.is_finite()
            || !(-90.0..=90.0).contains(&latitude)
            || !(-180.0..=180.0).contains(&longitude)
        {
            return Err("invalid geographic coordinates".into());
        }

        self.latitude = Some(latitude);
        self.longitude = Some(longitude);
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn set_parent(
        &mut self,
        parent_id: Uuid,
    ) -> Result<(), String> {
        if parent_id == self.id {
            return Err("a location cannot be its own parent".into());
        }

        self.parent_id = Some(parent_id);
        self.updated_at = Utc::now();

        Ok(())
    }
}
