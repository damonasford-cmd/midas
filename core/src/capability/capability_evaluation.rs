use anyhow::{bail, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCheck {
    pub name: String,
    pub passed: bool,
    pub details: String,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityEvaluation {
    pub id: Uuid,
    pub capability_id: Uuid,
    pub successful: bool,
    pub score: f64,
    pub checks: Vec<EvaluationCheck>,
    pub evidence: Value,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}

impl CapabilityEvaluation {
    pub fn new(
        capability_id: Uuid,
        checks: Vec<EvaluationCheck>,
        score: f64,
        evidence: Value,
        validity: Duration,
    ) -> Result<Self> {
        if !score.is_finite() || !(0.0..=1.0).contains(&score) {
            bail!("Evaluation score must be finite and between 0 and 1");
        }

        if checks.is_empty() {
            bail!("An evaluation must contain at least one check");
        }

        if !evidence.is_object() {
            bail!("Evaluation evidence must be a JSON object");
        }

        if validity <= Duration::zero() {
            bail!("Evaluation validity must be positive");
        }

        let completed_at = Utc::now();

        if checks.iter().any(|check| {
            check.name.trim().is_empty()
                || check.name.len() > 160
                || check.details.len() > 16_384
                || check.checked_at > completed_at
        }) {
            bail!("Evaluation contains an invalid check");
        }

        let successful = score >= 0.80 && checks.iter().all(|c| c.passed);

        Ok(Self {
            id: Uuid::new_v4(),
            capability_id,
            successful,
            score,
            checks,
            evidence,
            started_at: completed_at,
            completed_at,
            valid_until: completed_at + validity,
        })
    }

    pub fn is_current(&self, now: DateTime<Utc>) -> bool {
        self.successful
            && self.completed_at <= now
            && self.valid_until > now
    }
}
