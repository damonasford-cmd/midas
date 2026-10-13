use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerceptionModality {
    Text,
    WebPage,
    Document,
    Image,
    Audio,
    Video,
    StructuredData,
    Sensor,
    SystemEvent,
    Unknown,
}

impl PerceptionModality {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::WebPage => "web_page",
            Self::Document => "document",
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Video => "video",
            Self::StructuredData => "structured_data",
            Self::Sensor => "sensor",
            Self::SystemEvent => "system_event",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for PerceptionModality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for PerceptionModality {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "text" => Ok(Self::Text),
            "web_page" => Ok(Self::WebPage),
            "document" => Ok(Self::Document),
            "image" => Ok(Self::Image),
            "audio" => Ok(Self::Audio),
            "video" => Ok(Self::Video),
            "structured_data" => Ok(Self::StructuredData),
            "sensor" => Ok(Self::Sensor),
            "system_event" => Ok(Self::SystemEvent),
            "unknown" => Ok(Self::Unknown),
            other => Err(format!("Unknown perception modality: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationStatus {
    Received,
    Accepted,
    Rejected,
    Processed,
    Failed,
    Archived,
}

impl ObservationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Received => "received",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Processed => "processed",
            Self::Failed => "failed",
            Self::Archived => "archived",
        }
    }
}

impl TryFrom<&str> for ObservationStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "received" => Ok(Self::Received),
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            "processed" => Ok(Self::Processed),
            "failed" => Ok(Self::Failed),
            "archived" => Ok(Self::Archived),
            other => Err(format!("Unknown observation status: {other}")),
        }
    }
}
