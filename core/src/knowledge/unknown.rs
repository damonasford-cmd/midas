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
pub enum UnknownKind {
    MissingInformation,
    ConflictingInformation,
    Unverified,
    Inaccessible,
    Ambiguous,
    InsufficientEvidence,
    FutureOutcome,
    OutsideCurrentCapability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unknown {
    pub id: Uuid,

    pub description: String,

    pub kind: UnknownKind,

    pub importance: f32,

    pub blocking: bool,

    pub research_required: bool,
}

impl Unknown {
    pub fn new(
        description: impl Into<String>,
        kind: UnknownKind,
    ) -> Result<Self, String> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                "unknown description cannot be empty"
                    .into()
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            description,
            kind,
            importance: 0.5,
            blocking: false,
            research_required: true,
        })
    }
}
