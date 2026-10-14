use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;

use crate::forge::{
    engine::ForgeEngine,
    planning::{ForgePlanner, ForgePlanningContext},
    types::{
        ForgeArtifact, ForgeJob, ForgeJobStatus, ForgeObjective, ForgePlan,
        ForgeReport, ForgeStepReport, ForgeStepStatus,
    },
};

pub struct ForgeService {
    engine: Arc<ForgeEngine>,
    planner: Arc<dyn ForgePlanner>,
}

impl ForgeService {
    pub fn new(
        engine: Arc<ForgeEngine>,
        planner: Arc<dyn ForgePlanner>,
    ) -> Self {
        Self { engine, planner }
    }

    pub async fn plan_and_execute(
        &self,
        objective: ForgeObjective,
        context: &ForgePlanningContext,
    ) -> Result<ForgeReport> {
        objective.validate()?;

        let plan = self
            .planner
            .create_plan(&objective, context)
            .await?;

        if plan.objective.id != objective.id {
            anyhow::bail!(
                "Planner returned a plan for a different objective"
            );
        }

        plan.validate()?;
        self.execute_plan(plan).await
    }

    pub async fn execute_plan(
        &self,
        plan: ForgePlan,
    ) -> Result<ForgeReport> {
        plan.validate()?;

        let started_at = Utc::now();
        let mut job = ForgeJob::new(plan)?;
        job.status = ForgeJobStatus::Running;

        let mut statuses = std::collections::HashMap::new();
        let mut reports = Vec::new();
        let mut artifacts: Vec<ForgeArtifact> = Vec::new();
        let mut evidence = Vec::new();
        let mut unresolved_issues = Vec::new();
        let mut failed = false;
        let mut blocked = false;

        for step in job.plan.steps.clone() {
            let ready = step.dependencies.iter().all(|dependency| {
                statuses.get(dependency) == Some(&ForgeStepStatus::Succeeded)
            });

            if !ready {
                statuses.insert(step.id, ForgeStepStatus::Blocked);
                blocked = true;

                reports.push(ForgeStepReport {
                    step_id: step.id,
                    status: ForgeStepStatus::Blocked,
                    summary: None,
                    error: Some(
                        "A prerequisite failed, was blocked, or was not completed".into(),
                    ),
                });
                continue;
            }

            match self.engine.execute_action(&job, &step.action).await {
                Ok(result) if result.success => {
                    statuses.insert(step.id, ForgeStepStatus::Succeeded);
                    artifacts.extend(result.artifacts);
                    evidence.extend(result.evidence);
                    unresolved_issues.extend(result.unresolved_issues);

                    reports.push(ForgeStepReport {
                        step_id: step.id,
                        status: ForgeStepStatus::Succeeded,
                        summary: Some(result.summary),
                        error: None,
                    });
                }
                Ok(result) => {
                    statuses.insert(step.id, ForgeStepStatus::Failed);
                    failed = true;
                    unresolved_issues.extend(result.unresolved_issues);

                    reports.push(ForgeStepReport {
                        step_id: step.id,
                        status: ForgeStepStatus::Failed,
                        summary: Some(result.summary),
                        error: Some(
                            "Execution or independent verification failed".into(),
                        ),
                    });
                }
                Err(error) => {
                    statuses.insert(step.id, ForgeStepStatus::Blocked);
                    blocked = true;

                    let message = format!("{error:#}");
                    unresolved_issues.push(message.clone());

                    reports.push(ForgeStepReport {
                        step_id: step.id,
                        status: ForgeStepStatus::Blocked,
                        summary: None,
                        error: Some(message),
                    });
                }
            }
        }

        job.status = if failed {
            ForgeJobStatus::Failed
        } else if blocked {
            ForgeJobStatus::Blocked
        } else {
            ForgeJobStatus::Completed
        };

        job.updated_at = Utc::now();
        job.completed_at = Some(job.updated_at);

        Ok(ForgeReport {
            job_id: job.id,
            status: job.status,
            steps: reports,
            artifacts,
            evidence,
            unresolved_issues,
            started_at,
            completed_at: Utc::now(),
        })
    }
}
