use crate::learning::Learning;

#[derive(Debug, Clone)]
pub struct Correction {
    pub required: bool,
    pub action: String,
}

pub struct CorrectionEngine;

impl CorrectionEngine {
    pub fn evaluate(learning: &Learning) -> Correction {
        Correction {
            required: false,
            action: format!("Évaluation terminée : {}", learning.lesson),
        }
    }
}
