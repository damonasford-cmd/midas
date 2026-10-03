use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub kind: EventKind,
    pub source: String,
    pub payload: Value,
}

impl Event {
    pub fn new(
        kind: EventKind,
        source: impl Into<String>,
        payload: Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            kind,
            source: source.into(),
            payload,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    SystemStarted,
    SystemStopped,
    Heartbeat,

    UserMessage,
    ExternalMessage,

    WorldObservation,
    MarketObservation,
    MediaObservation,
    GeopoliticalObservation,
    PhysicalObservation,

    GoalCreated,
    TaskCreated,
    TaskUpdated,

    CapabilityDetected,
    CapabilityMissing,
    CapabilityIntegrated,

    ResearchStarted,
    ResearchCompleted,

    ExperimentStarted,
    ExperimentCompleted,

    DecisionCreated,
    ActionRequested,
    ActionAuthorized,
    ActionRejected,
    ActionExecuted,
    ActionFailed,

    ErrorDetected,
    CorrectionStarted,
    CorrectionCompleted,

    LearningRecorded,
    EvolutionProposed,
    EvolutionValidated,
    EvolutionDeployed,

    Alert,
    ShutdownRequested,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub priority: f64,
    pub active: bool,
    pub metadata: Value,
}

impl Goal {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            title: title.into(),
            description: description.into(),
            priority: 0.5,
            active: true,
            metadata: Value::Object(Default::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub goal_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub priority: f64,
    pub status: TaskStatus,
    pub dependencies: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Planning,
    Running,
    Waiting,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub modality: ObservationModality,
    pub content: Value,
    pub confidence: f64,
    pub provenance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObservationModality {
    Text,
    Image,
    Audio,
    Video,
    Sensor,
    Web,
    Market,
    File,
    Software,
    Physical,
    Multimodal,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub objective: String,
    pub selected_action: Option<Uuid>,
    pub alternatives: Vec<String>,
    pub reasoning_summary: String,
    pub confidence: f64,
    pub requires_authorization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub domain: ActionDomain,
    pub description: String,
    pub parameters: Value,
    pub reversibility: ActionReversibility,
    pub impact: ActionImpact,
    pub risk: Risk,
    pub authorization: AuthorizationRequirement,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionDomain {
    Internal,
    Information,
    Software,
    Infrastructure,
    Communication,
    Business,
    Finance,
    Market,
    Physical,
    Robotics,
    Research,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionReversibility {
    FullyReversible,
    PartiallyReversible,
    Irreversible,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionImpact {
    Negligible,
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationRequirement {
    None,
    SystemPolicy,
    UserConfirmation,
    MultiFactor,
    ExternalAuthorization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub probability: f64,
    pub severity: f64,
    pub rationale: String,
}

impl Risk {
    pub fn score(&self) -> f64 {
        self.probability.clamp(0.0, 1.0)
            * self.severity.clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub success: bool,
    pub output: Value,
    pub error: Option<String>,
    pub observations: Vec<Observation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub available: bool,
    pub version: Option<String>,
    pub dependencies: Vec<String>,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub goals: Vec<Uuid>,
    pub resources: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Proposed,
    Researching,
    Building,
    Testing,
    Staging,
    Production,
    Paused,
    Completed,
    Archived,
}
