use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub id: Uuid,
    pub name: String,
    pub initial_state: Value,
    pub parameters: Value,
    pub duration: f64,
    pub resolution: f64,
    pub result: Option<Value>,
}

pub struct SimulationEngine;

impl Default for SimulationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        name: impl Into<String>,
        initial_state: Value,
        parameters: Value,
        duration: f64,
        resolution: f64,
    ) -> Simulation {
        Simulation {
            id: Uuid::new_v4(),
            name: name.into(),
            initial_state,
            parameters,
            duration,
            resolution,
            result: None,
        }
    }

    pub fn record_result(
        &self,
        simulation: &mut Simulation,
        result: Value,
    ) -> Result<()> {
        simulation.result = Some(result);
        Ok(())
    }
}
