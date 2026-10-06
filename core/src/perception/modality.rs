use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum PerceptionModality {
    Text,
    StructuredData,
    Image,
    Video,
    Audio,
    Voice,
    File,
    Document,
    Web,
    Software,
    Screen,
    Sensor,
    Machine,
    Device,
    Network,
    Event,
    Environment,
    Location,
    Telemetry,
    HumanInteraction,
    ExternalSystem,
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
pub enum PerceptionModalityGroup {
    Human,
    Digital,
    Visual,
    Auditory,
    Physical,
    Environmental,
    Network,
    System,
}

impl PerceptionModality {
    pub fn group(&self) -> PerceptionModalityGroup {
        match self {
            Self::Text
            | Self::Audio
            | Self::Voice
            | Self::HumanInteraction => {
                PerceptionModalityGroup::Human
            }

            Self::StructuredData
            | Self::File
            | Self::Document
            | Self::Web
            | Self::Software
            | Self::Screen => {
                PerceptionModalityGroup::Digital
            }

            Self::Image | Self::Video => {
                PerceptionModalityGroup::Visual
            }

            Self::Sensor
            | Self::Machine
            | Self::Device
            | Self::Telemetry => {
                PerceptionModalityGroup::Physical
            }

            Self::Environment
            | Self::Location => {
                PerceptionModalityGroup::Environmental
            }

            Self::Network
            | Self::ExternalSystem => {
                PerceptionModalityGroup::Network
            }

            Self::Event => {
                PerceptionModalityGroup::System
            }
        }
    }
}
