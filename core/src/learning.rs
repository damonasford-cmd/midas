use crate::observation::Observation;

#[derive(Debug, Clone)]
pub struct Learning {
    pub lesson: String,
}

pub struct LearningEngine;

impl LearningEngine {
    pub fn learn(observation: &Observation) -> Learning {
        let lesson = if observation.success {
            format!("Résultat observé avec succès : {}", observation.result)
        } else {
            format!("Échec observé : {}", observation.result)
        };

        Learning { lesson }
    }
}
