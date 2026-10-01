use crate::reflection::Reflection;

#[derive(Debug, Clone)]
pub struct Decision {
    pub action: String,
    pub rationale: String,
    pub confidence: f32,
    pub requires_verification: bool,
}

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn decide(
        reflection: &Reflection,
    ) -> Decision {
        let requires_verification =
            !reflection
                .verification_points
                .is_empty()
                || !reflection.risks.is_empty();

        let confidence =
            if reflection.risks.is_empty() {
                0.9
            } else {
                0.6
            };

        let action =
            if requires_verification {
                "Vérifier les informations critiques avant exécution."
                    .to_string()
            } else {
                "Poursuivre avec l'action déterminée."
                    .to_string()
            };

        let rationale = format!(
            "Décision fondée sur une réflexion {:?}, {} hypothèse(s) et {} risque(s).",
            reflection.depth,
            reflection.hypotheses.len(),
            reflection.risks.len()
        );

        Decision {
            action,
            rationale,
            confidence,
            requires_verification,
        }
    }
}
