use crate::decision::Decision;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionImpact {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionReversibility {
    Reversible,
    PartiallyReversible,
    Irreversible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionAuthorization {
    NoneRequired,
    Authorized,
    RequiresApproval,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
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
            reversibility: ActionReversibility::Reversible,
            authorization: ActionAuthorization::NoneRequired,
            domain: ActionDomain::Internal,
            estimated_cost: None,
            affects_external_system: false,
            affects_third_party: false,
            requires_confirmation: false,
        }
    }
}

impl ActionRiskProfile {
    pub fn low() -> Self {
        Self::default()
    }

    pub fn medium() -> Self {
        Self {
            impact: ActionImpact::Medium,
            ..Self::default()
        }
    }

    pub fn high() -> Self {
        Self {
            impact: ActionImpact::High,
            reversibility:
                ActionReversibility::PartiallyReversible,
            affects_external_system: true,
            ..Self::default()
        }
    }

    pub fn critical() -> Self {
        Self {
            impact: ActionImpact::Critical,
            reversibility:
                ActionReversibility::Irreversible,
            authorization:
                ActionAuthorization::RequiresApproval,
            requires_confirmation: true,
            ..Self::default()
        }
    }

    pub fn requires_verification(&self) -> bool {
        matches!(
            self.impact,
            ActionImpact::High | ActionImpact::Critical
        ) || matches!(
            self.reversibility,
            ActionReversibility::Irreversible
                | ActionReversibility::PartiallyReversible
        ) || matches!(
            self.authorization,
            ActionAuthorization::RequiresApproval
                | ActionAuthorization::Blocked
        ) || self.requires_confirmation
    }

    pub fn requires_authorization(&self) -> bool {
        matches!(
            self.authorization,
            ActionAuthorization::RequiresApproval
                | ActionAuthorization::Blocked
        )
    }

    pub fn is_critical(&self) -> bool {
        self.impact == ActionImpact::Critical
            || self.reversibility
                == ActionReversibility::Irreversible
            || self.authorization
                == ActionAuthorization::RequiresApproval
    }

