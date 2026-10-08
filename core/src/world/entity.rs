
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityKind {
    Person,
    Organization,
    Country,
    City,
    Company,
    Software,
    Machine,
    Device,
    Service,
    Institution,
    Market,
    Asset,
    NaturalObject,
    Concept,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityStatus {
    Active,
    Inactive,
    Unknown,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub kind: EntityKind,
    pub canonical_name: String,
    pub aliases: Vec<String>,
    pub description: Option<String>,
    pub status: EntityStatus,
    pub attributes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entity {
    pub fn new(
        kind: EntityKind,
        canonical_name: impl Into<String>,
    ) -> Result<Self, String> {
        let canonical_name = canonical_name.into();

        if canonical_name.trim().is_empty() {
            return Err("entity name cannot be empty".into());
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            canonical_name,
            aliases: Vec::new(),
            description: None,
            status: EntityStatus::Unknown,
            attributes: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn add_alias(
        &mut self,
        alias: impl Into<String>,
    ) -> Result<(), String> {
        let alias = alias.into().trim().to_owned();

        if alias.is_empty() {
            return Err("entity alias cannot be empty".into());
        }

        if alias != self.canonical_name
            && !self.aliases.iter().any(|item| item == &alias)
        {
            self.aliases.push(alias);
            self.updated_at = Utc::now();
        }

        Ok(())
    }

    pub fn set_attribute(
        &mut self,
        key: impl Into<String>,
        value: serde_json::Value,
    ) -> Result<(), String> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err("attribute key cannot be empty".into());
        }

        if !self.attributes.is_object() {
            self.attributes = serde_json::json!({});
        }

        self.attributes[key] = value;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn matches_name(&self, query: &str) -> bool {
        let query = query.trim();

        self.canonical_name.eq_ignore_ascii_case(query)
            || self.aliases.iter().any(|alias| {
                alias.eq_ignore_ascii_case(query)
            })
    }
}
