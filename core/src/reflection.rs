use serde::{Deserialize, Serialize};

use crate::cognition::Understanding;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reflection {
    pub reasoning: String,
    pub hypotheses: Vec<String>,
    pub risks: Vec<String>,
    pub verification_points: Vec<String>,
    pub missing_information: Vec<String>,
    pub decomposition: Vec<String>,
    pub depth: ReflectionDepth,
    pub verification_level: VerificationLevel,
    pub complexity: ComplexityLevel,
    pub impact: ImpactLevel,
    pub reversibility: Reversibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReflectionDepth {
    Short,
    Normal,
    Deep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationLevel {
    Minimal,
    Standard,
    Reinforced,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reversibility {
    Reversible,
    PartiallyReversible,
    Irreversible,
}

pub struct ReflectionEngine;

impl ReflectionEngine {
    pub fn reflect(
        understanding: &Understanding,
    ) -> Reflection {
        let complexity =
            Self::assess_complexity(
                understanding,
            );

        let impact =
            Self::assess_impact(
                understanding,
            );

        let reversibility =
            Self::assess_reversibility(
                understanding,
            );

        let depth =
            Self::choose_depth(
                complexity,
                impact,
                reversibility,
                understanding,
            );

        let verification_level =
            Self::choose_verification_level(
                complexity,
                impact,
                reversibility,
                understanding,
            );

        let decomposition =
            Self::decompose(
                understanding,
            );

        let missing_information =
            understanding
                .uncertainties
                .clone();

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
                verification_level,
            );

        let reasoning =
            Self::build_reasoning(
                understanding,
                complexity,
                impact,
                reversibility,
                depth,
                verification_level,
                &decomposition,
                &hypotheses,
                &risks,
                &verification_points,
            );

        Reflection {
            reasoning,
            hypotheses,
            risks,
            verification_points,
            missing_information,
            decomposition,
            depth,
            verification_level,
            complexity,
            impact,
            reversibility,
        }
    }

    fn assess_complexity(
        understanding: &Understanding,
    ) -> ComplexityLevel {
        let facts =
            understanding.facts.len();

        let uncertainties =
            understanding
                .uncertainties
                .len();

        let modalities =
            understanding
                .modalities
                .len();

        if facts <= 1
            && uncertainties == 0
            && modalities <= 1
        {
            ComplexityLevel::Low
        } else if facts <= 4
            && uncertainties <= 2
            && modalities <= 3
        {
            ComplexityLevel::Medium
        } else if facts <= 8
            && uncertainties <= 4
            && modalities <= 6
        {
            ComplexityLevel::High
        } else {
            ComplexityLevel::VeryHigh
        }
    }

    fn assess_impact(
        understanding: &Understanding,
    ) -> ImpactLevel {
        let uncertainty_count =
            understanding
                .uncertainties
                .len();

        let fact_count =
            understanding.facts.len();

        if uncertainty_count >= 5 {
            ImpactLevel::Critical
        } else if fact_count >= 8
            || uncertainty_count >= 3
        {
            ImpactLevel::High
        } else if fact_count >= 3
            || uncertainty_count >= 1
        {
            ImpactLevel::Medium
        } else {
            ImpactLevel::Low
        }
    }

    fn assess_reversibility(
        understanding: &Understanding,
    ) -> Reversibility {
        if understanding
            .uncertainties
            .len()
            >= 5
        {
            Reversibility::Irreversible
        } else if understanding
            .uncertainties
            .len()
            >= 2
        {
            Reversibility::PartiallyReversible
        } else {
            Reversibility::Reversible
        }
    }

    fn choose_depth(
        complexity: ComplexityLevel,
        impact: ImpactLevel,
        reversibility: Reversibility,
        understanding: &Understanding,
    ) -> ReflectionDepth {
        if impact == ImpactLevel::Critical
            || reversibility
                == Reversibility::Irreversible
            || complexity
                == ComplexityLevel::VeryHigh
        {
            return ReflectionDepth::Deep;
        }

        if impact == ImpactLevel::High
            || complexity
                == ComplexityLevel::High
            || reversibility
                == Reversibility::PartiallyReversible
        {
            return ReflectionDepth::Deep;
        }

        if complexity
            == ComplexityLevel::Medium
            || !understanding
                .uncertainties
                .is_empty()
        {
            return ReflectionDepth::Normal;
        }

        ReflectionDepth::Short
    }

    fn choose_verification_level(
        complexity: ComplexityLevel,
        impact: ImpactLevel,
        reversibility: Reversibility,
        understanding: &Understanding,
    ) -> VerificationLevel {
        if impact == ImpactLevel::Critical
            || reversibility
                == Reversibility::Irreversible
            || complexity
                == ComplexityLevel::VeryHigh
        {
            return VerificationLevel::Critical;
        }

        if impact == ImpactLevel::High
            || complexity
                == ComplexityLevel::High
            || reversibility
                == Reversibility::PartiallyReversible
        {
            return VerificationLevel::Reinforced;
        }

        if complexity
            == ComplexityLevel::Medium
            || !understanding
                .uncertainties
                .is_empty()
        {
            return VerificationLevel::Standard;
        }

        VerificationLevel::Minimal
    }

    fn decompose(
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
            .enumerate()
            .map(|(index, fact)| {
                format!(
                    "Étape {} : analyser {}",
                    index + 1,
                    fact
                )
            })
            .collect()
    }

    fn build_hypotheses(
        understanding: &Understanding,
    ) -> Vec<String> {
        if understanding.facts.is_empty() {
            return Vec::new();
        }

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
        let mut risks =
            understanding
                .uncertainties
                .iter()
                .map(|uncertainty| {
                    format!(
                        "Risque lié à l'incertitude : {}",
                        uncertainty
                    )
                })
                .collect::<Vec<_>>();

        if understanding
            .facts
            .is_empty()
        {
            risks.push(
                "Absence de faits exploitables."
                    .to_string(),
            );
        }

        risks
    }

    fn verification_points(
        understanding: &Understanding,
        verification_level: VerificationLevel,
    ) -> Vec<String> {
        let mut points =
            understanding
                .facts
                .iter()
                .map(|fact| {
                    format!(
                        "Vérifier : {}",
                        fact
                    )
                })
                .collect::<Vec<_>>();

        for uncertainty in
            &understanding.uncertainties
        {
            points.push(
                format!(
                    "Résoudre ou réduire l'incertitude : {}",
                    uncertainty
                ),
            );
        }

        match verification_level {
            VerificationLevel::Minimal => {}

            VerificationLevel::Standard => {
                points.push(
                    "Vérifier les informations nécessaires avant décision."
                        .to_string(),
                );
            }

            VerificationLevel::Reinforced => {
                points.push(
                    "Vérifier les informations critiques."
                        .to_string(),
                );

                points.push(
                    "Contrôler les conséquences possibles avant exécution."
                        .to_string(),
                );

                points.push(
                    "Effectuer une seconde vérification indépendante lorsque possible."
                        .to_string(),
                );
            }

            VerificationLevel::Critical => {
                points.push(
                    "Effectuer une vérification complète des informations critiques."
                        .to_string(),
                );

                points.push(
                    "Vérifier indépendamment les hypothèses principales."
                        .to_string(),
                );

                points.push(
                    "Évaluer les conséquences et la réversibilité."
                        .to_string(),
                );

                points.push(
                    "Valider les conditions nécessaires avant toute action critique."
                        .to_string(),
                );
            }
        }

        points
    }

    fn build_reasoning(
        understanding: &Understanding,
        complexity: ComplexityLevel,
        impact: ImpactLevel,
        reversibility: Reversibility,
        depth: ReflectionDepth,
        verification_level: VerificationLevel,
        decomposition: &[String],
        hypotheses: &[String],
        risks: &[String],
        verification_points: &[String],
    ) -> String {
        format!(
            "Réflexion {:?}. \
             Complexité={:?}. \
             Impact={:?}. \
             Réversibilité={:?}. \
             Vérification={:?}. \
             Faits={}. \
             Incertitudes={}. \
             Décomposition={}. \
             Hypothèses={}. \
             Risques={}. \
             Vérifications={}.",
            depth,
            complexity,
            impact,
            reversibility,
            verification_level,
            understanding.facts.len(),
            understanding.uncertainties.len(),
            decomposition.len(),
            hypotheses.len(),
            risks.len(),
            verification_points.len(),
        )
    }
}
