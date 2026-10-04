use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unknown {
    pub id: Uuid,
    pub question: String,
    pub domain: String,
    pub importance: f64,
    pub discovered_at: DateTime<Utc>,
    pub status: UnknownStatus,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnknownStatus {
    Open,
    Investigating,
    PartiallyResolved,
    Resolved,
    UnresolvableWithCurrentEvidence,
}

#[derive(Default)]
pub struct UnknownEngine {
    unknowns: Vec<Unknown>,
}

impl UnknownEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        question: impl Into<String>,
        domain: impl Into<String>,
        importance: f64,
    ) -> Unknown {
        let unknown = Unknown {
            id: Uuid::new_v4(),
            question: question.into(),
            domain: domain.into(),
            importance: importance.clamp(0.0, 1.0),
            discovered_at: Utc::now(),
            status: UnknownStatus::Open,
            evidence: Vec::new(),
        };

        self.unknowns.push(unknown.clone());

        unknown
    }

    pub fn all(&self) -> &[Unknown] {
        &self.unknowns
    }
}
