use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicIndicator {
    pub id: Uuid,
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
    pub source: String,
}

#[derive(Debug, Clone, Default)]
pub struct EconomicEngine;

impl EconomicEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn indicator(
        &self,
        name: impl Into<String>,
        value: f64,
        unit: impl Into<String>,
        timestamp: impl Into<String>,
        source: impl Into<String>,
    ) -> EconomicIndicator {
        EconomicIndicator {
            id: Uuid::new_v4(),
            name: name.into(),
            value,
            unit: unit.into(),
            timestamp: timestamp.into(),
            source: source.into(),
        }
    }
}
