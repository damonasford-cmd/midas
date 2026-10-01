use crate::decision::Decision;

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub description: String,
}

pub struct ActionEngine;

impl ActionEngine {
    pub fn execute(decision: &Decision) -> ActionResult {
        match decision {
            Decision::NoAction => ActionResult {
                success: true,
                description: "Aucune action requise.".to_string(),
            },

            Decision::Act(description) => ActionResult {
                success: false,
                description: format!(
                    "Action proposée mais aucun connecteur d'exécution n'est encore attaché : {}",
                    description
                ),
            },

            Decision::RequestInformation(request) => ActionResult {
                success: true,
                description: format!("Information requise : {}", request),
            },
        }
    }
}
