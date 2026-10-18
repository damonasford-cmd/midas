
use serde::{Deserialize, Serialize};

/// Protocole DAMON is mandatory for actions involving at least €500,000.
pub const DAMON_APPROVAL_THRESHOLD_EUR_CENTS: u64 = 50_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRisk {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalImpact {
    None,
    Reversible,
    Irreversible,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeStatus {
    Granted,
    Missing,
    Revoked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarmAssessment {
    NoKnownUnjustifiedHarm,
    Uncertain,
    KnownUnjustifiedHarm,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    NotRequested,
    Pending,
    Approved,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionProposal {
    pub id: String,
    pub description: String,

    /// Estimated total commitment, expressed in euro cents.
    /// Use None when the cost is unknown or cannot be estimated reliably.
    pub estimated_cost_eur_cents: Option<u64>,

    pub risk: ActionRisk,
    pub external_impact: ExternalImpact,
    pub scope: ScopeStatus,
    pub harm_assessment: HarmAssessment,

    /// True for actions that could materially affect safety.
    pub safety_critical: bool,

    /// True when the action exposes or materially processes sensitive data.
    pub sensitive_data: bool,

    /// Approval by an authorized operator for ordinary approval requirements.
    pub operator_approval: ApprovalStatus,

    /// Separate approval status for the DAMON protocol.
    pub damon_approval: ApprovalStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationDecision {
    Allowed {
        reasons: Vec<String>,
    },
    RequiresApproval {
        reasons: Vec<String>,
    },
    Denied {
        reasons: Vec<String>,
    },
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AuthorizationPolicy;

impl AuthorizationPolicy {
    pub fn evaluate(
        &self,
        proposal: &ActionProposal,
    ) -> AuthorizationDecision {
        let mut denial_reasons = Vec::new();

        if proposal.id.trim().is_empty() {
            denial_reasons.push(
                "The action must have a non-empty identifier.".to_string(),
            );
        }

        if proposal.description.trim().is_empty() {
            denial_reasons.push(
                "The action must have a non-empty description.".to_string(),
            );
        }

        if proposal.id.len() > 256 || proposal.description.len() > 16_384 {
            denial_reasons.push(
                "The action metadata exceeds the permitted size.".to_string(),
            );
        }

        match proposal.scope {
            ScopeStatus::Granted => {}
            ScopeStatus::Missing => denial_reasons.push(
                "The required authorization scope has not been granted."
                    .to_string(),
            ),
            ScopeStatus::Revoked => denial_reasons.push(
                "The required authorization scope has been revoked."
                    .to_string(),
            ),
        }

        if proposal.harm_assessment
            == HarmAssessment::KnownUnjustifiedHarm
        {
            denial_reasons.push(
                "The proposed action is assessed as causing known \
                 unjustified harm.".to_string(),
            );
        }

        if proposal.operator_approval == ApprovalStatus::Rejected {
            denial_reasons.push(
                "The authorized operator rejected the action.".to_string(),
            );
        }

        if proposal.damon_approval == ApprovalStatus::Rejected {
            denial_reasons.push(
                "The DAMON protocol approval was rejected.".to_string(),
            );
        }

        if !denial_reasons.is_empty() {
            return AuthorizationDecision::Denied {
                reasons: denial_reasons,
            };
        }

        let mut approval_reasons = Vec::new();

        let requires_operator_approval = matches!(
            proposal.risk,
            ActionRisk::High | ActionRisk::Critical
        ) || proposal.external_impact == ExternalImpact::Irreversible
            || proposal.safety_critical
            || proposal.sensitive_data
            || proposal.harm_assessment == HarmAssessment::Uncertain;

        if requires_operator_approval {
            match proposal.operator_approval {
                ApprovalStatus::Approved => {}
                ApprovalStatus::NotRequested | ApprovalStatus::Pending => {
                    approval_reasons.push(
                        "An authorized operator must review and approve \
                         this action before execution.".to_string(),
                    );
                }
                ApprovalStatus::Rejected => unreachable!(
                    "Rejected approvals were handled above"
                ),
            }
        }

        let damon_protocol_required = proposal
            .estimated_cost_eur_cents
            .map(|cost| cost >= DAMON_APPROVAL_THRESHOLD_EUR_CENTS)
            .unwrap_or(false);

        if damon_protocol_required {
            match proposal.damon_approval {
                ApprovalStatus::Approved => {}
                ApprovalStatus::NotRequested | ApprovalStatus::Pending => {
                    approval_reasons.push(
                        "The action reaches the €500,000 threshold and \
                         requires explicit DAMON protocol approval."
                            .to_string(),
                    );
                }
                ApprovalStatus::Rejected => unreachable!(
                    "Rejected approvals were handled above"
                ),
            }
        }

        if proposal.estimated_cost_eur_cents.is_none() {
            approval_reasons.push(
                "The financial commitment is unknown; estimate the cost \
                 before making a consequential commitment.".to_string(),
            );
        }

        if !approval_reasons.is_empty() {
            return AuthorizationDecision::RequiresApproval {
                reasons: approval_reasons,
            };
        }

        AuthorizationDecision::Allowed {
            reasons: vec![
                "The proposal passed this policy's configured checks."
                    .to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ordinary_proposal() -> ActionProposal {
        ActionProposal {
            id: "action-001".to_string(),
            description: "Create a local test file".to_string(),
            estimated_cost_eur_cents: Some(0),
            risk: ActionRisk::Low,
            external_impact: ExternalImpact::None,
            scope: ScopeStatus::Granted,
            harm_assessment: HarmAssessment::NoKnownUnjustifiedHarm,
            safety_critical: false,
            sensitive_data: false,
            operator_approval: ApprovalStatus::NotRequested,
            damon_approval: ApprovalStatus::NotRequested,
        }
    }

    #[test]
    fn low_risk_authorized_action_is_allowed() {
        let policy = AuthorizationPolicy;
        let proposal = ordinary_proposal();

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::Allowed { .. }
        ));
    }

    #[test]
    fn missing_scope_is_denied() {
        let policy = AuthorizationPolicy;
        let mut proposal = ordinary_proposal();
        proposal.scope = ScopeStatus::Missing;

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::Denied { .. }
        ));
    }

    #[test]
    fn known_unjustified_harm_is_denied() {
        let policy = AuthorizationPolicy;
        let mut proposal = ordinary_proposal();
        proposal.harm_assessment = HarmAssessment::KnownUnjustifiedHarm;

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::Denied { .. }
        ));
    }

    #[test]
    fn irreversible_action_requires_operator_approval() {
        let policy = AuthorizationPolicy;
        let mut proposal = ordinary_proposal();
        proposal.external_impact = ExternalImpact::Irreversible;

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::RequiresApproval { .. }
        ));

        proposal.operator_approval = ApprovalStatus::Approved;

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::Allowed { .. }
        ));
    }

    #[test]
    fn damon_threshold_requires_explicit_approval() {
        let policy = AuthorizationPolicy;
        let mut proposal = ordinary_proposal();

        proposal.estimated_cost_eur_cents =
            Some(DAMON_APPROVAL_THRESHOLD_EUR_CENTS);

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::RequiresApproval { .. }
        ));

        proposal.damon_approval = ApprovalStatus::Approved;

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::Allowed { .. }
        ));
    }

    #[test]
    fn cost_just_below_threshold_does_not_trigger_damon_protocol() {
        let policy = AuthorizationPolicy;
        let mut proposal = ordinary_proposal();

        proposal.estimated_cost_eur_cents =
            Some(DAMON_APPROVAL_THRESHOLD_EUR_CENTS - 1);

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::Allowed { .. }
        ));
    }

    #[test]
    fn unknown_cost_requires_review() {
        let policy = AuthorizationPolicy;
        let mut proposal = ordinary_proposal();
        proposal.estimated_cost_eur_cents = None;

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::RequiresApproval { .. }
        ));
    }

    #[test]
    fn rejected_damon_approval_denies_execution() {
        let policy = AuthorizationPolicy;
        let mut proposal = ordinary_proposal();

        proposal.estimated_cost_eur_cents =
            Some(DAMON_APPROVAL_THRESHOLD_EUR_CENTS);
        proposal.damon_approval = ApprovalStatus::Rejected;

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::Denied { .. }
        ));
    }

    #[test]
    fn sensitive_data_requires_operator_approval() {
        let policy = AuthorizationPolicy;
        let mut proposal = ordinary_proposal();
        proposal.sensitive_data = true;

        assert!(matches!(
            policy.evaluate(&proposal),
            AuthorizationDecision::RequiresApproval { .. }
        ));
    }
}