    pub fn summary(&self) -> String {
        format!(
            "impact={:?}, reversibilité={:?}, \
             autorisation={:?}, domaine={:?}, \
             coût={:?}, système_externe={}, \
             tiers={}, confirmation={}",
            self.impact,
            self.reversibility,
            self.authorization,
            self.domain,
            self.estimated_cost,
            self.affects_external_system,
            self.affects_third_party,
            self.requires_confirmation,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ActionRequest {
    pub description: String,
    pub profile: ActionRiskProfile,
}

impl ActionRequest {
    pub fn new(
        description: impl Into<String>,
        profile: ActionRiskProfile,
    ) -> Self {
        Self {
            description: description.into(),
            profile,
        }
    }

    pub fn internal(
        description: impl Into<String>,
    ) -> Self {
        Self::new(
            description,
            ActionRiskProfile::low(),
        )
    }

    pub fn external(
        description: impl Into<String>,
        impact: ActionImpact,
    ) -> Self {
        let mut profile =
            ActionRiskProfile::default();

        profile.impact = impact;
        profile.affects_external_system = true;
        profile.domain =
            ActionDomain::Infrastructure;

        Self::new(
            description,
            profile,
        )
    }

    pub fn financial(
        description: impl Into<String>,
        impact: ActionImpact,
        cost: Option<f64>,
    ) -> Self {
        let mut profile =
            ActionRiskProfile::default();

        profile.impact = impact;
        profile.domain = ActionDomain::Financial;
        profile.estimated_cost = cost;
        profile.affects_external_system = true;

        if matches!(
            impact,
            ActionImpact::High | ActionImpact::Critical
        ) {
            profile.requires_confirmation = true;
        }

        Self::new(
            description,
            profile,
        )
    }

    pub fn physical(
        description: impl Into<String>,
        impact: ActionImpact,
        reversible: ActionReversibility,
    ) -> Self {
        let mut profile =
            ActionRiskProfile::default();

        profile.impact = impact;
        profile.reversibility = reversible;
        profile.domain = ActionDomain::Physical;
        profile.affects_external_system = true;

        if impact == ActionImpact::Critical
            || reversible
                == ActionReversibility::Irreversible
        {
            profile.requires_confirmation = true;
        }

        Self::new(
            description,
            profile,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationDecision {
    Allowed,
    RequiresApproval,
    Denied,
}

#[derive(Debug, Clone)]
pub struct AuthorizationResult {
    pub decision: AuthorizationDecision,
    pub reason: String,
}

pub struct ActionAuthorizationGate;

impl ActionAuthorizationGate {
    pub fn evaluate(
        profile: &ActionRiskProfile,
    ) -> AuthorizationResult {
        match profile.authorization {
            ActionAuthorization::Blocked => {
                AuthorizationResult {
                    decision:
                        AuthorizationDecision::Denied,
                    reason:
                        "Action bloquée par la politique d'autorisation."
                            .to_string(),
                }
            }

            ActionAuthorization::RequiresApproval => {
                AuthorizationResult {
                    decision:
                        AuthorizationDecision::RequiresApproval,
                    reason:
                        "Validation explicite requise avant exécution."
                            .to_string(),
                }
            }

            ActionAuthorization::Authorized
            | ActionAuthorization::NoneRequired => {
                if profile.requires_confirmation {
                    AuthorizationResult {
                        decision:
                            AuthorizationDecision::RequiresApproval,
                        reason:
                            "Confirmation requise avant exécution."
                                .to_string(),
                    }
                } else {
                    AuthorizationResult {
                        decision:
                            AuthorizationDecision::Allowed,
                        reason:
                            "Action autorisée."
                                .to_string(),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Refused,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub id: Uuid,
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
            action: action.into(),
            profile,
            reason: reason.into(),
            status: ApprovalStatus::Pending,
        }
    }

    pub fn approve(&mut self) {
        self.status = ApprovalStatus::Approved;
    }

    pub fn refuse(&mut self) {
        self.status = ApprovalStatus::Refused;
    }

    pub fn cancel(&mut self) {
        self.status = ApprovalStatus::Cancelled;
    }

    pub fn expire(&mut self) {
        self.status = ApprovalStatus::Expired;
    }

    pub fn is_pending(&self) -> bool {
        self.status == ApprovalStatus::Pending
    }

    pub fn is_approved(&self) -> bool {
        self.status == ApprovalStatus::Approved
    }

    pub fn summary(&self) -> String {
        format!(
            "Validation {} : action={}, statut={:?}, \
             impact={:?}, coût={:?}, raison={}",
            self.id,
            self.action,
            self.status,
            self.profile.impact,
            self.profile.estimated_cost,
            self.reason,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub description: String,
    pub success: bool,
    pub executed: bool,
    pub profile: ActionRiskProfile,
    pub authorization: AuthorizationResult,
    pub approval: Option<ApprovalRequest>,
    pub message: String,
}

pub struct ActionEngine;

impl ActionEngine {
    pub fn prepare(
        request: ActionRequest,
    ) -> ActionRequest {
        request
    }

    pub fn execute(
        decision: &Decision,
    ) -> ActionResult {
        let description =
            decision.action.clone();

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
                ActionResult {
                    description,
                    success: false,
                    executed: false,
                    profile,
                    authorization,
                    approval: None,
                    message:
                        "Action refusée : autorisation bloquée."
                            .to_string(),
                }
            }

            AuthorizationDecision::RequiresApproval => {
                let approval =
                    ApprovalRequest::new(
                        description.clone(),
                        profile.clone(),
                        authorization.reason.clone(),
                    );

                ActionResult {
                    description,
                    success: false,
                    executed: false,
                    profile,
                    authorization,
                    approval: Some(approval),
                    message:
                        "Action suspendue : demande de validation créée."
                            .to_string(),
                }
            }

            AuthorizationDecision::Allowed => {
                /*
                 * Le Core actuel ne possède pas encore
                 * de connecteur réel.
                 *
                 * L'autorisation est donc validée,
                 * mais aucune action externe n'est exécutée
                 * à ce stade.
                 */

                ActionResult {
                    description,
                    success: true,
                    executed: false,
                    profile,
                    authorization,
                    approval: None,
                    message:
                        "Action autorisée et prête pour un connecteur réel."
                            .to_string(),
                }
            }
        }
    }

    fn profile_from_decision(
        decision: &Decision,
    ) -> ActionRiskProfile {
        use crate::reflection::VerificationLevel;

        match decision.verification_level {
            VerificationLevel::Critical => {
                ActionRiskProfile::critical()
            }

            VerificationLevel::Reinforced => {
                ActionRiskProfile::high()
            }

            VerificationLevel::Standard => {
                ActionRiskProfile::medium()
            }

            VerificationLevel::Minimal => {
                ActionRiskProfile::low()
            }
        }
    }
}
