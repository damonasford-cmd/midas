use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarthObservation {
    pub id: Uuid,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude_meters: Option<f64>,
    pub description: String,
    pub source: String,
}

#[derive(Debug, Clone, Default)]
pub struct EarthPerception;

impl EarthPerception {
    pub fn new() -> Self {
        Self
    }

    pub fn observe(
        &self,
        description: impl Into<String>,
        source: impl Into<String>,
    ) -> EarthObservation {
        EarthObservation {
            id: Uuid::new_v4(),
            latitude: None,
            longitude: None,
            altitude_meters: None,
            description: description.into(),
            source: source.into(),
        }
    }
}
