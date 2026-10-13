
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::research::evidence::{
    Evidence, EvidencePolarity, VerificationLevel,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HypothesisStatus {
    Proposed,
    UnderInvestigation,
    Supported,
    Refuted,
    Inconclusive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    pub id: Uuid,
    pub objective_id: Uuid,
    pub statement: String,
    pub rationale: String,
    pub status: HypothesisStatus,
    pub supporting_evidence_ids: Vec<Uuid>,
    pub refuting_evidence_ids: Vec<Uuid>,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Hypothesis {
    pub fn new(
        objective_id: Uuid,
        statement: impl Into<String>,
        rationale: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let statement = statement.into().trim().to_owned();
        anyhow::ensure!(!statement.is_empty(), "Hypothèse vide");

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            objective_id,
            statement,
            rationale: rationale.into(),
            status: HypothesisStatus::Proposed,
            supporting_evidence_ids: Vec::new(),
            refuting_evidence_ids: Vec::new(),
            confidence: 0.0,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn evaluate(&mut self, evidence: &[Evidence]) {
        let relevant: Vec<_> = evidence.iter()
            .filter(|item| {
                item.statement.to_lowercase().contains(&self.statement.to_lowercase())
                    || self.statement.to_lowercase().contains(&item.statement.to_lowercase())
            })
            .collect();

        self.supporting_evidence_ids.clear();
        self.refuting_evidence_ids.clear();

        let mut support = 0.0_f64;
        let mut refute = 0.0_f64;

        for item in relevant {
            let verification_factor = match item.verification {
                VerificationLevel::Unchecked => 0.5,
                VerificationLevel::SingleSource => 0.7,
                VerificationLevel::IndependentlyCorroborated => 0.9,
                VerificationLevel::ExperimentallyObserved => 0.95,
                VerificationLevel::FormallyVerified => 1.0,
                VerificationLevel::Disputed => 0.25,
            };

            let weight = item.reliability * verification_factor;

            match item.polarity {
                EvidencePolarity::Supports => {
                    support += weight;
                    self.supporting_evidence_ids.push(item.id);
                }
                EvidencePolarity::Refutes => {
                    refute += weight;
                    self.refuting_evidence_ids.push(item.id);
                }
                EvidencePolarity::Neutral => {}
            }
        }

        let total = support + refute;
        self.confidence = if total > 0.0 {
            (support / total).clamp(0.0, 1.0)
        } else {
            0.0
        };

        self.status = if support > 0.0 && refute > 0.0 && (support - refute).abs() < total * 0.25 {
            HypothesisStatus::Inconclusive
        } else if refute > support && refute > 0.0 {
            HypothesisStatus::Refuted
        } else if support > refute && support > 0.0 {
            HypothesisStatus::Supported
        } else {
            HypothesisStatus::Inconclusive
        };

        self.updated_at = Utc::now();
    }
}
