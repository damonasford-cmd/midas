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
pub enum PayloadKind {
    Text,
    Json,
    Binary,
    Image,
    Video,
    Audio,
    Document,
    Telemetry,
    Event,
    SensorReading,
    SoftwareState,
    EnvironmentState,
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
pub enum PayloadEncoding {
    Utf8,
    Json,
    Base64,
    Raw,
    Compressed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionPayload {
    pub kind: PayloadKind,

    pub encoding: PayloadEncoding,

    pub content_type: Option<String>,

    pub data: Vec<u8>,

    pub size_bytes: usize,

    pub checksum: Option<String>,
}

impl PerceptionPayload {
    pub fn new(
        kind: PayloadKind,
        encoding: PayloadEncoding,
        data: Vec<u8>,
    ) -> Self {
        let size_bytes = data.len();

        Self {
            kind,
            encoding,
            content_type: None,
            data,
            size_bytes,
            checksum: None,
        }
    }

    pub fn empty(
        kind: PayloadKind,
    ) -> Self {
        Self::new(
            kind,
            PayloadEncoding::Raw,
            Vec::new(),
        )
    }
}
