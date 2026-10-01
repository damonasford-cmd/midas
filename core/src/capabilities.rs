use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub available: bool,
}

#[derive(Debug, Default)]
pub struct CapabilityRegistry {
    capabilities: HashMap<String, Capability>,
}

impl CapabilityRegistry {
    pub fn register(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) {
        let name = name.into();

        self.capabilities.insert(
            name.clone(),
            Capability {
                name,
                description: description.into(),
                available: true,
            },
        );
    }

    pub fn has(&self, name: &str) -> bool {
        self.capabilities
            .get(name)
            .map(|c| c.available)
            .unwrap_or(false)
    }

    pub fn missing(&self, required: &[String]) -> Vec<String> {
        required
            .iter()
            .filter(|name| !self.has(name))
            .cloned()
            .collect()
    }

    pub fn all(&self) -> Vec<Capability> {
        self.capabilities.values().cloned().collect()
    }
}
