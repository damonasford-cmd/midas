use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};

use super::capability::CapabilityKey;

#[derive(Debug, Clone, Default)]
pub struct CapabilityGraph {
    edges: HashMap<CapabilityKey, HashSet<CapabilityKey>>,
}

impl CapabilityGraph {
    pub fn from_edges(
        edges: impl IntoIterator<Item = (CapabilityKey, CapabilityKey)>,
    ) -> Result<Self> {
        let mut graph = Self::default();

        for (capability, dependency) in edges {
            graph.add_dependency(capability, dependency)?;
        }

        Ok(graph)
    }

    pub fn add_dependency(
        &mut self,
        capability: CapabilityKey,
        dependency: CapabilityKey,
    ) -> Result<()> {
        if capability == dependency {
            bail!("a capability cannot depend on itself");
        }

        let key = capability.clone();
        let inserted = self
            .edges
            .entry(key.clone())
            .or_default()
            .insert(dependency.clone());

        if inserted {
            if let Err(error) = self.validate_acyclic() {
                if let Some(dependencies) = self.edges.get_mut(&key) {
                    dependencies.remove(&dependency);
                    if dependencies.is_empty() {
                        self.edges.remove(&key);
                    }
                }
                return Err(error);
            }
        }

        Ok(())
    }

    pub fn dependencies_of(
        &self,
        capability: &CapabilityKey,
    ) -> Vec<CapabilityKey> {
        let mut result: Vec<_> = self
            .edges
            .get(capability)
            .into_iter()
            .flat_map(|items| items.iter().cloned())
            .collect();

        result.sort_by(|a, b| a.as_str().cmp(b.as_str()));
        result
    }

    pub fn validate_known_nodes(
        &self,
        known: &HashSet<CapabilityKey>,
    ) -> Result<()> {
        for (capability, dependencies) in &self.edges {
            if !known.contains(capability) {
                bail!("unknown capability node: {}", capability.as_str());
            }

            for dependency in dependencies {
                if !known.contains(dependency) {
                    bail!("unknown dependency node: {}", dependency.as_str());
                }
            }
        }

        Ok(())
    }

    pub fn validate_acyclic(&self) -> Result<()> {
        fn visit(
            node: &CapabilityKey,
            edges: &HashMap<CapabilityKey, HashSet<CapabilityKey>>,
            visiting: &mut HashSet<CapabilityKey>,
            visited: &mut HashSet<CapabilityKey>,
        ) -> Result<()> {
            if visited.contains(node) {
                return Ok(());
            }

            if !visiting.insert(node.clone()) {
                bail!("capability dependency cycle detected at {}", node.as_str());
            }

            if let Some(dependencies) = edges.get(node) {
                for dependency in dependencies {
                    visit(dependency, edges, visiting, visited)?;
                }
            }

            visiting.remove(node);
            visited.insert(node.clone());
            Ok(())
        }

        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        for node in self.edges.keys() {
            visit(node, &self.edges, &mut visiting, &mut visited)?;
        }

        Ok(())
    }
}
