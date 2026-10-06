use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum CapabilityAvailability {
    Available,
    PartiallyAvailable,
    Missing,
    Unknown,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum CapabilityGapRoute {
    UseExisting,
    SearchExternal,
    IntegrateExisting,
    Design,
    Build,
    Test,
    Validate,
    Integrate,
    Learn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub id: Uuid,

    pub name: String,

    pub description: String,

    pub availability:
        CapabilityAvailability,

    pub required: bool,

    pub routes: Vec<CapabilityGapRoute>,
}

impl CapabilityRequirement {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Result<Self, String> {
        let name = name.into();
        let description = description.into();

        if name.trim().is_empty() {
            return Err(
                "capability name cannot be empty".into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            availability:
                CapabilityAvailability::Unknown,
            required,
            routes: vec![
                CapabilityGapRoute::SearchExternal,
                CapabilityGapRoute::Design,
                CapabilityGapRoute::Build,
                CapabilityGapRoute::Test,
                CapabilityGapRoute::Validate,
                CapabilityGapRoute::Integrate,
                CapabilityGapRoute::Learn,
            ],
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGap {
    pub id: Uuid,

    pub requirement_id: Uuid,

    pub capability_name: String,

    pub reason: String,

    pub blocking: bool,

    pub selected_route:
        Option<CapabilityGapRoute>,
}

impl CapabilityGap {
    pub fn new(
        requirement_id: Uuid,
        capability_name: impl Into<String>,
        reason: impl Into<String>,
        blocking: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            requirement_id,
            capability_name: capability_name.into(),
            reason: reason.into(),
            blocking,
            selected_route: None,
        }
    }
}
