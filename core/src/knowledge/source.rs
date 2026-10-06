use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum SourceAuthority {
    Unknown,
    Low,
    Moderate,
    High,
    Authoritative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSource {
    pub id: String,
    pub name: Option<String>,
    pub source_type: String,

    pub authority: SourceAuthority,

    pub reliability: f32,

    pub location: Option<String>,
}

impl KnowledgeSource {
    pub fn new(
        id: impl Into<String>,
        source_type: impl Into<String>,
    ) -> Result<Self, String> {
        let id = id.into();
        let source_type = source_type.into();

        if id.trim().is_empty() {
            return Err(
                "knowledge source id cannot be empty".into()
            );
        }

        if source_type.trim().is_empty() {
            return Err(
                "knowledge source type cannot be empty".into()
            );
        }

        Ok(Self {
            id,
            name: None,
            source_type,
            authority: SourceAuthority::Unknown,
            reliability: 0.5,
            location: None,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.reliability) {
            return Err(
                "source reliability must be between 0 and 1"
                    .into(),
            );
        }

        Ok(())
    }
}
