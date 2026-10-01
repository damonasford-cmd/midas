use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::decision::Decision;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionImpact {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionReversibility {
    Reversible,
    PartiallyReversible,
    Irreversible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionAuthorization {
    NoneRequired,
    Authorized,
    RequiresApproval,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionDomain {
    Internal,
    Information,
    Software,
    Financial,
    Communication,
    Infrastructure,
    Physical,
    Public,
    Security,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRiskProfile {
    pub impact: ActionImpact,
    pub reversibility: ActionReversibility,
    pub authorization: ActionAuthorization,
    pub domain: ActionDomain,
    pub estimated_cost: Option<f64>,
    pub affects_external_system: bool,
    pub affects_third_party: bool,
    pub requires_confirmation: bool,
}

impl Default for ActionRiskProfile {
    fn default() -> Self {
        Self {
            impact: ActionImpact::Low,
            reversibility:
                ActionReversibility::Reversible,
            authorization:
                ActionAuthorization::NoneRequired,
            domain: ActionDomain::Internal,
            estimated_cost: None,
            affects_external_system: false,
            affects_third_party: false,
            requires_confirmation: false,
        }
    }
}

impl ActionRiskProfile {
    pub fn internal() -> Self {
        Self::default()
    }

    pub fn requires_approval(&self) -> bool {
        self.authorization
            == ActionAuthorization::RequiresApproval
            || self.requires_confirmation
    }

    pub fn is_blocked(&self) -> bool {
        self.authorization
            == ActionAuthorization::Blocked
    }

    pub fn is_external(&self) -> bool {
        self.affects_external_system
    }

    pub fn is_high_impact(&self) -> bool {
        matches!(
            self.impact,
            ActionImpact::High
                | ActionImpact::Critical
        )
    }

    pub fn is_irreversible(&self) -> bool {
        self.reversibility
            == ActionReversibility::Irreversible
    }

    pub fn is_critical(&self) -> bool {
        self.impact
            == ActionImpact::Critical
            || self.is_irreversible()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRequest {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub reason: String,
    pub profile: ActionRiskProfile,
}

impl ActionRequest {
    pub fn new(
        action: impl Into<String>,
        reason: impl Into<String>,
        profile: ActionRiskProfile,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            reason: reason.into(),
            profile,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorizationDecision {
    Allowed,
    RequiresApproval,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub decision: AuthorizationDecision,
    pub reason: String,
}

impl AuthorizationResult {
    pub fn allowed(
        reason: impl Into<String>,
    ) -> Self {
        Self {
            decision:
                AuthorizationDecision::Allowed,
            reason: reason.into(),
        }
    }

    pub fn requires_approval(
        reason: impl Into<String>,
    ) -> Self {
        Self {
            decision:
                AuthorizationDecision::RequiresApproval,
            reason: reason.into(),
        }
    }

    pub fn denied(
        reason: impl Into<String>,
    ) -> Self {
        Self {
            decision:
                AuthorizationDecision::Denied,
            reason: reason.into(),
        }
    }

    pub fn is_allowed(&self) -> bool {
        self.decision
            == AuthorizationDecision::Allowed
    }

    pub fn needs_approval(&self) -> bool {
        self.decision
            == AuthorizationDecision::RequiresApproval
    }

    pub fn is_denied(&self) -> bool {
        self.decision
            == AuthorizationDecision::Denied
    }
}

pub struct ActionAuthorizationGate;

impl ActionAuthorizationGate {
    pub fn evaluate(
        profile: &ActionRiskProfile,
    ) -> AuthorizationResult {
        if profile.is_blocked() {
            return AuthorizationResult::denied(
                "Action bloquée par la politique d'autorisation.",
            );
        }

        if profile.requires_approval() {
            return AuthorizationResult::requires_approval(
                "Validation requise avant exécution.",
            );
        }

        if profile.is_irreversible()
            && profile.is_high_impact()
        {
            return AuthorizationResult::requires_approval(
                "Action à fort impact et irréversible : validation requise.",
            );
        }

        if profile.affects_third_party
            && profile.is_high_impact()
        {
            return AuthorizationResult::requires_approval(
                "Action à fort impact affectant un tiers : validation requise.",
            );
        }

        AuthorizationResult::allowed(
            "Action autorisée par le contrôle de risque.",
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Refused,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub profile: ActionRiskProfile,
    pub reason: String,
    pub status: ApprovalStatus,
}

impl ApprovalRequest {
    pub fn new(
        action: impl Into<String>,
        profile: ActionRiskProfile,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            profile,
            reason: reason.into(),
            status: ApprovalStatus::Pending,
        }
    }

    pub fn approve(&mut self) {
        if self.status
            == ApprovalStatus::Pending
        {
            self.status =
                ApprovalStatus::Approved;
        }
    }

    pub fn refuse(&mut self) {
        if self.status
            == ApprovalStatus::Pending
        {
            self.status =
                ApprovalStatus::Refused;
        }
    }

    pub fn cancel(&mut self) {
        if self.status
            == ApprovalStatus::Pending
        {
            self.status =
                ApprovalStatus::Cancelled;
        }
    }

    pub fn expire(&mut self) {
        if self.status
            == ApprovalStatus::Pending
        {
            self.status =
                ApprovalStatus::Expired;
        }
    }

    pub fn is_pending(&self) -> bool {
        self.status
            == ApprovalStatus::Pending
    }

    pub fn is_approved(&self) -> bool {
        self.status
            == ApprovalStatus::Approved
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

    /*
     * Le profil reste attaché au résultat de l'action.
     *
     * Il accompagne donc :
     *
     * Decision
     *     ↓
     * ActionResult
     *     ↓
     * ExecutionRequest
     *     ↓
     * ConnectorRequest
     */
    pub profile: ActionRiskProfile,

    pub authorization:
        AuthorizationResult,

    pub approval:
        Option<ApprovalRequest>,

    pub connector:
        Option<String>,
}

impl ActionResult {
    pub fn denied(
        action: impl Into<String>,
        description: impl Into<String>,
        profile: ActionRiskProfile,
        authorization: AuthorizationResult,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            description: description.into(),
            message:
                authorization.reason.clone(),
            success: false,
            executed: false,
            profile,
            authorization,
            approval: None,
            connector: None,
        }
    }

    pub fn requires_approval(
        action: impl Into<String>,
        description: impl Into<String>,
        profile: ActionRiskProfile,
        authorization: AuthorizationResult,
        approval: ApprovalRequest,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            description: description.into(),
            message:
                "Action en attente de validation."
                    .to_string(),
            success: false,
            executed: false,
            profile,
            authorization,
            approval: Some(approval),
            connector: None,
        }
    }

    pub fn prepared(
        action: impl Into<String>,
        description: impl Into<String>,
        profile: ActionRiskProfile,
        authorization: AuthorizationResult,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action: action.into(),
            description: description.into(),
            message:
                "Action autorisée et prête pour un connecteur réel."
                    .to_string(),
            success: true,
            executed: false,
            profile,
            authorization,
            approval: None,
            connector: None,
        }
    }

    pub fn with_connector(
        mut self,
        connector: impl Into<String>,
    ) -> Self {
        self.connector =
            Some(connector.into());

        self
    }

    pub fn is_authorized(&self) -> bool {
        self.authorization.is_allowed()
    }

    pub fn needs_approval(&self) -> bool {
        self.authorization
            .needs_approval()
            || self.approval.is_some()
    }

    pub fn is_denied(&self) -> bool {
        self.authorization.is_denied()
    }
}

pub struct ActionEngine;

impl ActionEngine {
    pub fn prepare(
        decision: &Decision,
    ) -> ActionResult {
        let profile =
            Self::profile_from_decision(
                decision,
            );

        let authorization =
            ActionAuthorizationGate::evaluate(
                &profile,
            );

        match authorization.decision {
            AuthorizationDecision::Denied => {
                ActionResult::denied(
                    decision.action.clone(),
                    decision.rationale.clone(),
                    profile,
                    authorization,
                )
            }

            AuthorizationDecision::RequiresApproval => {
                let approval =
                    ApprovalRequest::new(
                        decision.action.clone(),
                        profile.clone(),
                        decision.rationale.clone(),
                    );

                ActionResult::requires_approval(
                    decision.action.clone(),
                    decision.rationale.clone(),
                    profile,
                    authorization,
                    approval,
                )
            }

            AuthorizationDecision::Allowed => {
                ActionResult::prepared(
                    decision.action.clone(),
                    decision.rationale.clone(),
                    profile,
                    authorization,
                )
            }
        }
    }

    pub fn execute(
        decision: &Decision,
    ) -> ActionResult {
        let mut result =
            Self::prepare(decision);

        if result.approval.is_some() {
            result.message =
                "Action suspendue : validation requise avant exécution."
                    .to_string();

            result.success = false;
            result.executed = false;

            return result;
        }

        if result.is_denied() {
            result.success = false;
            result.executed = false;

            return result;
        }

        if !decision.executable {
            result.success = false;
            result.executed = false;
            result.message =
                "La décision n'est pas exécutable dans son état actuel."
                    .to_string();

            return result;
        }

        /*
         * Le Core ne simule pas une exécution réelle ici.
         *
         * L'ExecutionEngine prendra ensuite le relais :
         *
         * ActionResult
         *      ↓
         * ExecutionRequest
         *      ↓
         * ConnectorRegistry
         *      ↓
         * Connector
         *      ↓
         * Résultat réel
         */

        result.message =
            "Action autorisée et prête pour l'ExecutionEngine."
                .to_string();

        result.success = true;
        result.executed = false;

        result
    }

    fn profile_from_decision(
        decision: &Decision,
    ) -> ActionRiskProfile {
        let authorization =
            if decision.requires_approval {
                ActionAuthorization::
                    RequiresApproval
            } else {
                ActionAuthorization::
                    Authorized
            };

        let domain =
            decision.domain;

        ActionRiskProfile {
            impact:
                decision.impact,

            reversibility:
                decision.reversibility,

            authorization,

            domain,

            estimated_cost:
                None,

            affects_external_system:
                !matches!(
                    domain,
                    ActionDomain::Internal
                        | ActionDomain::Information
                ),

            affects_third_party:
                matches!(
                    domain,
                    ActionDomain::Communication
                        | ActionDomain::Financial
                        | ActionDomain::Public
                        | ActionDomain::Physical
                ),

            requires_confirmation:
                decision.requires_approval,
        }
    }
}
