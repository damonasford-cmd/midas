use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum EventKind {
    SystemStarted,
    SystemStopping,
    SystemStopped,
    Heartbeat,
    GoalCreated,
    GoalUpdated,
    TaskCreated,
    TaskUpdated,
    ObservationReceived,
    DecisionCreated,
    ActionRequested,
    ActionCompleted,
    ActionFailed,
    ResourceChanged,
    Error,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub kind: EventKind,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub source: String,
    pub payload: serde_json::Value,
}

impl Event {
    pub fn new(
        kind: EventKind,
        source: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            timestamp: Utc::now(),
            correlation_id: None,
            causation_id: None,
            source: source.into(),
            payload,
        }
    }

    pub fn with_correlation(
        mut self,
        correlation_id: Uuid,
    ) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_causation(
        mut self,
        causation_id: Uuid,
    ) -> Self {
        self.causation_id = Some(causation_id);
        self
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum GoalStatus {
    Created,
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
    pub priority: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Goal {
    pub fn new(
        objective: impl Into<String>,
        priority: u32,
    ) -> Result<Self, String> {
        let objective = objective.into();

        if objective.trim().is_empty() {
            return Err("goal objective cannot be empty".into());
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            objective,
            status: GoalStatus::Created,
            priority,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum TaskStatus {
    Created,
    Queued,
    Running,
    Blocked,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub goal_id: Uuid,
    pub description: String,
    pub status: TaskStatus,
    pub priority: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        goal_id: Uuid,
        description: impl Into<String>,
        priority: u32,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err("task description cannot be empty".into());
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            goal_id,
            description,
            status: TaskStatus::Created,
            priority,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ObservationModality {
    Text,
    StructuredData,
    Image,
    Video,
    Audio,
    Voice,
    File,
    Web,
    Software,
    Screen,
    Sensor,
    Machine,
    Device,
    Network,
    HumanInteraction,
    Environment,
    Location,
    Event,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Uuid,
    pub modality: ObservationModality,
    pub content: serde_json::Value,
    pub observed_at: DateTime<Utc>,
    pub confidence: f32,
    pub source: Option<String>,
}

impl Observation {
    pub fn new(
        modality: ObservationModality,
        content: serde_json::Value,
        confidence: f32,
        source: Option<String>,
    ) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(
                "observation confidence must be between 0 and 1"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            modality,
            content,
            observed_at: Utc::now(),
            confidence,
            source,
        })
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ActionDomain {
    Digital,
    Communication,
    Financial,
    Physical,
    Infrastructure,
    Software,
    ExternalSystem,
    Other,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum AuthorizationRequirement {
    None,
    User,
    System,
    Elevated,
    Critical,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ActionStatus {
    Planned,
    Authorized,
    Executing,
    Completed,
    Failed,
    Cancelled,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    pub id: Uuid,
    pub domain: ActionDomain,
    pub description: String,
    pub authorization: AuthorizationRequirement,
    pub status: ActionStatus,
    pub created_at: DateTime<Utc>,
}

impl ActionIntent {
    pub fn new(
        domain: ActionDomain,
        description: impl Into<String>,
        authorization: AuthorizationRequirement,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                "action description cannot be empty".into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            domain,
            description,
            authorization,
            status: ActionStatus::Planned,
            created_at: Utc::now(),
        })
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum RiskLevel {
    None,
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub level: RiskLevel,
    pub description: String,
    pub mitigations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_id: Uuid,
    pub status: ActionStatus,
    pub success: bool,
    pub output: serde_json::Value,
    pub error: Option<String>,
    pub completed_at: DateTime<Utc>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum DecisionStatus {
    Proposed,
    Verified,
    Authorized,
    Rejected,
    Executing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: Uuid,
    pub objective: String,
    pub status: DecisionStatus,
    pub rationale: String,
    pub confidence: f32,
    pub risks: Vec<Risk>,
    pub actions: Vec<ActionIntent>,
    pub created_at: DateTime<Utc>,
}

impl Decision {
    pub fn new(
        objective: impl Into<String>,
        rationale: impl Into<String>,
        confidence: f32,
    ) -> Result<Self, String> {
        let objective = objective.into();
        let rationale = rationale.into();

        if objective.trim().is_empty() {
            return Err("decision objective cannot be empty".into());
        }

        if rationale.trim().is_empty() {
            return Err("decision rationale cannot be empty".into());
        }

        if !(0.0..=1.0).contains(&confidence) {
            return Err(
                "decision confidence must be between 0 and 1"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            objective,
            status: DecisionStatus::Proposed,
            rationale,
            confidence,
            risks: Vec::new(),
            actions: Vec::new(),
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub available: bool,
    pub version: Option<String>,
    pub provider: Option<String>,
}

impl Capability {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let name = name.into();
        let description = description.into();

        if name.trim().is_empty() {
            return Err("capability name cannot be empty".into());
        }

        if description.trim().is_empty() {
            return Err(
                "capability description cannot be empty".into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            available: false,
            version: None,
            provider: None,
        })
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ProjectStatus {
    Planned,
    Active,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub objective: String,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        name: impl Into<String>,
        objective: impl Into<String>,
    ) -> Result<Self, String> {
        let name = name.into();
        let objective = objective.into();

        if name.trim().is_empty() {
            return Err("project name cannot be empty".into());
        }

        if objective.trim().is_empty() {
            return Err(
                "project objective cannot be empty".into()
            );
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            objective,
            status: ProjectStatus::Planned,
            created_at: now,
            updated_at: now,
        })
    }
}
