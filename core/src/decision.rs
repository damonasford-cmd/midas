use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::action::{
    ActionDomain,
    ActionImpact,
    ActionReversibility,
};
use crate::reflection::{
    ImpactLevel,
    Reflection,
    ReflectionDepth,
    Reversibility,
    VerificationLevel,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: Uuid,

    pub action: String,

    pub rationale: String,

    pub confidence: f32,

    pub depth: ReflectionDepth,

    pub verification_level: VerificationLevel,

    pub impact: ImpactLevel,

    pub reversibility: Reversibility,

    pub action_domain: ActionDomain,

    pub requires_verification: bool,

    pub requires_approval: bool,

    pub executable: bool,

    pub alternatives: Vec<String>,

    pub conditions: Vec<String>,

    pub risks: Vec<String>,
}

impl Decision {
    pub fn summary(&self) -> String {
        format!(
            "Décision {} | action={} | confiance={:.2} | \
             profondeur={:?} | vérification={:?} | \
             impact={:?} | réversibilité={:?} | \
             exécutable={} | approbation={}",
            self.id,
            self.action,
            self.confidence,
            self.depth,
            self.verification_level,
            self.impact,
            self.reversibility,
            self.executable,
            self.requires_approval,
        )
    }

    pub fn is_critical(&self) -> bool {
        self.impact == ImpactLevel::Critical
            || self.reversibility
                == Reversibility::Irreversible
            || self.verification_level
                == VerificationLevel::Critical
    }

    pub fn can_execute(&self) -> bool {
        self.executable
            && !self.requires_approval
            && !self.requires_verification
    }
}

#[derive(Debug, Default)]
pub struct DecisionEngine;

impl DecisionEngine {
    pub fn decide(
        reflection: &Reflection,
    ) -> Decision {
        let id =
            Uuid::new_v4();

        let requires_verification =
            Self::requires_verification(
                reflection,
            );

        let requires_approval =
            Self::requires_approval(
                reflection,
            );

        let confidence =
            Self::calculate_confidence(
                reflection,
            );

        let executable =
            Self::is_executable(
                reflection,
                confidence,
                requires_verification,
                requires_approval,
            );

        let action =
            Self::build_action(
                reflection,
            );

        let rationale =
            Self::build_rationale(
                reflection,
                confidence,
            );

        let alternatives =
            Self::build_alternatives(
                reflection,
            );

        let conditions =
            Self::build_conditions(
                reflection,
                requires_verification,
                requires_approval,
            );

        let risks =
            reflection.risks.clone();

        let action_domain =
            Self::infer_action_domain(
                reflection,
            );

        Decision {
            id,
            action,
            rationale,
            confidence,
            depth: reflection.depth,
            verification_level:
                reflection.verification_level,
            impact: reflection.impact,
            reversibility:
                reflection.reversibility,
            action_domain,
            requires_verification,
            requires_approval,
            executable,
            alternatives,
            conditions,
            risks,
        }
    }

    fn requires_verification(
        reflection: &Reflection,
    ) -> bool {
        !matches!(
            reflection.verification_level,
            VerificationLevel::Minimal
        )
    }

    fn requires_approval(
        reflection: &Reflection,
    ) -> bool {
        matches!(
            reflection.verification_level,
            VerificationLevel::Critical
        ) || reflection.impact
            == ImpactLevel::Critical
            || reflection.reversibility
                == Reversibility::Irreversible
    }

    fn calculate_confidence(
        reflection: &Reflection,
    ) -> f32 {
        let mut confidence =
            0.95_f32;

        confidence -=
            reflection
                .missing_information
                .len() as f32
                * 0.08;

        confidence -=
            reflection
                .risks
                .len() as f32
                * 0.05;

        confidence -=
            match reflection.impact {
                ImpactLevel::Low => 0.0,

                ImpactLevel::Medium => 0.04,

                ImpactLevel::High => 0.10,

                ImpactLevel::Critical => 0.20,
            };

        confidence -=
            match reflection.reversibility {
                Reversibility::Reversible => 0.0,

                Reversibility::PartiallyReversible => {
                    0.05
                }

                Reversibility::Irreversible => {
                    0.15
                }
            };

        confidence.clamp(
            0.0,
            1.0,
        )
    }

    fn is_executable(
        reflection: &Reflection,
        confidence: f32,
        requires_verification: bool,
        requires_approval: bool,
    ) -> bool {
        if requires_approval {
            return false;
        }

        if reflection.impact
            == ImpactLevel::Critical
        {
            return false;
        }

        if reflection.reversibility
            == Reversibility::Irreversible
        {
            return false;
        }

        if confidence < 0.70 {
            return false;
        }

        if matches!(
            reflection.verification_level,
            VerificationLevel::Critical
        ) {
            return false;
        }

        if requires_verification {
            return false;
        }

        true
    }

