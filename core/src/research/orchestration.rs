
use anyhow::{Context, Result};

use crate::research::{
    discovery::{create_report, ResearchReport},
    domain::{
        ResearchObjective, ResearchRun, ResearchStatus, TaskStatus,
    },
    evidence::Evidence,
    methods::ResearchMethodRegistry,
    objectives::decompose_objective,
    repository::ResearchRepository,
};

pub struct ResearchEngine {
    registry: ResearchMethodRegistry,
    repository: ResearchRepository,
}

impl ResearchEngine {
    pub fn new(
        registry: ResearchMethodRegistry,
        repository: ResearchRepository,
    ) -> Self {
        Self {
            registry,
            repository,
        }
    }

    pub fn available_methods(
        &self,
    ) -> Vec<crate::research::methods::MethodDescriptor> {
        self.registry.descriptors()
    }

    pub async fn investigate(
        &self,
        objective: ResearchObjective,
    ) -> Result<ResearchReport> {
        objective.validate()?;

        let tasks = decompose_objective(&objective)?;
        let mut run = ResearchRun::new(objective, tasks);
        run.status = ResearchStatus::Running;

        let mut evidence: Vec<Evidence> = Vec::new();

        let mut order: Vec<usize> = (0..run.tasks.len()).collect();
        order.sort_by_key(|index| {
            std::cmp::Reverse(run.tasks[*index].priority)
        });

        for index in order {
            let task_snapshot = run.tasks[index].clone();

            let dependencies_satisfied = task_snapshot.depends_on.iter().all(|dependency| {
                run.tasks.iter().any(|candidate| {
                    candidate.id == *dependency
                        && candidate.status == TaskStatus::Completed
                })
            });

            if !dependencies_satisfied {
                run.tasks[index].status = TaskStatus::Blocked;
                run.tasks[index].error =
                    Some("Une ou plusieurs tâches préalables ne sont pas terminées.".to_owned());
                continue;
            }

            run.tasks[index].status = TaskStatus::Running;

            let methods = self.registry.available_for(task_snapshot.method);

            if methods.is_empty() {
                run.tasks[index].status = TaskStatus::Blocked;
                run.tasks[index].error = Some(format!(
                    "Aucune méthode active pour {:?}. Il faut connecter la capacité correspondante.",
                    task_snapshot.method
                ));

                run.warnings.push(format!(
                    "Tâche bloquée : {} — aucune méthode compatible.",
                    task_snapshot.title
                ));
                continue;
            }

            let mut task_evidence = Vec::new();
            let mut summaries = Vec::new();
            let mut failures = Vec::new();

            for method in methods {
                let descriptor = method.descriptor();

                match method
                    .investigate(&run.objective, &task_snapshot)
                    .await
                {
                    Ok(result) => {
                        summaries.push(format!("{} : {}", descriptor.name, result.summary));
                        run.warnings.extend(result.warnings);

                        for item in result.evidence {
                            if let Err(error) = item.validate() {
                                failures.push(format!(
                                    "{} a produit une preuve invalide : {}",
                                    descriptor.name, error
                                ));
                                continue;
                            }

                            if item.research_task_id != task_snapshot.id {
                                failures.push(format!(
                                    "{} a produit une preuve rattachée à une autre tâche.",
                                    descriptor.name
                                ));
                                continue;
                            }

                            task_evidence.push(item);
                        }
                    }
                    Err(error) => {
                        failures.push(format!(
                            "{} : {}",
                            descriptor.name,
                            error
                        ));
                    }
                }
            }

            if !task_evidence.is_empty() || !summaries.is_empty() {
                run.tasks[index].status = TaskStatus::Completed;
                run.tasks[index].result_summary = Some(summaries.join(" | "));
                evidence.extend(task_evidence);
            } else {
                run.tasks[index].status = TaskStatus::Failed;
                run.tasks[index].error = Some(failures.join(" | "));
                run.warnings.push(format!(
                    "Aucune méthode n'a réussi la tâche : {}.",
                    task_snapshot.title
                ));
            }
        }

        let completed = run.tasks.iter()
            .filter(|task| task.status == TaskStatus::Completed)
            .count();

        let failed = run.tasks.iter()
            .filter(|task| {
                matches!(
                    task.status,
                    TaskStatus::Failed | TaskStatus::Blocked
                )
            })
            .count();

        run.status = if failed == 0 {
            ResearchStatus::Completed
        } else if completed > 0 {
            ResearchStatus::PartiallyCompleted
        } else {
            ResearchStatus::Blocked
        };

        run.completed_at = Some(chrono::Utc::now());

        let report = create_report(run, evidence);

        self.repository
            .save(&report)
            .await
            .context("Le rapport est construit, mais son enregistrement a échoué")?;

        Ok(report)
    }

    pub async fn get_report(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<ResearchReport>> {
        self.repository.get(id).await
    }
}
