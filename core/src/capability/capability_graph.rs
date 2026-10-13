use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct CapabilityGraph {
    dependencies: HashMap<Uuid, HashSet<Uuid>>,
}

impl CapabilityGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_edges(edges: impl IntoIterator<Item = (Uuid, Uuid)>) -> Result<Self> {
        let mut graph = Self::new();

        for (capability_id, dependency_id) in edges {
            graph.add_dependency(capability_id, dependency_id)?;
        }

        Ok(graph)
    }

    pub fn add_node(&mut self, capability_id: Uuid) {
        self.dependencies.entry(capability_id).or_default();
    }

    pub fn add_dependency(
        &mut self,
        capability_id: Uuid,
        dependency_id: Uuid,
    ) -> Result<()> {
        if capability_id == dependency_id {
            bail!("A capability cannot depend on itself");
        }

        self.add_node(capability_id);
        self.add_node(dependency_id);

        if self.reaches(dependency_id, capability_id) {
            bail!("Dependency would introduce a cycle");
        }

        self.dependencies
            .entry(capability_id)
            .or_default()
            .insert(dependency_id);

        Ok(())
    }

    pub fn dependencies_of(&self, capability_id: Uuid) -> Vec<Uuid> {
        let mut result: Vec<Uuid> = self
            .dependencies
            .get(&capability_id)
            .into_iter()
            .flat_map(|set| set.iter().copied())
            .collect();

        result.sort_unstable();
        result
    }

    pub fn transitive_dependencies(&self, capability_id: Uuid) -> Result<Vec<Uuid>> {
        if !self.dependencies.contains_key(&capability_id) {
            bail!("Capability does not exist in graph");
        }

        let mut visited = HashSet::new();
        let mut pending = self.dependencies_of(capability_id);

        while let Some(current) = pending.pop() {
            if !visited.insert(current) {
                continue;
            }

            pending.extend(self.dependencies_of(current));
        }

        visited.remove(&capability_id);

        let mut result: Vec<Uuid> = visited.into_iter().collect();
        result.sort_unstable();
        Ok(result)
    }

    pub fn has_cycle(&self) -> bool {
        fn visit(
            node: Uuid,
            graph: &HashMap<Uuid, HashSet<Uuid>>,
            visiting: &mut HashSet<Uuid>,
            visited: &mut HashSet<Uuid>,
        ) -> bool {
            if visiting.contains(&node) {
                return true;
            }

            if visited.contains(&node) {
                return false;
            }

            visiting.insert(node);

            if let Some(children) = graph.get(&node) {
                for child in children {
                    if visit(*child, graph, visiting, visited) {
                        return true;
                    }
                }
            }

            visiting.remove(&node);
            visited.insert(node);
            false
        }

        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        self.dependencies.keys().copied().any(|node| {
            visit(node, &self.dependencies, &mut visiting, &mut visited)
        })
    }

    fn reaches(&self, start: Uuid, target: Uuid) -> bool {
        let mut pending = vec![start];
        let mut visited = HashSet::new();

        while let Some(node) = pending.pop() {
            if node == target {
                return true;
            }

            if !visited.insert(node) {
                continue;
            }

            pending.extend(self.dependencies_of(node));
        }

        false
    }
}
