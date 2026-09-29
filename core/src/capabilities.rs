use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct CapabilityRegistry {
    capabilities: BTreeSet<String>,
}

impl CapabilityRegistry {
    pub fn register(
        &mut self,
        capability: impl Into<String>,
    ) {
        self.capabilities.insert(capability.into());
    }

    pub fn has(&self, capability: &str) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn all(&self) -> Vec<String> {
        self.capabilities.iter().cloned().collect()
    }
}
