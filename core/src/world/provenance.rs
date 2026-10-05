use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceReliability {
    Unknown,
    Low,
    Medium,
    High,
    Verified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationSource {
    pub id: Uuid,
    pub name: String,
    pub source_type: String,
    pub reference: Option<String>,
    pub reliability: SourceReliability,
}

impl InformationSource {
    pub fn new(
        name: impl Into<String>,
        source_type: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            source_type: source_type.into(),
            reference: None,
            reliability: SourceReliability::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub source: InformationSource,
    pub collected_at: DateTime<Utc>,
    pub collector: String,
    pub transformations: Vec<String>,
}

impl Provenance {
    pub fn new(
        source: InformationSource,
        collector: impl Into<String>,
    ) -> Self {
        Self {
            source,
            collected_at: Utc::now(),
            collector: collector.into(),
            transformations: Vec::new(),
        }
    }

    pub fn add_transformation(
        &mut self,
        transformation: impl Into<String>,
    ) {
        self.transformations.push(transformation.into());
    }
}
