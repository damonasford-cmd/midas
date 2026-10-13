
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentKind {
    SoftwareTest,
    Simulation,
    DataAnalysis,
    LaboratoryTest,
    PhysicalPrototype,
    FieldStudy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentStatus {
    Proposed,
    AwaitingAuthorization,
    Ready,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentPlan {
    pub id: Uuid,
    pub objective_id: Uuid,
    pub hypothesis_id: Option<Uuid>,
    pub title: String,
    pub question: String,
    pub kind: ExperimentKind,
    pub procedure: Vec<String>,
    pub required_resources: Vec<String>,
    pub success_criteria: Vec<String>,
    pub risks: Vec<String>,
    pub requires_human_authorization: bool,
    pub status: ExperimentStatus,
    pub proposed_at: DateTime<Utc>,
    pub metadata: Value,
}

impl ExperimentPlan {
    pub fn new(
        objective_id: Uuid,
        title: impl Into<String>,
        question: impl Into<String>,
        kind: ExperimentKind,
    ) -> anyhow::Result<Self> {
        let title = title.into().trim().to_owned();
        let question = question.into().trim().to_owned();

        anyhow::ensure!(!title.is_empty(), "Titre de l'expérience obligatoire");
        anyhow::ensure!(!question.is_empty(), "Question de l'expérience obligatoire");

        Ok(Self {
            id: Uuid::new_v4(),
            objective_id,
            hypothesis_id: None,
            title,
            question,
            kind,
            procedure: Vec::new(),
            required_resources: Vec::new(),
            success_criteria: Vec::new(),
            risks: Vec::new(),
            requires_human_authorization: matches!(
                kind,
                ExperimentKind::LaboratoryTest
                    | ExperimentKind::PhysicalPrototype
                    | ExperimentKind::FieldStudy
            ),
            status: ExperimentStatus::Proposed,
            proposed_at: Utc::now(),
            metadata: serde_json::json!({}),
        })
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.title.trim().is_empty(), "Titre vide");
        anyhow::ensure!(!self.question.trim().is_empty(), "Question vide");
        anyhow::ensure!(!self.procedure.is_empty(), "Procédure expérimentale manquante");
        anyhow::ensure!(!self.success_criteria.is_empty(), "Critères de réussite manquants");
        Ok(())
    }

    pub fn mark_ready(&mut self, authorization_confirmed: bool) -> anyhow::Result<()> {
        self.validate()?;

        if self.requires_human_authorization && !authorization_confirmed {
            self.status = ExperimentStatus::AwaitingAuthorization;
            anyhow::bail!("Autorisation humaine requise avant l'expérience");
        }

        self.status = ExperimentStatus::Ready;
        Ok(())
    }
}
