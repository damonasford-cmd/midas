use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ForgePlane {
    Internal,
    External,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ForgeDomain {
    Software,
    Hardware,
    Robotics,
    Materials,
    ScientificResearch,
    Manufacturing,
    Infrastructure,
    Energy,
    Finance,
    Biology,
    Communication,
    Organization,
    DigitalSystems,
    PhysicalSystems,
    General,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InitiativeOrigin {
    User,
    MidasInitiative,
    ResearchDiscovery,
    ExistingProject,
    ExternalRequest,
    Recovery,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ForgeJobStatus {
    Proposed,
    Planned,
    AwaitingAuthorization,
    Ready,
    Running,
    Blocked,
    Failed,
    Completed,
    Cancelled,
    Superseded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ForgeStepStatus {
    Pending,
    Ready,
    Running,
    Blocked,
    Failed,
    Succeeded,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ForgeRisk {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ForgeActionKind {
    Research,
    Design,
    Create,
    Modify,
    Build,
    Acquire,
    Manufacture,
    Experiment,
    Measure,
    Test,
    Repair,
    Optimize,
    Integrate,
    Deploy,
    Monitor,
    Rollback,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    SourceReference,
    StaticAnalysis,
    Compilation,
    AutomatedTest,
    Measurement,
    PhysicalInspection,
    LaboratoryResult,
    DeploymentHealth,
    HumanConfirmation,
    Simulation,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RealityLevel {
    Proposed,
    Designed,
    Simulated,
    Built,
    Measured,
    IndependentlyVerified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceCriterion {
    pub id: Uuid,
    pub description: String,
    pub required: bool,
    pub measurement: Option<String>,
    pub target: Option<Value>,
}

impl AcceptanceCriterion {
    pub fn new(description: impl Into<String>, required: bool) -> Result<Self> {
        let description = description.into();
        if description.trim().is_empty() {
            bail!("Acceptance criterion cannot be empty");
        }

        Ok(Self {
            id: Uuid::new_v4(),
            description,
            required,
            measurement: None,
            target: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeObjective {
    pub id: Uuid,
    pub title: String,
    pub desired_outcome: String,
    pub plane: ForgePlane,
    pub domains: Vec<ForgeDomain>,
    pub origin: InitiativeOrigin,
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
    pub constraints: Value,
    pub context: Value,
    pub created_at: DateTime<Utc>,
}

impl ForgeObjective {
    pub fn validate(&self) -> Result<()> {
        if self.title.trim().is_empty() {
            bail!("Forge objective title is required");
        }
        if self.desired_outcome.trim().is_empty() {
            bail!("Forge desired outcome is required");
        }
        if self.acceptance_criteria.is_empty() {
            bail!("At least one acceptance criterion is required");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeAction {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub kind: ForgeActionKind,
    pub plane: ForgePlane,
    pub domains: Vec<ForgeDomain>,
    pub operation: String,
    pub inputs: Value,
    pub expected_outputs: Vec<String>,
    pub required_permissions: Vec<String>,
    pub risk: ForgeRisk,
    pub requires_human_authorization: bool,
    pub metadata: Value,
}

impl ForgeAction {
    pub fn validate(&self) -> Result<()> {
        if self.title.trim().is_empty() {
            bail!("Forge action title is required");
        }
        if self.operation.trim().is_empty() {
            bail!("Forge operation identifier is required");
        }
        if self.expected_outputs.is_empty() {
            bail!("Forge action must declare expected outputs");
        }
        if self.required_permissions.iter().any(|p| p.trim().is_empty()) {
            bail!("Forge action contains an empty permission");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeStep {
    pub id: Uuid,
    pub action: ForgeAction,
    pub dependencies: Vec<Uuid>,
    pub status: ForgeStepStatus,
    pub attempts: u32,
    pub output_summary: Option<String>,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl ForgeStep {
    pub fn new(action: ForgeAction) -> Self {
        Self {
            id: Uuid::new_v4(),
            action,
            dependencies: Vec::new(),
            status: ForgeStepStatus::Pending,
            attempts: 0,
            output_summary: None,
            error: None,
            started_at: None,
            completed_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgePlan {
    pub id: Uuid,
    pub version: u32,
    pub objective: ForgeObjective,
    pub steps: Vec<ForgeStep>,
    pub created_at: DateTime<Utc>,
    pub metadata: Value,
}

impl ForgePlan {
    pub fn new(objective: ForgeObjective, steps: Vec<ForgeStep>) -> Self {
        Self {
            id: Uuid::new_v4(),
            version: 1,
            objective,
            steps,
            created_at: Utc::now(),
            metadata: Value::Null,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.objective.validate()?;

        if self.steps.is_empty() {
            bail!("Forge plan cannot be empty");
        }

        let mut ids = HashSet::new();
        for step in &self.steps {
            step.action.validate()?;

            if !ids.insert(step.id) {
                bail!("Duplicate Forge step ID");
            }
            if step.dependencies.contains(&step.id) {
                bail!("A Forge step cannot depend on itself");
            }
        }

        let by_id: HashMap<Uuid, &ForgeStep> =
            self.steps.iter().map(|s| (s.id, s)).collect();

        for step in &self.steps {
            for dependency in &step.dependencies {
                if !by_id.contains_key(dependency) {
                    bail!("Forge plan references an unknown dependency");
                }
            }
        }

        fn visit(
            id: Uuid,
            steps: &HashMap<Uuid, &ForgeStep>,
            visiting: &mut HashSet<Uuid>,
            visited: &mut HashSet<Uuid>,
        ) -> Result<()> {
            if visited.contains(&id) {
                return Ok(());
            }
            if !visiting.insert(id) {
                bail!("Forge plan contains a dependency cycle");
            }

            let step = steps
                .get(&id)
                .ok_or_else(|| anyhow::anyhow!("Unknown Forge step"))?;

            for dep in &step.dependencies {
                visit(*dep, steps, visiting, visited)?;
            }

            visiting.remove(&id);
            visited.insert(id);
            Ok(())
        }

        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        for step in &self.steps {
            visit(step.id, &by_id, &mut visiting, &mut visited)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeJob {
    pub id: Uuid,
    pub plan: ForgePlan,
    pub status: ForgeJobStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl ForgeJob {
    pub fn new(plan: ForgePlan) -> Result<Self> {
        plan.validate()?;
        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            plan,
            status: ForgeJobStatus::Planned,
            created_at: now,
            updated_at: now,
            completed_at: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeArtifact {
    pub id: Uuid,
    pub job_id: Uuid,
    pub step_id: Uuid,
    pub name: String,
    pub description: String,
    pub uri: String,
    pub domains: Vec<ForgeDomain>,
    pub plane: ForgePlane,
    pub reality: RealityLevel,
    pub checksum: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeEvidence {
    pub id: Uuid,
    pub kind: EvidenceKind,
    pub description: String,
    pub source: String,
    pub observed_at: DateTime<Utc>,
    pub independently_verified: bool,
    pub measured_values: Value,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeStepResult {
    pub success: bool,
    pub summary: String,
    pub artifacts: Vec<ForgeArtifact>,
    pub evidence: Vec<ForgeEvidence>,
    pub unresolved_issues: Vec<String>,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeStepReport {
    pub step_id: Uuid,
    pub status: ForgeStepStatus,
    pub summary: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeReport {
    pub job_id: Uuid,
    pub status: ForgeJobStatus,
    pub steps: Vec<ForgeStepReport>,
    pub artifacts: Vec<ForgeArtifact>,
    pub evidence: Vec<ForgeEvidence>,
    pub unresolved_issues: Vec<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}
