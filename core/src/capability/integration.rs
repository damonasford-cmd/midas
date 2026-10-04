use crate::capability::graph::CapabilityGraph;
use crate::foundation::contracts::Capability;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct CapabilityIntegration {
    graph: CapabilityGraph,
}

impl CapabilityIntegration {
    pub fn new() -> Self {
        Self {
            graph: CapabilityGraph::new(),
        }
    }

    pub fn register(&mut self, capability: Capability) -> Result<()> {
        for dependency in &capability.dependencies {
            if !self.graph.contains(dependency) {
                return Err(anyhow!(
                    "missing capability dependency: {}",
                    dependency
                ));
            }
        }

        self.graph.register(capability);
        Ok(())
    }

    pub fn force_register(&mut self, capability: Capability) {
        self.graph.register(capability);
    }

    pub fn graph(&self) -> &CapabilityGraph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut CapabilityGraph {
        &mut self.graph
    }

    pub fn activate(
        &mut self,
        id: &str,
        version: impl Into<String>,
    ) -> Result<()> {
        let dependencies = self.graph.dependencies_of(id);

        for dependency in dependencies {
            match self.graph.get(&dependency) {
                Some(capability) if capability.available => {}
                Some(_) => {
                    return Err(anyhow!(
                        "dependency '{}' is unavailable",
                        dependency
                    ));
                }
                None => {
                    return Err(anyhow!(
                        "dependency '{}' does not exist",
                        dependency
                    ));
                }
            }
        }

        if !self.graph.mark_available(id, version) {
            return Err(anyhow!("capability '{}' does not exist", id));
        }

        Ok(())
    }
}
