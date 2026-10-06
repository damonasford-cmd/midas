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
pub enum SourceKind {
    User,
    Camera,
    Microphone,
    FileSystem,
    Database,
    Web,
    Api,
    Sensor,
    Machine,
    Device,
    OperatingSystem,
    Network,
    Application,
    Service,
    EventBus,
    WorldModel,
    Memory,
    ExternalSystem,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionSource {
    pub id: Uuid,

    pub kind: SourceKind,

    pub name: String,

    pub address: Option<String>,

    pub authenticated: bool,

    pub trusted: bool,

    pub reliability: f32,
}

impl PerceptionSource {
    pub fn new(
        kind: SourceKind,
        name: impl Into<String>,
    ) -> Result<Self, String> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(
                "perception source name cannot be empty"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            name,
            address: None,
            authenticated: false,
            trusted: false,
            reliability: 0.5,
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
