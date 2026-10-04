use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanObservation {
    pub id: Uuid,
    pub region: String,
    pub variables: Vec<String>,
    pub measurements: Vec<f64>,
    pub source: String,
}

#[derive(Debug, Clone, Default)]
pub struct OceanPerception;

impl OceanPerception {
    pub fn new() -> Self {
        Self
    }

    pub fn observe(
        &self,
        region: impl Into<String>,
        source: impl Into<String>,
    ) -> OceanObservation {
        OceanObservation {
            id: Uuid::new_v4(),
            region: region.into(),
            variables: Vec::new(),
            measurements: Vec::new(),
            source: source.into(),
        }
    }
}
