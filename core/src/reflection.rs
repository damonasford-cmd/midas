use crate::cognition::Understanding;

#[derive(Debug, Clone)]
pub struct Reflection {
    pub reasoning: String,
    pub hypotheses: Vec<String>,
    pub risks: Vec<String>,
    pub verification_points: Vec<String>,
    pub depth: ReflectionDepth,
    pub verification_level: VerificationLevel,
    pub complexity: ComplexityLevel,
    pub impact: ImpactLevel,
    pub reversibility: Reversibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionDepth {
    Short,
    Normal,
    Deep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationLevel {
    Minimal,
    Standard,
    Reinforced,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                understanding,
                complexity,
                impact,
                reversibility,
            );

        let verification_level =
            Self::choose_verification_level(
                understanding,
                complexity,
                impact,
                reversibility,
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
                verification_level,
            );

        let reasoning = format!(
            "Réflexion {:?}. Complexité {:?}. Impact {:?}. Réversibilité {:?}. Vérification {:?}. {} fait(s), {} incertitude(s), {} hypothèse(s), {} risque(s).",
            depth,
            complexity,
            impact,
            reversibility,
            verification_level,
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
            understanding.uncertainties.len();

        if facts <= 1 && uncertainties == 0 {
            ComplexityLevel::Low
        } else if facts <= 4 && uncertainties <= 2 {
            ComplexityLevel::Medium
        } else if facts <= 8 && uncertainties <= 4 {
            ComplexityLevel::High
        } else {
            ComplexityLevel::VeryHigh
        }
    }

    fn assess_impact(
        understanding: &Understanding,
    ) -> ImpactLevel {
        let uncertainty_count =
            understanding.uncertainties.len();

        let fact_count =
            understanding.facts.len();

        /*
         * À ce niveau du Core, l'impact est estimé
         * à partir des informations disponibles.
         *
         * Les connecteurs réels pourront plus tard
         * fournir des métadonnées explicites :
         * argent, système critique, action publique,
         * irréversibilité, etc.
         */

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
        /*
         * Sans métadonnée d'action explicite,
         * MIDAS reste conservateur dès qu'une
         * situation est fortement incertaine.
         */

        if understanding.uncertainties.len() >= 5 {
            Reversibility::Irreversible
        } else if understanding.uncertainties.len() >= 2 {
            Reversibility::PartiallyReversible
        } else {
            Reversibility::Reversible
        }
    }

    fn choose_depth(
        understanding: &Understanding,
        complexity: ComplexityLevel,
        impact: ImpactLevel,
        reversibility: Reversibility,
    ) -> ReflectionDepth {
        if impact == ImpactLevel::Critical
            || reversibility == Reversibility::Irreversible
            || complexity == ComplexityLevel::VeryHigh
        {
            return ReflectionDepth::Deep;
        }

        if impact == ImpactLevel::High
            || complexity == ComplexityLevel::High
            || reversibility
                == Reversibility::PartiallyReversible
        {
            return ReflectionDepth::Deep;
        }

        if complexity == ComplexityLevel::Medium
            || !understanding.uncertainties.is_empty()
        {
            return ReflectionDepth::Normal;
        }

        ReflectionDepth::Short
    }

    fn choose_verification_level(
        understanding: &Understanding,
        complexity: ComplexityLevel,
        impact: ImpactLevel,
        reversibility: Reversibility,
    ) -> VerificationLevel {
        if impact == ImpactLevel::Critical
            || reversibility == Reversibility::Irreversible
            || complexity == ComplexityLevel::VeryHigh
        {
            return VerificationLevel::Critical;
        }

        if impact == ImpactLevel::High
            || complexity == ComplexityLevel::High
            || reversibility
                == Reversibility::PartiallyReversible
        {
            return VerificationLevel::Reinforced;
        }

        if complexity == ComplexityLevel::Medium
            || !understanding.uncertainties.is_empty()
        {
            return VerificationLevel::Standard;
        }

        VerificationLevel::Minimal
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
                    "Vérification complète des informations critiques."
                        .to_string(),
                );

                points.push(
                    "Vérification indépendante des hypothèses principales."
                        .to_string(),
                );

                points.push(
                    "Évaluation des conséquences et de la réversibilité."
                        .to_string(),
                );

                points.push(
                    "Validation finale avant toute action critique."
                        .to_string(),
                );
            }
        }

        points
    }
}
