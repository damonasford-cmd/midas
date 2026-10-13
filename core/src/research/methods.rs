
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::research::{
    domain::{ResearchMethodKind, ResearchObjective, ResearchTask},
    evidence::Evidence,
};

#[derive(Debug, Clone)]
pub struct MethodDescriptor {
    pub id: String,
    pub name: String,
    pub kind: ResearchMethodKind,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct MethodResult {
    pub summary: String,
    pub evidence: Vec<Evidence>,
    pub warnings: Vec<String>,
}

#[async_trait]
pub trait ResearchMethod: Send + Sync {
    fn descriptor(&self) -> MethodDescriptor;

    async fn investigate(
        &self,
        objective: &ResearchObjective,
        task: &ResearchTask,
    ) -> Result<MethodResult>;
}

#[derive(Default)]
pub struct ResearchMethodRegistry {
    methods: Vec<Arc<dyn ResearchMethod>>,
}

impl ResearchMethodRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, method: Arc<dyn ResearchMethod>) -> Result<()> {
        let descriptor = method.descriptor();

        anyhow::ensure!(!descriptor.id.trim().is_empty(), "ID de méthode vide");
        anyhow::ensure!(!descriptor.name.trim().is_empty(), "Nom de méthode vide");

        if self.methods.iter().any(|existing| {
            existing.descriptor().id == descriptor.id
        }) {
            anyhow::bail!("Méthode déjà enregistrée : {}", descriptor.id);
        }

        self.methods.push(method);
        Ok(())
    }

    pub fn available_for(
        &self,
        kind: ResearchMethodKind,
    ) -> Vec<Arc<dyn ResearchMethod>> {
        self.methods.iter()
            .filter(|method| {
                let descriptor = method.descriptor();
                descriptor.enabled
                    && (descriptor.kind == kind
                        || descriptor.kind == ResearchMethodKind::GeneralAnalysis)
            })
            .cloned()
            .collect()
    }

    pub fn descriptors(&self) -> Vec<MethodDescriptor> {
        self.methods.iter()
            .map(|method| method.descriptor())
            .collect()
    }
}
