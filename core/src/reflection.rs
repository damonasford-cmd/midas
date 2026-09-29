use crate::cognition::Understanding;

#[derive(Debug, Clone)]
pub struct Reflection {
    pub reasoning: String,
    pub confidence: f32,
}

pub fn reflect(
    goal: &str,
    understanding: &Understanding,
) -> Reflection {
    let confidence = if understanding.facts.is_empty() {
        0.2
    } else {
        0.8
    };

    Reflection {
        reasoning: format!(
            "Réflexion sur l'objectif '{goal}'. {}",
            understanding.summary
        ),
        confidence,
    }
}
