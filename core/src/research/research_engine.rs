use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchQuestion {
    pub id: Uuid,
    pub question: String,
    pub domain: String,
    pub importance: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub id: Uuid,
    pub question_id: Uuid,
    pub conclusion: String,
    pub evidence: Vec<String>,
    pub uncertainties: Vec<String>,
    pub confidence: f64,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct ResearchEngine;

impl ResearchEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn question(
        &self,
        question: impl Into<String>,
        domain: impl Into<String>,
        importance: f64,
    ) -> ResearchQuestion {
        ResearchQuestion {
            id: Uuid::new_v4(),
            question: question.into(),
            domain: domain.into(),
            importance: importance.clamp(0.0, 1.0),
            created_at: Utc::now(),
        }
    }

    pub fn result(
        &self,
        question: &ResearchQuestion,
        conclusion: impl Into<String>,
        evidence: Vec<String>,
        uncertainties: Vec<String>,
        confidence: f64,
    ) -> ResearchResult {
        ResearchResult {
            id: Uuid::new_v4(),
            question_id: question.id,
            conclusion: conclusion.into(),
            evidence,
            uncertainties,
            confidence: confidence.clamp(0.0, 1.0),
            completed_at: Utc::now(),
        }
    }
}
