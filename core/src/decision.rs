use crate::reflection::Reflection;

#[derive(Debug, Clone)]
pub struct Decision {
    pub action: String,
    pub rationale: String,
}

pub fn decide(
    goal: &str,
    reflection: &Reflection,
) -> Decision {
    Decision {
        action: format!(
            "Préparer l'action nécessaire pour: {goal}"
        ),
        rationale: format!(
            "{} Confiance: {:.2}",
            reflection.reasoning,
            reflection.confidence
        ),
    }
}
