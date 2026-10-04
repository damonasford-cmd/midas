use crate::foundation::contracts::Task;

#[derive(Debug, Clone, Default)]
pub struct PrioritizationEngine;

impl PrioritizationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn rank(&self, mut tasks: Vec<Task>) -> Vec<Task> {
        tasks.sort_by(|a, b| b.priority.total_cmp(&a.priority));
        tasks
    }

    pub fn highest(&self, tasks: &[Task]) -> Option<&Task> {
        tasks
            .iter()
            .max_by(|a, b| a.priority.total_cmp(&b.priority))
    }
}
