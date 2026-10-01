use crate::reflection::Reflection;

#[derive(Debug, Clone)]
pub enum Decision {
    NoAction,
    Act(String),
    RequestInformation(String),
}

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn decide(reflection: &Reflection) -> Decision {
        if reflection.confidence <= 0.0 {
            Decision::RequestInformation(
                "Informations insuffisantes pour prendre une décision.".to_string(),
            )
        } else {
            Decision::NoAction
        }
    }
}
