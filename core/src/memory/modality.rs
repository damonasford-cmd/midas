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
pub enum MemoryModality {
    Text,
    Image,
    Audio,
    Video,
    File,
    StructuredData,
    Code,
    Event,
    Observation,
    Action,
    Decision,
    Experience,
    Mixed,
    Unknown,
}
