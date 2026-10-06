use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::source::KnowledgeSource;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ProvenanceKind {
    DirectObservation,
    UserProvided,
    Retrieved,
    Derived,
    Inferred,
    Imported,
    ModelGenerated,
    Historical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeProvenance {
    pub kind: ProvenanceKind,

    pub source: Option<KnowledgeSource>,

    pub memory_id: Option<Uuid>,

    pub event_id: Option<Uuid>,

    pub acquired_at: DateTime<Utc>,

    pub transformation_chain: Vec<String>,
}

impl Default for KnowledgeProvenance {
    fn default() -> Self {
        Self {
            kind: ProvenanceKind::Unknown,
            source: None,
            memory_id: None,
            event_id: None,
            acquired_at: Utc::now(),
            transformation_chain: Vec::new(),
        }
    }
}

impl KnowledgeProvenance {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(source) = &self.source {
            source.validate()?;
        }

        Ok(())
    }
}
