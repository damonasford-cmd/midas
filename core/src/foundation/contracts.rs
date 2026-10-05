use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventKind {
    SystemStarted,
    SystemStopping,
    SystemStopped,
    Heartbeat,
    ObservationReceived,
    GoalCreated,
    GoalUpdated,
    TaskCreated,
    TaskUpdated,
    DecisionCreated,
    ActionRequested,
    ActionStarted,
    ActionCompleted,
    ActionFailed,
    CapabilityChanged,
    MemoryUpdated,
    WorldStateChanged,
    Error,
    Warning,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub kind: EventKind,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
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
            kind,
            timestamp: Utc::now(),
            source: source.into(),
            correlation_id: None,
            causation_id: None,
            payload,
        }
    }

    pub fn with_correlation(mut self, id: Uuid) -> Self {
        self.correlation_id = Some(id);
        self
    }

    pub fn with_causation(mut self, id: Uuid) -> Self {
        self.causation_id = Some(id);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GoalStatus {
    Proposed,
    Active,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub objective: String,
    pub status: GoalStatus,
    pub priority: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub parent_id: Option<Uuid>,
    pub metadata: Value,
}

impl Goal {
    pub fn new(objective: impl Into<String>) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            objective: objective.into(),
            status: GoalStatus::Proposed,
            priority: 0.5,
            created_at: now,
            updated_at: now,
            parent_id: None,
            metadata: Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Ready,
    Running,
    Waiting,
    Completed,
    Failed,
    Cancelled,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub goal_id: Uuid,
    pub description: String,
    pub status: TaskStatus,
    pub dependencies: Vec<Uuid>,
    pub priority: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        goal_id: Uuid,
        description: impl Into<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            goal_id,
            description: description.into(),
            status: TaskStatus::Pending,
            dependencies: Vec::new(),
            priority: 0.5,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObservationModality {
    Text,
    Audio,
    Image,
    Video,
    File,
    Web,
    Sensor,
    Software,
    Machine,
    Market,
    Human,
    Environmental,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Uuid,
    pub modality: ObservationModality,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
    pub confidence: f64,
    pub provenance: Option<String>,
}

impl Observation {
    pub fn new(
        modality: ObservationModality,
        source: impl Into<String>,
        content: Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            modality,
            source: source.into(),
            timestamp: Utc::now(),
            content,
            confidence: 1.0,
            provenance: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionDomain {
    Digital,
    Communication,
    Financial,
    Business,
    Research,
    Creation,
    Infrastructure,
    Physical,
    Robotics,
    System,
    Custom(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorizationRequirement {
    None,
    Internal,
    Damon,
    Critical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionStatus {
    Planned,
    Authorized,
    Running,
    Completed,
    Failed,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    pub id: Uuid,
    pub domain: ActionDomain,
    pub objective: String,
    pub target: String,
    pub parameters: Value,
    pub reversibility: f64,
    pub impact: f64,
    pub risk: Risk,
    pub authorization: AuthorizationRequirement,
    pub status: ActionStatus,
    pub created_at: DateTime<Utc>,
}

impl ActionIntent {
    pub fn new(
        domain: ActionDomain,
        objective: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            domain,
            objective: objective.into(),
            target: target.into(),
            parameters: Value::Null,
            reversibility: 1.0,
            impact: 0.0,
            risk: Risk::default(),
            authorization: AuthorizationRequirement::None,
            status: ActionStatus::Planned,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Negligible,
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub level: RiskLevel,
    pub probability: f64,
    pub impact: f64,
    pub reasons: Vec<String>,
    pub mitigations: Vec<String>,
}

impl Default for Risk {
    fn default() -> Self {
        Self {
            level: RiskLevel::Negligible,
            probability: 0.0,
            impact: 0.0,
            reasons: Vec::new(),
            mitigations: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_id: Uuid,
    pub status: ActionStatus,
    pub success: bool,
    pub output: Value,
    pub error: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionStatus {
    Proposed,
    Selected,
    Authorized,
    Rejected,
    Executed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: Uuid,
    pub objective: String,
    pub selected_action: Option<Uuid>,
    pub alternatives: Vec<Uuid>,
    pub confidence: f64,
    pub status: DecisionStatus,
    pub reasoning_summary: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub native: bool,
    pub available: bool,
    pub version: Option<String>,
    pub dependencies: Vec<String>,
    pub metadata: Value,
}

impl Capability {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        native: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            native,
            available: false,
            version: None,
            dependencies: Vec::new(),
            metadata: Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectStatus {
    Concept,
    Planned,
    Active,
    Paused,
    Staging,
    Production,
    Completed,
    Failed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub objective: String,
    pub status: ProjectStatus,
    pub goals: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        name: impl Into<String>,
        objective: impl Into<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            objective: objective.into(),
            status: ProjectStatus::Concept,
            goals: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
