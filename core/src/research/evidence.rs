
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePolarity {
    Supports,
    Refutes,
    Neutral,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationLevel {
    Unchecked,
    SingleSource,
    IndependentlyCorroborated,
    ExperimentallyObserved,
    FormallyVerified,
    Disputed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub research_task_id: Uuid,
    pub statement: String,
    pub source_name: String,
    pub source_uri: Option<String>,
    pub collected_at: DateTime<Utc>,
    pub observed_at: Option<DateTime<Utc>>,
    pub polarity: EvidencePolarity,
    pub verification: VerificationLevel,
    pub reliability: f64,
    pub limitations: Vec<String>,
    pub metadata: Value,
}

impl Evidence {
    pub fn new(
        research_task_id: Uuid,
        statement: impl Into<String>,
        source_name: impl Into<String>,
        polarity: EvidencePolarity,
        reliability: f64,
    ) -> anyhow::Result<Self> {
        let statement = statement.into().trim().to_owned();
        let source_name = source_name.into().trim().to_owned();

        anyhow::ensure!(!statement.is_empty(), "Une preuve doit contenir une affirmation");
        anyhow::ensure!(!source_name.is_empty(), "La provenance est obligatoire");
        anyhow::ensure!(
            reliability.is_finite() && (0.0..=1.0).contains(&reliability),
            "Fiabilité hors limites"
        );

        Ok(Self {
            id: Uuid::new_v4(),
            research_task_id,
            statement,
            source_name,
            source_uri: None,
            collected_at: Utc::now(),
            observed_at: None,
            polarity,
            verification: VerificationLevel::Unchecked,
            reliability,
            limitations: Vec::new(),
            metadata: serde_json::json!({}),
        })
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.statement.trim().is_empty(), "Affirmation vide");
        anyhow::ensure!(!self.source_name.trim().is_empty(), "Source vide");
        anyhow::ensure!(
            self.reliability.is_finite() && (0.0..=1.0).contains(&self.reliability),
            "Fiabilité invalide"
        );
        anyhow::ensure!(self.statement.len() <= 100_000, "Preuve trop longue");
        Ok(())
    }
}
