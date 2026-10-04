use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: Uuid,
    pub name: String,
    pub hypothesis: String,
    pub variables: Vec<String>,
    pub controls: Vec<String>,
    pub expected_result: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub experiment_id: Uuid,
    pub observed_result: String,
    pub measurements: Vec<f64>,
    pub succeeded: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ExperimentationEngine;

impl ExperimentationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        name: impl Into<String>,
        hypothesis: impl Into<String>,
        expected_result: impl Into<String>,
    ) -> Experiment {
        Experiment {
            id: Uuid::new_v4(),
            name: name.into(),
            hypothesis: hypothesis.into(),
            variables: Vec::new(),
            controls: Vec::new(),
            expected_result: expected_result.into(),
            created_at: Utc::now(),
        }
    }

    pub fn record(
        &self,
        experiment: &Experiment,
        observed_result: impl Into<String>,
        measurements: Vec<f64>,
        succeeded: bool,
    ) -> ExperimentResult {
        ExperimentResult {
            experiment_id: experiment.id,
            observed_result: observed_result.into(),
            measurements,
            succeeded,
            notes: Vec::new(),
        }
    }
}
