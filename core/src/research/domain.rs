
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchObjective {
    pub id: Uuid,
    pub title: String,
    pub desired_outcome: String,
    pub context: String,
    pub constraints: Vec<String>,
    pub questions: Vec<String>,
    pub metadata: Value,
}

impl ResearchObjective {
    pub fn new(
        title: impl Into<String>,
        desired_outcome: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let title = title.into().trim().to_owned();
        let desired_outcome = desired_outcome.into().trim().to_owned();

        anyhow::ensure!(!title.is_empty(), "L'objectif doit avoir un titre");
        anyhow::ensure!(!desired_outcome.is_empty(), "Le résultat attendu est obligatoire");

        Ok(Self {
            id: Uuid::new_v4(),
            title,
            desired_outcome,
            context: String::new(),
            constraints: Vec::new(),
            questions: Vec::new(),
            metadata: serde_json::json!({}),
        })
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.title.trim().is_empty(), "Titre vide");
        anyhow::ensure!(!self.desired_outcome.trim().is_empty(), "Résultat attendu vide");
        anyhow::ensure!(self.title.len() <= 2_000, "Titre trop long");
        anyhow::ensure!(self.desired_outcome.len() <= 20_000, "Résultat attendu trop long");
        anyhow::ensure!(self.context.len() <= 100_000, "Contexte trop long");
        anyhow::ensure!(self.questions.len() <= 500, "Trop de questions");
        anyhow::ensure!(self.constraints.len() <= 500, "Trop de contraintes");
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResearchStatus {
    Planned,
    Running,
    Completed,
    PartiallyCompleted,
    Blocked,
    Failed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Blocked,
    Skipped,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ResearchMethodKind {
    WebSearch,
    DocumentAnalysis,
    DataAnalysis,
    CodeInspection,
    Experiment,
    Simulation,
    DomainExpert,
    InternalKnowledge,
    GeneralAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTask {
    pub id: Uuid,
    pub objective_id: Uuid,
    pub title: String,
    pub question: String,
    pub method: ResearchMethodKind,
    pub priority: u8,
    pub depends_on: Vec<Uuid>,
    pub status: TaskStatus,
    pub result_summary: Option<String>,
    pub error: Option<String>,
}

impl ResearchTask {
    pub fn new(
        objective_id: Uuid,
        title: impl Into<String>,
        question: impl Into<String>,
        method: ResearchMethodKind,
        priority: u8,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            objective_id,
            title: title.into(),
            question: question.into(),
            method,
            priority: priority.clamp(1, 10),
            depends_on: Vec::new(),
            status: TaskStatus::Pending,
            result_summary: None,
            error: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchRun {
    pub id: Uuid,
    pub objective: ResearchObjective,
    pub status: ResearchStatus,
    pub tasks: Vec<ResearchTask>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub warnings: Vec<String>,
    pub metadata: Value,
}

impl ResearchRun {
    pub fn new(objective: ResearchObjective, tasks: Vec<ResearchTask>) -> Self {
        Self {
            id: Uuid::new_v4(),
            objective,
            status: ResearchStatus::Planned,
            tasks,
            started_at: Utc::now(),
            completed_at: None,
            warnings: Vec::new(),
            metadata: serde_json::json!({}),
        }
    }
}
