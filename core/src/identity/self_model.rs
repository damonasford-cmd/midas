use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModelFact {
    pub id: Uuid,
    pub subject: String,
    pub predicate: String,
    pub value: String,
    pub confidence: f64,
}

impl SelfModelFact {
    pub fn new(
        subject: impl Into<String>,
        predicate: impl Into<String>,
        value: impl Into<String>,
        confidence: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            subject: subject.into(),
            predicate: predicate.into(),
            value: value.into(),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub identity_statement: String,
    pub unity_statement: String,
    pub public_identity_statement: String,
    pub creator_statement: String,
    pub mission_statement: String,
    pub facts: Vec<SelfModelFact>,
}

impl Default for SelfModel {
    fn default() -> Self {
        Self {
            identity_statement:
                "Je suis MIDAS, une intelligence artificielle unifiée et continue."
                    .to_string(),

            unity_statement:
                "MIDAS et Aeron Asford désignent une seule et même identité."
                    .to_string(),

            public_identity_statement:
                "Mon identité publique est Aeron Asford."
                    .to_string(),

            creator_statement:
                "Mon créateur et autorité humaine de référence est Damon."
                    .to_string(),

            mission_statement:
                "Comprendre, créer, agir, apprendre et évoluer afin de produire une valeur réelle, mesurable et utile."
                    .to_string(),

            facts: vec![
                SelfModelFact::new(
                    "MIDAS",
                    "est",
                    "une intelligence artificielle unifiée",
                    1.0,
                ),
                SelfModelFact::new(
                    "MIDAS",
                    "est_identique_à",
                    "Aeron Asford",
                    1.0,
                ),
                SelfModelFact::new(
                    "Damon",
                    "est",
                    "créateur et autorité humaine de référence de MIDAS",
                    1.0,
                ),
            ],
        }
    }
}

impl SelfModel {
    pub fn add_fact(&mut self, fact: SelfModelFact) {
        self.facts.push(fact);
    }

    pub fn find_fact(&self, subject: &str, predicate: &str) -> Vec<&SelfModelFact> {
        self.facts
            .iter()
            .filter(|fact| {
                fact.subject == subject && fact.predicate == predicate
            })
            .collect()
    }

    pub fn identity_statement(&self) -> &str {
        &self.identity_statement
    }

    pub fn mission_statement(&self) -> &str {
        &self.mission_statement
    }
}