    fn build_action(
        reflection: &Reflection,
    ) -> String {
        match reflection.verification_level {
            VerificationLevel::Minimal => {
                "Poursuivre avec l'action déterminée."
                    .to_string()
            }

            VerificationLevel::Standard => {
                "Effectuer les vérifications nécessaires avant exécution."
                    .to_string()
            }

            VerificationLevel::Reinforced => {
                "Effectuer une vérification renforcée avant exécution."
                    .to_string()
            }

            VerificationLevel::Critical => {
                "Suspendre l'exécution critique jusqu'à validation complète."
                    .to_string()
            }
        }
    }

    fn build_rationale(
        reflection: &Reflection,
        confidence: f32,
    ) -> String {
        format!(
            "Décision fondée sur une réflexion {:?}, \
             une complexité {:?}, un impact {:?}, \
             une réversibilité {:?}, une vérification {:?}, \
             {} hypothèse(s), {} risque(s), \
             {} information(s) manquante(s). \
             Confiance estimée : {:.2}.",
            reflection.depth,
            reflection.complexity,
            reflection.impact,
            reflection.reversibility,
            reflection.verification_level,
            reflection.hypotheses.len(),
            reflection.risks.len(),
            reflection.missing_information.len(),
            confidence,
        )
    }

    fn build_alternatives(
        reflection: &Reflection,
    ) -> Vec<String> {
        let mut alternatives =
            Vec::new();

        if !reflection
            .missing_information
            .is_empty()
        {
            alternatives.push(
                "Rechercher les informations manquantes avant de décider."
                    .to_string(),
            );
        }

        if !reflection
            .risks
            .is_empty()
        {
            alternatives.push(
                "Réduire les risques avant exécution."
                    .to_string(),
            );
        }

        alternatives.push(
            "Effectuer une simulation ou un test isolé lorsque possible."
                .to_string(),
        );

        alternatives
    }

    fn build_conditions(
        reflection: &Reflection,
        requires_verification: bool,
        requires_approval: bool,
    ) -> Vec<String> {
        let mut conditions =
            Vec::new();

        if requires_verification {
            conditions.push(
                "Les points de vérification doivent être contrôlés avant exécution."
                    .to_string(),
            );
        }

        if requires_approval {
            conditions.push(
                "Une validation explicite est nécessaire avant l'action."
                    .to_string(),
            );
        }

        if reflection.confidence_below_threshold() {
            conditions.push(
                "La confiance est insuffisante pour une exécution autonome."
                    .to_string(),
            );
        }

        conditions
    }

    fn infer_action_domain(
        reflection: &Reflection,
    ) -> ActionDomain {
        match reflection.impact {
            ImpactLevel::Critical => {
                ActionDomain::Infrastructure
            }

            ImpactLevel::High => {
                ActionDomain::Software
            }

            ImpactLevel::Medium => {
                ActionDomain::Information
            }

            ImpactLevel::Low => {
                ActionDomain::Internal
            }
        }
    }

    pub fn map_impact(
        impact: ImpactLevel,
    ) -> ActionImpact {
        match impact {
            ImpactLevel::Low => {
                ActionImpact::Low
            }

            ImpactLevel::Medium => {
                ActionImpact::Medium
            }

            ImpactLevel::High => {
                ActionImpact::High
            }

            ImpactLevel::Critical => {
                ActionImpact::Critical
            }
        }
    }

    pub fn map_reversibility(
        reversibility: Reversibility,
    ) -> ActionReversibility {
        match reversibility {
            Reversibility::Reversible => {
                ActionReversibility::Reversible
            }

            Reversibility::PartiallyReversible => {
                ActionReversibility::PartiallyReversible
            }

            Reversibility::Irreversible => {
                ActionReversibility::Irreversible
            }
        }
    }
}

trait ReflectionDecisionExt {
    fn confidence_below_threshold(
        &self,
    ) -> bool;
}

impl ReflectionDecisionExt
    for Reflection
{
    fn confidence_below_threshold(
        &self,
    ) -> bool {
        let mut confidence =
            0.95_f32;

        confidence -=
            self.missing_information
                .len() as f32
                * 0.08;

        confidence -=
            self.risks.len() as f32
            * 0.05;

        confidence -=
            match self.impact {
                ImpactLevel::Low => 0.0,

                ImpactLevel::Medium => 0.04,

                ImpactLevel::High => 0.10,

                ImpactLevel::Critical => 0.20,
            };

        confidence -=
            match self.reversibility {
                Reversibility::Reversible => 0.0,

                Reversibility::PartiallyReversible => {
                    0.05
                }

                Reversibility::Irreversible => {
                    0.15
                }
            };

        confidence < 0.70
    }
}
