use crate::decision::Decision;

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
            ActionImpact::High
                | ActionImpact::Critical
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
        profile.domain = ActionDomain::Infrastructure;

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
            ActionImpact::High
                | ActionImpact::Critical
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

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub description: String,
    pub success: bool,
    pub executed: bool,
    pub profile: ActionRiskProfile,
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

        /*
         * À ce stade, le moteur ne possède pas encore
         * les connecteurs réels permettant d'agir sur
         * le monde extérieur.
         *
         * On ne prétend donc pas qu'une action réelle
         * a été exécutée.
         *
         * Cette couche prépare le contrat qui sera utilisé
         * par les futurs connecteurs.
         */

        let profile =
            ActionRiskProfile::default();

        if decision.requires_verification {
            return ActionResult {
                description,
                success: false,
                executed: false,
                profile,
                message:
                    "Action non exécutée : vérification requise avant exécution."
                        .to_string(),
            };
        }

        ActionResult {
            description,
            success: true,
            executed: false,
            profile,
            message:
                "Action préparée. Aucun connecteur réel n'est encore attaché."
                    .to_string(),
        }
    }
}
