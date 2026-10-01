use crate::perception::Perception;

#[derive(Debug, Clone)]
pub struct Understanding {
    pub summary: String,
    pub facts: Vec<String>,
    pub uncertainties: Vec<String>,
}

impl Understanding {
    pub fn new(
        summary: impl Into<String>,
    ) -> Self {
        Self {
            summary: summary.into(),
            facts: Vec::new(),
            uncertainties: Vec::new(),
        }
    }
}

pub struct Cognition;

impl Cognition {
    pub fn understand(
        perceptions: &[Perception],
    ) -> Understanding {
        if perceptions.is_empty() {
            return Understanding {
                summary: "Aucune information perçue.".to_string(),
                facts: Vec::new(),
                uncertainties: vec![
                    "Informations insuffisantes pour comprendre la situation."
                        .to_string(),
                ],
            };
        }

        let facts = perceptions
            .iter()
            .map(|perception| {
                format!(
                    "[{}] {}",
                    perception.source,
                    perception.content
                )
            })
            .collect::<Vec<_>>();

        let summary = format!(
            "{} perception(s) analysée(s).",
            perceptions.len()
        );

        Understanding {
            summary,
            facts,
            uncertainties: Vec::new(),
        }
    }

    pub fn decompose(
        understanding: &Understanding,
    ) -> Vec<String> {
        if understanding.facts.is_empty() {
            return vec![
                "Déterminer les informations nécessaires avant toute action."
                    .to_string(),
            ];
        }

        understanding
            .facts
            .iter()
            .map(|fact| {
                format!(
                    "Analyser le problème contenu dans : {}",
                    fact
                )
            })
            .collect()
    }

    pub fn missing_information(
        understanding: &Understanding,
    ) -> Vec<String> {
        understanding
            .uncertainties
            .clone()
    }
}
