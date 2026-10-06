use std::collections::HashMap;

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
pub enum CognitiveTaskStatus {
    Pending,
    Ready,
    Running,
    Blocked,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveTask {
    pub id: Uuid,

    pub title: String,

    pub description: String,

    pub status: CognitiveTaskStatus,

    pub dependencies: Vec<Uuid>,

    pub priority: i32,

    pub estimated_complexity: f32,

    pub reversible: bool,
}

impl CognitiveTask {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, String> {
        let title = title.into();
        let description = description.into();

        if title.trim().is_empty() {
            return Err(
                "cognitive task title cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            title,
            description,
            status: CognitiveTaskStatus::Pending,
            dependencies: Vec::new(),
            priority: 0,
            estimated_complexity: 0.5,
            reversible: true,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct CognitiveTaskGraph {
    tasks: HashMap<Uuid, CognitiveTask>,
}

impl CognitiveTaskGraph {
    pub fn insert(
        &mut self,
        task: CognitiveTask,
    ) -> Result<(), String> {
        if self.tasks.contains_key(&task.id) {
            return Err(
                "cognitive task already exists".into()
            );
        }

        self.tasks.insert(task.id, task);

        Ok(())
    }

    pub fn get(
        &self,
        id: Uuid,
    ) -> Option<&CognitiveTask> {
        self.tasks.get(&id)
    }

    pub fn get_mut(
        &mut self,
        id: Uuid,
    ) -> Option<&mut CognitiveTask> {
        self.tasks.get_mut(&id)
    }

    pub fn ready_tasks(&self) -> Vec<&CognitiveTask> {
        self.tasks
            .values()
            .filter(|task| {
                if matches!(
                    task.status,
                    CognitiveTaskStatus::Completed
                        | CognitiveTaskStatus::Cancelled
                ) {
                    return false;
                }

                task.dependencies.iter().all(|dependency| {
                    self.tasks
                        .get(dependency)
                        .map(|task| {
                            task.status
                                == CognitiveTaskStatus::Completed
                        })
                        .unwrap_or(false)
                }) || task.dependencies.is_empty()
            })
            .collect()
    }

    pub fn all(&self) -> Vec<&CognitiveTask> {
        self.tasks.values().collect()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
