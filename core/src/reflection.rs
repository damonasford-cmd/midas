use crate::cognition::Understanding;

#[derive(Debug, Clone)]
pub struct Reflection {
    pub reasoning: String,
    pub confidence: f32,
}

pub struct ReflectionEngine;

impl ReflectionEngine {
    pub fn reflect(understanding: &Understanding) -> Reflection {
        Reflection {
            reasoning: format!(
                "Analyse interne de la situation : {}",
                understanding.summary
            ),
            confidence: if understanding.relevant_information.is_empty() {
                0.0
            } else {
                0.5
            },
        }
    }
}
