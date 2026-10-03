use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::foundation::contracts::{
    ActionDomain,
    ActionImpact,
    ActionIntent,
    ActionReversibility,
    AuthorizationRequirement,
    Decision,
    Risk,
};

use super::reasoning::ReasoningResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    pub objective: String,
    pub domain: ActionDomain,
    pub impact: ActionImpact,
    pub reversibility: ActionReversibility,
    pub requires_authorization: bool,
}

pub struct DecisionEngine;

impl Default for DecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DecisionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare_action(
        &self,
        context: &DecisionContext,
        reasoning: &ReasoningResult,
        description: impl Into<String>,
        parameters: serde_json::Value,
    ) -> (Decision, ActionIntent) {
        let action_id = Uuid::new_v4();

        let requires_authorization =
            context.requires_authorization
                || matches!(
                    context.impact,
                    ActionImpact::High | ActionImpact::Critical
                );

        let authorization = if requires_authorization {
            AuthorizationRequirement::UserConfirmation
        } else {
            AuthorizationRequirement::SystemPolicy
        };

        let risk = Risk {
            probability: (1.0 - reasoning.confidence).clamp(0.0, 1.0),
            severity: match context.impact {
                ActionImpact::Negligible => 0.05,
                ActionImpact::Low => 0.2,
                ActionImpact::Moderate => 0.5,
                ActionImpact::High => 0.8,
                ActionImpact::Critical => 1.0,
            },
            rationale: "Risque initial dérivé du niveau d'incertitude et de l'impact.".to_string(),
        };

        let action = ActionIntent {
            id: action_id,
            created_at: chrono::Utc::now(),
            domain: context.domain.clone(),
            description: description.into(),
            parameters,
            reversibility: context.reversibility.clone(),
            impact: context.impact.clone(),
            risk,
            authorization,
            idempotency_key: format!("midas-action-{action_id}"),
        };

        let decision = Decision {
            id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            objective: context.objective.clone(),
            selected_action: Some(action_id),
            alternatives: Vec::new(),
            reasoning_summary: reasoning.conclusion.clone(),
            confidence: reasoning.confidence,
            requires_authorization,
        };

        (decision, action)
    }
}
