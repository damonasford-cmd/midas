use crate::perception::Perception;

#[derive(Debug, Clone)]
pub struct Understanding {
    pub summary: String,
    pub relevant_information: Vec<String>,
}

pub struct Cognition;

impl Cognition {
    pub fn understand(perceptions: &[Perception]) -> Understanding {
        let relevant_information = perceptions
            .iter()
            .map(|p| format!("[{:?}] {}: {}", p.kind, p.source, p.content))
            .collect::<Vec<_>>();

        let summary = if relevant_information.is_empty() {
            "Aucune information perceptive disponible.".to_string()
        } else {
            format!(
                "{} élément(s) perceptif(s) reçu(s).",
                relevant_information.len()
            )
        };

        Understanding {
            summary,
            relevant_information,
        }
    }
}
