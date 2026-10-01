use crate::cognition::Understanding;

#[derive(Debug, Clone)]
pub struct Reflection {
    pub reasoning: String,
    pub hypotheses: Vec<String>,
    pub risks: Vec<String>,
    pub verification_points: Vec<String>,
    pub depth: ReflectionDepth,
}

#[derive(Debug, Clone, Copy)]
pub enum ReflectionDepth {
    Short,
    Normal,
    Deep,
}

pub struct ReflectionEngine;

impl ReflectionEngine {
    pub fn reflect(
        understanding: &Understanding,
    ) -> Reflection {
        let depth =
            Self::choose_depth(
                understanding,
            );

        let hypotheses =
            Self::build_hypotheses(
                understanding,
            );

        let risks =
            Self::identify_risks(
                understanding,
            );

        let verification_points =
            Self::verification_points(
                understanding,
            );

        let reasoning = format!(
            "Réflexion {:?}. {} fait(s), {} incertitude(s), {} hypothèse(s), {} risque(s).",
            depth,
            understanding.facts.len(),
            understanding.uncertainties.len(),
            hypotheses.len(),
            risks.len()
        );

        Reflection {
            reasoning,
            hypotheses,
            risks,
            verification_points,
            depth,
        }
    }

    fn choose_depth(
        understanding: &Understanding,
    ) -> ReflectionDepth {
        if understanding.facts.len() <= 1
            && understanding.uncertainties.is_empty()
        {
            ReflectionDepth::Short
        } else if understanding.uncertainties.len() <= 2 {
            ReflectionDepth::Normal
        } else {
            ReflectionDepth::Deep
        }
    }

    fn build_hypotheses(
        understanding: &Understanding,
    ) -> Vec<String> {
        understanding
            .facts
            .iter()
            .map(|fact| {
                format!(
                    "Hypothèse à vérifier : {}",
                    fact
                )
            })
            .collect()
    }

    fn identify_risks(
        understanding: &Understanding,
    ) -> Vec<String> {
        understanding
            .uncertainties
            .iter()
            .map(|uncertainty| {
                format!(
                    "Risque lié à l'incertitude : {}",
                    uncertainty
                )
            })
            .collect()
    }

    fn verification_points(
        understanding: &Understanding,
    ) -> Vec<String> {
        understanding
            .facts
            .iter()
            .map(|fact| {
                format!(
                    "Vérifier : {}",
                    fact
                )
            })
            .collect()
    }
}
