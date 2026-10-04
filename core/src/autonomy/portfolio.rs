use crate::foundation::contracts::{Project, ProjectStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub name: String,
    pub projects: Vec<Project>,
}

impl Portfolio {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            projects: Vec::new(),
        }
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn active_projects(&self) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|project| {
                !matches!(
                    project.status,
                    ProjectStatus::Completed | ProjectStatus::Archived
                )
            })
            .collect()
    }
}
