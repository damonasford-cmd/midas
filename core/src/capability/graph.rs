use crate::foundation::contracts::Capability;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapabilityGraph {
    capabilities: HashMap<String, Capability>,
    dependencies: HashMap<String, HashSet<String>>,
}

impl CapabilityGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, capability: Capability) {
        let id = capability.id.to_string();

        self.dependencies
            .entry(id.clone())
            .or_default()
            .extend(capability.dependencies.iter().cloned());

        self.capabilities.insert(id, capability);
    }

    pub fn get(&self, id: &str) -> Option<&Capability> {
        self.capabilities.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Capability> {
        self.capabilities.get_mut(id)
    }

    pub fn all(&self) -> Vec<Capability> {
        self.capabilities.values().cloned().collect()
    }

    pub fn available(&self) -> Vec<Capability> {
        self.capabilities
            .values()
            .filter(|capability| capability.available)
            .cloned()
            .collect()
    }

    pub fn missing(&self) -> Vec<Capability> {
        self.capabilities
            .values()
            .filter(|capability| !capability.available)
            .cloned()
            .collect()
    }

    pub fn dependencies_of(&self, id: &str) -> Vec<String> {
        self.dependencies
            .get(id)
            .map(|items| items.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn dependency_chain(&self, id: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        self.collect_dependencies(id, &mut visited, &mut result);

        result
    }

    fn collect_dependencies(
        &self,
        id: &str,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) {
        if !visited.insert(id.to_string()) {
            return;
        }

        if let Some(dependencies) = self.dependencies.get(id) {
            for dependency in dependencies {
                result.push(dependency.clone());
                self.collect_dependencies(dependency, visited, result);
            }
        }
    }

    pub fn mark_available(&mut self, id: &str, version: impl Into<String>) -> bool {
        if let Some(capability) = self.capabilities.get_mut(id) {
            capability.available = true;
            capability.version = version.into();
            return true;
        }

        false
    }

    pub fn mark_unavailable(&mut self, id: &str) -> bool {
        if let Some(capability) = self.capabilities.get_mut(id) {
            capability.available = false;
            return true;
        }

        false
    }

    pub fn contains(&self, id: &str) -> bool {
        self.capabilities.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }
}
