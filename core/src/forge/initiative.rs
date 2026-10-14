use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::forge::types::{ForgeObjective, InitiativeOrigin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeInitiative {
    pub id: Uuid,
    pub objective: ForgeObjective,
    pub reason: String,
    pub expected_value: String,
    pub urgency: f64,
    pub confidence: f64,
    pub estimated_cost: Option<Value>,
    pub dependencies: Vec<String>,
    pub risks: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl ForgeInitiative {
    pub fn validate(&self) -> anyhow::Result<()> {
        self.objective.validate()?;

        if self.reason.trim().is_empty() {
            anyhow::bail!("Initiative must explain why it was created");
        }
        if !self.urgency.is_finite() || !(0.0..=1.0).contains(&self.urgency) {
            anyhow::bail!("Urgency must be between 0 and 1");
        }
        if !self.confidence.is_finite() || !(0.0..=1.0).contains(&self.confidence) {
            anyhow::bail!("Confidence must be between 0 and 1");
        }

        Ok(())
    }

    pub fn origin(&self) -> &InitiativeOrigin {
        &self.objective.origin
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InitiativeDecision {
    AcceptForPlanning,
    Defer,
    Reject,
    RequestHumanDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiativeAssessment {
    pub initiative_id: Uuid,
    pub decision: InitiativeDecision,
    pub explanation: String,
    pub assessed_at: DateTime<Utc>,
}

pub fn assess_initiative(
    initiative: &ForgeInitiative,
    available_budget: Option<f64>,
    estimated_cost: Option<f64>,
    require_human_decision_for_spending: bool,
) -> anyhow::Result<InitiativeAssessment> {
    initiative.validate()?;

    let (decision, explanation) = if require_human_decision_for_spending
        && estimated_cost.unwrap_or(0.0) > 0.0
    {
        (
            InitiativeDecision::RequestHumanDecision,
            "The initiative implies spending that requires a separate decision.".to_string(),
        )
    } else if let (Some(budget), Some(cost)) = (available_budget, estimated_cost) {
        if !budget.is_finite() || !cost.is_finite() || budget < 0.0 || cost < 0.0 {
            anyhow::bail!("Budget and estimated cost must be finite and non-negative");
        }

        if cost > budget {
            (
                InitiativeDecision::Defer,
                "Estimated cost exceeds the available budget.".to_string(),
            )
        } else {
            (
                InitiativeDecision::AcceptForPlanning,
                "The initiative can proceed to planning under the supplied budget.".to_string(),
            )
        }
    } else {
        (
            InitiativeDecision::AcceptForPlanning,
            "The initiative can proceed to planning; unknown resource requirements must be investigated.".to_string(),
        )
    };

    Ok(InitiativeAssessment {
        initiative_id: initiative.id,
        decision,
        explanation,
        assessed_at: Utc::now(),
    })
}
