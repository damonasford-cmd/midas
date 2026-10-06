use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum ProvenanceReliability {
    Unknown,
    VeryLow,
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionProvenance {
    pub source_id: Uuid,

    pub source_name: String,

    pub acquired_at:
        chrono::DateTime<chrono::Utc>,

    pub event_id: Option<Uuid>,

    pub parent_observation:
        Option<Uuid>,

    pub transformation_chain:
        Vec<String>,

    pub direct:
        bool,

    pub reliability:
        ProvenanceReliability,
}

impl PerceptionProvenance {
    pub fn direct(
        source_id: Uuid,
        source_name: impl Into<String>,
    ) -> Self {
        Self {
            source_id,
            source_name: source_name.into(),
            acquired_at: chrono::Utc::now(),
            event_id: None,
            parent_observation: None,
            transformation_chain: Vec::new(),
            direct: true,
            reliability:
                ProvenanceReliability::Moderate,
        }
    }
}
