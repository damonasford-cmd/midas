use crate::observation::Observation;

#[derive(Debug, Clone)]
pub struct Learning {
    pub lesson: String,
}

pub fn learn(observation: &Observation) -> Learning {
    let lesson = if observation.success {
        "L'action observée a produit le résultat attendu."
            .into()
    } else {
        "L'action observée nécessite une analyse corrective."
            .into()
    };

    Learning { lesson }
}
