use crate::foundation::contracts::{Goal, Task, TaskStatus};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }

    pub fn decompose(&self, goal: &Goal) -> Vec<Task> {
        let now = Utc::now();

        vec![Task {
            id: Uuid::new_v4(),
            goal_id: goal.id,
            title: format!("Plan: {}", goal.title),
            description: goal.description.clone(),
            priority: goal.priority,
            status: TaskStatus::Planning,
            dependencies: Vec::new(),
            created_at: now,
            updated_at: now,
        }]
    }
}
