use crate::reflection::{
    Reflection,
    VerificationLevel,
};

#[derive(Debug, Clone)]
pub struct Decision {
    pub action: String,
    pub rationale: String,
    pub confidence: f32,
    pub requires_verification: bool,
    pub verification_level: VerificationLevel,
}

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn decide(
        reflection: &Reflection,
    ) -> Decision {
        let requires_verification =
            match reflection.verification_level {
                VerificationLevel::Minimal => false,

                VerificationLevel::Standard
                | VerificationLevel::Reinforced
                | VerificationLevel::Critical => true,
            };

        let confidence =
            Self::calculate_confidence(
                reflection,
            );

        let action =
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
            };

        let rationale = format!(
            "Décision fondée sur une réflexion {:?}, une complexité {:?}, un impact {:?}, une réversibilité {:?}, {} hypothèse(s) et {} risque(s). Niveau de vérification : {:?}.",
            reflection.depth,
            reflection.complexity,
            reflection.impact,
            reflection.reversibility,
            reflection.hypotheses.len(),
            reflection.risks.len(),
            reflection.verification_level,
        );

        Decision {
            action,
            rationale,
            confidence,
            requires_verification,
            verification_level:
                reflection.verification_level,
        }
    }

    fn calculate_confidence(
        reflection: &Reflection,
    ) -> f32 {
        let mut confidence = 0.95_f32;

        confidence -=
            reflection.uncertainty_penalty();

        confidence -=
            reflection.risk_penalty();

        confidence -=
            reflection.impact_penalty();

        confidence.clamp(0.0, 1.0)
    }
}

trait ReflectionConfidence {
    fn uncertainty_penalty(&self) -> f32;
    fn risk_penalty(&self) -> f32;
    fn impact_penalty(&self) -> f32;
}

impl ReflectionConfidence for Reflection {
    fn uncertainty_penalty(&self) -> f32 {
        (self
            .verification_points
            .iter()
            .filter(|point| {
                point.contains("incertitude")
                    || point.contains("informations")
            })
            .count() as f32)
            * 0.03
    }

    fn risk_penalty(&self) -> f32 {
        (self.risks.len() as f32) * 0.05
    }

    fn impact_penalty(&self) -> f32 {
        match self.impact {
            crate::reflection::ImpactLevel::Low => 0.0,

            crate::reflection::ImpactLevel::Medium => 0.05,

            crate::reflection::ImpactLevel::High => 0.12,

            crate::reflection::ImpactLevel::Critical => 0.25,
        }
    }
}
