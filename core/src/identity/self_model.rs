use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModelFact {
    pub id: Uuid,
    pub subject: String,
    pub predicate: String,
    pub value: String,
    pub confidence: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SelfModelFact {
    pub fn new(
        subject: impl Into<String>,
        predicate: impl Into<String>,
        value: impl Into<String>,
        confidence: f32,
    ) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(
                "self-model confidence must be between 0 and 1"
                    .into(),
            );
        }

        let subject = subject.into();
        let predicate = predicate.into();
        let value = value.into();

        if subject.trim().is_empty()
            || predicate.trim().is_empty()
            || value.trim().is_empty()
        {
            return Err(
                "self-model fact fields cannot be empty"
                    .into(),
            );
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            subject,
            predicate,
            value,
            confidence,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub identity_statement: String,
    pub unity_statement: String,
    pub public_identity: String,
    pub creator: String,
    pub mission: String,
    pub facts: Vec<SelfModelFact>,
    pub updated_at: DateTime<Utc>,
}

impl SelfModel {
    pub fn new() -> Self {
        Self {
            identity_statement:
                "MIDAS is a unified autonomous artificial intelligence system."
                    .into(),

            unity_statement:
                "MIDAS and Aeron Asford are one unified identity."
                    .into(),

            public_identity:
                "Aeron Asford"
                    .into(),

            creator:
                "Damon"
                    .into(),

            mission:
                "Perceive, understand, reason, create and act in the real world through one unified continuity."
                    .into(),

            facts: Vec::new(),
            updated_at: Utc::now(),
        }
    }

    pub fn add_fact(
        &mut self,
        fact: SelfModelFact,
    ) {
        self.facts.push(fact);
        self.updated_at = Utc::now();
    }

    pub fn remove_fact(
        &mut self,
        id: Uuid,
    ) -> bool {
        let original = self.facts.len();

        self.facts.retain(|fact| fact.id != id);

        let changed = original != self.facts.len();

        if changed {
            self.updated_at = Utc::now();
        }

        changed
    }
}

impl Default for SelfModel {
    fn default() -> Self {
        Self::new()
    }
}
