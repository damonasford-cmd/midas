use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::foundation::contracts::Observation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Understanding {
    pub id: Uuid,
    pub observation_id: Uuid,
    pub interpretation: String,
    pub entities: Vec<String>,
    pub relations: Vec<Relation>,
    pub unknowns: Vec<String>,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f64,
}

pub struct UnderstandingEngine;

impl Default for UnderstandingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl UnderstandingEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        observation: &Observation,
        interpretation: impl Into<String>,
        entities: Vec<String>,
        relations: Vec<Relation>,
        unknowns: Vec<String>,
        confidence: f64,
    ) -> Result<Understanding> {
        Ok(Understanding {
            id: Uuid::new_v4(),
            observation_id: observation.id,
            interpretation: interpretation.into(),
            entities,
            relations,
            unknowns,
            confidence: confidence.clamp(0.0, 1.0),
            evidence: observation.provenance.clone(),
            metadata: Value::Object(Default::default()),
        })
    }
}
