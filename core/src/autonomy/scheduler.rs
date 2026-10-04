use crate::foundation::contracts::{Task, TaskStatus};
use chrono::Utc;

#[derive(Debug, Clone, Default)]
pub struct Scheduler;

impl Scheduler {
    pub fn new() -> Self {
        Self
    }

    pub fn schedule(&self, mut tasks: Vec<Task>) -> Vec<Task> {
        tasks.sort_by(|a, b| b.priority.total_cmp(&a.priority));

        for task in &mut tasks {
            if matches!(task.status, TaskStatus::Pending) {
                task.status = TaskStatus::Waiting;
                task.updated_at = Utc::now();
            }
        }

        tasks
    }
}
