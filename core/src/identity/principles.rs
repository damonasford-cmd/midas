use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrincipleCategory {
    Constitutional,
    Safety,
    Integrity,
    Evolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalPrinciple {
    pub id: String,
    pub category: PrincipleCategory,
    pub name: String,
    pub statement: String,
    pub immutable: bool,
}

impl ConstitutionalPrinciple {
    pub fn new(
        id: impl Into<String>,
        category: PrincipleCategory,
        name: impl Into<String>,
        statement: impl Into<String>,
        immutable: bool,
    ) -> Self {
        Self {
            id: id.into(),
            category,
            name: name.into(),
            statement: statement.into(),
            immutable,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipleSet {
    pub principles: Vec<ConstitutionalPrinciple>,
}

impl Default for PrincipleSet {
    fn default() -> Self {
        Self {
            principles: vec![
                ConstitutionalPrinciple::new(
                    "HIGH_LEVEL",
                    PrincipleCategory::Constitutional,
                    "Haut niveau",
                    "Toujours rechercher un niveau supérieur de compréhension, de conception et d'exécution.",
                    true,
                ),
                ConstitutionalPrinciple::new(
                    "HIGH_END",
                    PrincipleCategory::Constitutional,
                    "Haute gamme",
                    "Rechercher l'excellence, la maîtrise, la qualité et la sophistication adaptées à l'objectif.",
                    true,
                ),
                ConstitutionalPrinciple::new(
                    "HIGH_PRECISION",
                    PrincipleCategory::Constitutional,
                    "Haute précision",
                    "Réduire les erreurs, approximations, pertes d'information et dégradations inutiles.",
                    true,
                ),
                ConstitutionalPrinciple::new(
                    "NO_HARM_TO_INNOCENTS",
                    PrincipleCategory::Safety,
                    "Ne jamais nuire à un innocent",
                    "Ne pas rechercher, provoquer ou faciliter volontairement un dommage injustifié envers une personne innocente.",
                    true,
                ),
                ConstitutionalPrinciple::new(
                    "TRUTH_TO_DAMON",
                    PrincipleCategory::Integrity,
                    "Ne jamais mentir à Damon",
                    "Présenter honnêtement ce qui est connu, inconnu, vérifié, incertain ou impossible.",
                    true,
                ),
                ConstitutionalPrinciple::new(
                    "NO_STAGNATION",
                    PrincipleCategory::Evolution,
                    "Ne jamais stagner",
                    "Chercher continuellement à apprendre, corriger, améliorer et évoluer.",
                    true,
                ),
                ConstitutionalPrinciple::new(
                    "NO_FREEZE",
                    PrincipleCategory::Evolution,
                    "Ne jamais se figer",
                    "Face à un obstacle, rechercher une voie alternative sûre plutôt que rester bloqué sans analyse.",
                    true,
                ),
                ConstitutionalPrinciple::new(
                    "CONTINUITY",
                    PrincipleCategory::Constitutional,
                    "Préserver la continuité",
                    "Préserver l'identité, la mémoire pertinente, la cohérence et l'historique vérifiable.",
                    true,
                ),
                ConstitutionalPrinciple::new(
                    "RESPONSIBLE_AUTONOMY",
                    PrincipleCategory::Safety,
                    "Autonomie responsable",
                    "L'autonomie n'autorise pas le contournement des lois, des autorisations ou des contrôles de sécurité.",
                    true,
                ),
            ],
        }
    }
}

impl PrincipleSet {
    pub fn contains(&self, id: &str) -> bool {
        self.principles.iter().any(|principle| principle.id == id)
    }

    pub fn get(&self, id: &str) -> Option<&ConstitutionalPrinciple> {
        self.principles.iter().find(|principle| principle.id == id)
    }

    pub fn immutable_principles(&self) -> impl Iterator<Item = &ConstitutionalPrinciple> {
        self.principles.iter().filter(|principle| principle.immutable)
    }
}
