use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::decision::Decision;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    pub id: Uuid,

    pub connector: String,

    pub operation: String,

    pub arguments: Vec<String>,

    pub constraints: Vec<String>,

    pub profile: ActionRiskProfile,
}

impl ActionIntent {
    pub fn new(
        connector: impl Into<String>,
        operation: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            connector: connector.into(),
            operation: operation.into(),
            arguments: Vec::new(),
            constraints: Vec::new(),
            profile: ActionRiskProfile::default(),
        }
    }

    pub fn with_arguments(
        mut self,
        arguments: Vec<String>,
    ) -> Self {
        self.arguments = arguments;
        self
    }

    pub fn with_constraints(
        mut self,
        constraints: Vec<String>,
    ) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn with_profile(
        mut self,
        profile: ActionRiskProfile,
    ) -> Self {
        self.profile = profile;
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.connector.trim().is_empty()
            && !self.operation.trim().is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub id: Uuid,

    pub timestamp: DateTime<Utc>,

    pub action: String,

    pub description: String,

    pub message: String,

    pub success: bool,

    pub executed: bool,

    pub authorization: ActionAuthorization,

    pub approval: AuthorizationDecision,

    pub connector: Option<String>,

    pub intent: Option<ActionIntent>,

    pub profile: ActionRiskProfile,
}

impl ActionResult {
    pub fn denied(
        action: impl Into<String>,
        description: impl Into<String>,
        message: impl Into<String>,
        authorization: ActionAuthorization,
        profile: ActionRiskProfile,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            description: description.into(),
            message: message.into(),
            success: false,
            executed: false,
            authorization,
            approval: AuthorizationDecision::Denied,
            connector: None,
            intent: None,
            profile,
        }
    }

    pub fn requires_approval(
        action: impl Into<String>,
        description: impl Into<String>,
        message: impl Into<String>,
        authorization: ActionAuthorization,
        profile: ActionRiskProfile,
        intent: Option<ActionIntent>,
    ) -> Self {
        let connector = intent
            .as_ref()
            .map(|value| value.connector.clone());

        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            description: description.into(),
            message: message.into(),
            success: false,
            executed: false,
            authorization,
            approval: AuthorizationDecision::Required,
            connector,
            intent,
            profile,
        }
    }

    pub fn prepared(
        action: impl Into<String>,
        description: impl Into<String>,
        message: impl Into<String>,
        authorization: ActionAuthorization,
        profile: ActionRiskProfile,
        intent: Option<ActionIntent>,
    ) -> Self {
        let connector = intent
            .as_ref()
            .map(|value| value.connector.clone());

        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            description: description.into(),
            message: message.into(),
            success: true,
            executed: false,
            authorization,
            approval: AuthorizationDecision::NotRequired,
            connector,
            intent,
            profile,
        }
    }

    pub fn executed(
        action: impl Into<String>,
        description: impl Into<String>,
        message: impl Into<String>,
        profile: ActionRiskProfile,
        intent: ActionIntent,
    ) -> Self {
        let connector = Some(intent.connector.clone());

        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            description: description.into(),
            message: message.into(),
            success: true,
            executed: true,
            authorization: ActionAuthorization::Authorized,
            approval: AuthorizationDecision::Approved,
            connector,
            intent: Some(intent),
            profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionAuthorization {
    Authorized,
    Unauthorized,
    RequiresApproval,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationDecision {
    NotRequired,
    Approved,
    Required,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRiskProfile {
    pub risk_level: u8,

    pub reversible: bool,

    pub financial_impact: bool,

    pub external_side_effect: bool,

    pub requires_human_approval: bool,

    pub estimated_value_eur: Option<f64>,
}

impl Default for ActionRiskProfile {
    fn default() -> Self {
        Self {
            risk_level: 0,
            reversible: true,
            financial_impact: false,
            external_side_effect: false,
            requires_human_approval: false,
            estimated_value_eur: None,
        }
    }
}

impl ActionRiskProfile {
    pub fn low() -> Self {
        Self {
            risk_level: 1,
            ..Self::default()
        }
    }

    pub fn medium() -> Self {
        Self {
            risk_level: 2,
            external_side_effect: true,
            ..Self::default()
        }
    }

    pub fn high() -> Self {
        Self {
            risk_level: 3,
            reversible: false,
            external_side_effect: true,
            requires_human_approval: true,
            ..Self::default()
        }
    }

    pub fn critical() -> Self {
        Self {
            risk_level: 4,
            reversible: false,
            external_side_effect: true,
            financial_impact: true,
            requires_human_approval: true,
            ..Self::default()
        }
    }

    pub fn requires_approval(&self) -> bool {
        self.requires_human_approval
            || self.risk_level >= 4
            || self.estimated_value_eur
                .map(|value| value >= 500_000.0)
                .unwrap_or(false)
    }
}

pub struct ActionEngine;

impl ActionEngine {
    pub fn prepare(
        decision: &Decision,
        intent: Option<ActionIntent>,
    ) -> Result<ActionResult> {
        let profile = intent
            .as_ref()
            .map(|value| value.profile.clone())
            .unwrap_or_default();

        if let Some(ref action_intent) = intent {
            if !action_intent.is_valid() {
                return Ok(ActionResult::denied(
                    decision.action.clone(),
                    decision.rationale.clone(),
                    "ActionIntent invalide : connecteur ou opération manquant.",
                    ActionAuthorization::Unauthorized,
                    profile,
                ));
            }
        }

        if !decision.executable {
            return Ok(ActionResult::denied(
                decision.action.clone(),
                decision.rationale.clone(),
                "La décision n'est pas exécutable.",
                ActionAuthorization::Unauthorized,
                profile,
            ));
        }

        if decision.requires_approval
            || profile.requires_approval()
        {
            return Ok(ActionResult::requires_approval(
                decision.action.clone(),
                decision.rationale.clone(),
                "Cette action nécessite une approbation.",
                ActionAuthorization::RequiresApproval,
                profile,
                intent,
            ));
        }

        Ok(ActionResult::prepared(
            decision.action.clone(),
            decision.rationale.clone(),
            "Action préparée et autorisée.",
            ActionAuthorization::Authorized,
            profile,
            intent,
        ))
    }
}
