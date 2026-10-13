use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::capability_state::CapabilityState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityKey(pub String);

impl CapabilityKey {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into().trim().to_ascii_lowercase();

        if value.is_empty() {
            bail!("capability key cannot be empty");
        }

        if value.len() > 160 {
            bail!("capability key cannot exceed 160 bytes");
        }

        if !value.chars().all(|c| {
            c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || matches!(c, '.' | '_' | '-' | '/')
        }) {
            bail!("capability key contains unsupported characters");
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCapability {
    pub key: CapabilityKey,
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub owner: String,
    pub dependencies: Vec<CapabilityKey>,
    pub required_permissions: Vec<String>,
    pub requirements: Value,
    pub metadata: Value,
}

impl NewCapability {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            bail!("capability name cannot be empty");
        }
        if self.description.trim().is_empty() {
            bail!("capability description cannot be empty");
        }
        if self.category.trim().is_empty() {
            bail!("capability category cannot be empty");
        }
        if self.version.trim().is_empty() {
            bail!("capability version cannot be empty");
        }
        if self.owner.trim().is_empty() {
            bail!("capability owner cannot be empty");
        }
        if self.name.len() > 300
            || self.description.len() > 20_000
            || self.category.len() > 120
            || self.version.len() > 120
            || self.owner.len() > 200
        {
            bail!("capability field exceeds its maximum length");
        }

        let mut seen = std::collections::HashSet::new();
        for dependency in &self.dependencies {
            if dependency == &self.key {
                bail!("a capability cannot depend on itself");
            }
            if !seen.insert(dependency.as_str()) {
                bail!("duplicate capability dependency");
            }
        }

        for permission in &self.required_permissions {
            if permission.trim().is_empty() {
                bail!("required permissions cannot contain empty values");
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: Uuid,
    pub key: CapabilityKey,
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub owner: String,
    pub state: CapabilityState,
    pub dependencies: Vec<CapabilityKey>,
    pub required_permissions: Vec<String>,
    pub requirements: Value,
    pub metadata: Value,
    pub revision: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySummary {
    pub key: CapabilityKey,
    pub state: CapabilityState,
    pub revision: i64,
    pub last_evaluated_at: Option<DateTime<Utc>>,
    pub evaluation_valid_until: Option<DateTime<Utc>>,
}
