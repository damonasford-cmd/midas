use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perception {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub modality: PerceptionModality,
    pub content: String,
    pub metadata: serde_json::Value,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerceptionModality {
    Text,
    Data,
    File,
    Image,
    Audio,
    Video,
    Code,
    Event,
    Sensor,
    Web,
    Software,
    Machine,
    ActionResult,
    Unknown,
}

impl Perception {
    pub fn new(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: source.into(),
            modality: PerceptionModality::Text,
            content: content.into(),
            metadata: serde_json::Value::Object(
                serde_json::Map::new(),
            ),
            confidence: 1.0,
        }
    }

    pub fn with_modality(
        mut self,
        modality: PerceptionModality,
    ) -> Self {
        self.modality = modality;
        self
    }

    pub fn with_confidence(
        mut self,
        confidence: f32,
    ) -> Self {
        self.confidence =
            confidence.clamp(0.0, 1.0);

        self
    }

    pub fn with_metadata(
        mut self,
        metadata: serde_json::Value,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn text(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Text,
            )
    }

    pub fn data(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Data,
            )
    }

    pub fn file(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::File,
            )
    }

    pub fn image(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Image,
            )
    }

    pub fn audio(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Audio,
            )
    }

    pub fn video(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Video,
            )
    }

    pub fn code(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Code,
            )
    }

    pub fn event(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Event,
            )
    }

    pub fn sensor(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Sensor,
            )
    }

    pub fn web(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Web,
            )
    }

    pub fn software(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Software,
            )
    }

    pub fn machine(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Machine,
            )
    }

    pub fn action_result(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::ActionResult,
            )
    }

    pub fn unknown(
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(source, content)
            .with_modality(
                PerceptionModality::Unknown,
            )
    }

    pub fn is_reliable(&self) -> bool {
        self.confidence >= 0.8
    }

    pub fn summary(&self) -> String {
        format!(
            "[{:?}] {} : {}",
            self.modality,
            self.source,
            self.content
        )
    }
}

#[derive(Debug, Default)]
pub struct PerceptionEngine;

impl PerceptionEngine {
    pub fn ingest(
        perception: Perception,
    ) -> Perception {
        perception
    }

    pub fn ingest_many(
        perceptions: Vec<Perception>,
    ) -> Vec<Perception> {
        perceptions
    }

    pub fn filter_reliable(
        perceptions: &[Perception],
    ) -> Vec<Perception> {
        perceptions
            .iter()
            .filter(|perception| {
                perception.is_reliable()
            })
            .cloned()
            .collect()
    }

    pub fn by_modality(
        perceptions: &[Perception],
        modality: PerceptionModality,
    ) -> Vec<Perception> {
        perceptions
            .iter()
            .filter(|perception| {
                perception.modality == modality
            })
            .cloned()
            .collect()
    }

    pub fn summarize(
        perceptions: &[Perception],
    ) -> String {
        if perceptions.is_empty() {
            return "Aucune perception.".to_string();
        }

        perceptions
            .iter()
            .map(Perception::summary)
            .collect::<Vec<_>>()
            .join("\n")
    }
}
