use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEntity {
    pub id: String,
    pub entity_type: String,
    pub attributes: HashMap<String, Value>,
    pub confidence: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldModel {
    pub entities: HashMap<String, WorldEntity>,
    pub relations: Vec<WorldRelation>,
    pub last_update: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldRelation {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f64,
    pub source: String,
}

impl WorldModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn upsert_entity(
        &mut self,
        id: impl Into<String>,
        entity_type: impl Into<String>,
        attributes: HashMap<String, Value>,
        confidence: f64,
    ) {
        let id = id.into();

        self.entities.insert(
            id.clone(),
            WorldEntity {
                id,
                entity_type: entity_type.into(),
                attributes,
                confidence: confidence.clamp(0.0, 1.0),
                updated_at: Utc::now(),
            },
        );

        self.last_update = Some(Utc::now());
    }

    pub fn add_relation(
        &mut self,
        relation: WorldRelation,
    ) {
        self.relations.push(relation);
        self.last_update = Some(Utc::now());
    }

    pub fn relation_exists(
        &self,
        subject: &str,
        predicate: &str,
        object: &str,
    ) -> bool {
        self.relations.iter().any(|relation| {
            relation.subject == subject
                && relation.predicate == predicate
                && relation.object == object
        })
    }

    pub fn snapshot(&self) -> Result<Value> {
        Ok(serde_json::to_value(self)?)
    }
}
