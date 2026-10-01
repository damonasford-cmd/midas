use serde::{Deserialize, Serialize};

use crate::perception::{
    Perception,
    PerceptionModality,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Understanding {
    pub summary: String,
    pub facts: Vec<String>,
    pub uncertainties: Vec<String>,
    pub sources: Vec<String>,
    pub modalities: Vec<PerceptionModality>,
}

impl Understanding {
    pub fn new(
        summary: impl Into<String>,
    ) -> Self {
        Self {
            summary: summary.into(),
            facts: Vec::new(),
            uncertainties: Vec::new(),
            sources: Vec::new(),
            modalities: Vec::new(),
        }
    }

    pub fn add_fact(
        &mut self,
        fact: impl Into<String>,
    ) {
        self.facts.push(fact.into());
    }

    pub fn add_uncertainty(
        &mut self,
        uncertainty: impl Into<String>,
    ) {
        self.uncertainties.push(
            uncertainty.into(),
        );
    }

    pub fn add_source(
        &mut self,
        source: impl Into<String>,
    ) {
        let source = source.into();

        if !self.sources.contains(&source) {
            self.sources.push(source);
        }
    }

    pub fn add_modality(
        &mut self,
        modality: PerceptionModality,
    ) {
        if !self.modalities.contains(&modality) {
            self.modalities.push(modality);
        }
    }

    pub fn has_uncertainties(&self) -> bool {
        !self.uncertainties.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "{} fait(s), {} incertitude(s), \
             {} source(s), {} modalité(s).",
            self.facts.len(),
            self.uncertainties.len(),
            self.sources.len(),
            self.modalities.len(),
        )
    }
}

#[derive(Debug, Default)]
pub struct Cognition;

impl Cognition {
    pub fn understand(
        perceptions: &[Perception],
    ) -> Understanding {
        if perceptions.is_empty() {
            let mut understanding =
                Understanding::new(
                    "Aucune information perçue.",
                );

            understanding.add_uncertainty(
                "Informations insuffisantes pour comprendre la situation.",
            );

            return understanding;
        }

        let mut understanding =
            Understanding::new(
                format!(
                    "{} perception(s) intégrée(s) dans une compréhension unifiée.",
                    perceptions.len()
                ),
            );

        for perception in perceptions {
            understanding.add_source(
                perception.source.clone(),
            );

            understanding.add_modality(
                perception.modality,
            );

            let fact = format!(
                "[{:?}] {} : {}",
                perception.modality,
                perception.source,
                perception.content,
            );

            understanding.add_fact(
                fact,
            );

            if !perception.is_reliable() {
                understanding.add_uncertainty(
                    format!(
                        "Fiabilité faible de la perception {} : {:.2}",
                        perception.source,
                        perception.confidence
                    ),
                );
            }

            if perception.content.trim().is_empty() {
                understanding.add_uncertainty(
                    format!(
                        "Contenu vide provenant de {}.",
                        perception.source
                    ),
                );
            }
        }

        understanding
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

    pub fn identify_conflicts(
        perceptions: &[Perception],
    ) -> Vec<String> {
        let mut conflicts =
            Vec::new();

        for i in 0..perceptions.len() {
            for j in (i + 1)..perceptions.len() {
                let left =
                    &perceptions[i];

                let right =
                    &perceptions[j];

                if left.source == right.source
                    && left.content != right.content
                {
                    conflicts.push(
                        format!(
                            "Informations divergentes provenant de {}.",
                            left.source
                        ),
                    );
                }
            }
        }

        conflicts
    }
}
