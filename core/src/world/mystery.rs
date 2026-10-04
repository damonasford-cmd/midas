use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mystery {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub established_facts: Vec<String>,
    pub hypotheses: Vec<String>,
    pub contradictions: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub status: MysteryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MysteryStatus {
    Open,
    Investigating,
    EvidenceFound,
    Explained,
    Disputed,
}

#[derive(Default)]
pub struct MysteryEngine {
    mysteries: Vec<Mystery>,
}

impl MysteryEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Mystery {
        let mystery = Mystery {
            id: Uuid::new_v4(),
            title: title.into(),
            description: description.into(),
            established_facts: Vec::new(),
            hypotheses: Vec::new(),
            contradictions: Vec::new(),
            missing_evidence: Vec::new(),
            status: MysteryStatus::Open,
        };

        self.mysteries.push(mystery.clone());

        mystery
    }

    pub fn all(&self) -> &[Mystery] {
        &self.mysteries
    }
}
