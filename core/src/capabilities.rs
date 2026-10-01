use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub available: bool,
    pub version: Option<String>,
    pub source: CapabilitySource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilitySource {
    Native,
    IntegratedTool,
    ExternalService,
    Generated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub name: String,
    pub reason: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityAssessment {
    pub required: Vec<CapabilityRequirement>,
    pub available: Vec<String>,
    pub missing: Vec<String>,
    pub ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityPlan {
    UseExisting {
        capability: String,
    },

    SearchAndIntegrate {
        capability: String,
        reason: String,
    },

    DesignAndBuild {
        capability: String,
        reason: String,
    },
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
                version: None,
                source: CapabilitySource::Native,
            },
        );
    }

    pub fn register_with_source(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        source: CapabilitySource,
        version: Option<String>,
    ) {
        let name = name.into();

        self.capabilities.insert(
            name.clone(),
            Capability {
                name,
                description: description.into(),
                available: true,
                version,
                source,
            },
        );
    }

    pub fn disable(
        &mut self,
        name: &str,
    ) {
        if let Some(capability) =
            self.capabilities.get_mut(name)
        {
            capability.available = false;
        }
    }

    pub fn enable(
        &mut self,
        name: &str,
    ) {
        if let Some(capability) =
            self.capabilities.get_mut(name)
        {
            capability.available = true;
        }
    }

    pub fn has(
        &self,
        name: &str,
    ) -> bool {
        self.capabilities
            .get(name)
            .map(|capability| capability.available)
            .unwrap_or(false)
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&Capability> {
        self.capabilities.get(name)
    }

    pub fn all(
        &self,
    ) -> Vec<Capability> {
        self.capabilities
            .values()
            .cloned()
            .collect()
    }

    pub fn available(
        &self,
    ) -> Vec<Capability> {
        self.capabilities
            .values()
            .filter(|capability| capability.available)
            .cloned()
            .collect()
    }

    pub fn assess(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> CapabilityAssessment {
        let mut available = Vec::new();
        let mut missing = Vec::new();

        for requirement in requirements {
            if self.has(&requirement.name) {
                available.push(
                    requirement.name.clone(),
                );
            } else if requirement.required {
                missing.push(
                    requirement.name.clone(),
                );
            }
        }

        CapabilityAssessment {
            required: requirements.to_vec(),
            available,
            ready: missing.is_empty(),
            missing,
        }
    }

    pub fn build_plan(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> Vec<CapabilityPlan> {
        requirements
            .iter()
            .map(|requirement| {
                if self.has(&requirement.name) {
                    CapabilityPlan::UseExisting {
                        capability: requirement.name.clone(),
                    }
                } else {
                    CapabilityPlan::SearchAndIntegrate {
                        capability: requirement.name.clone(),
                        reason: requirement.reason.clone(),
                    }
                }
            })
            .collect()
    }

    pub fn plan_missing_capability(
        &self,
        requirement: &CapabilityRequirement,
    ) -> CapabilityPlan {
        if self.has(&requirement.name) {
            return CapabilityPlan::UseExisting {
                capability: requirement.name.clone(),
            };
        }

        CapabilityPlan::DesignAndBuild {
            capability: requirement.name.clone(),
            reason: requirement.reason.clone(),
        }
    }
}
