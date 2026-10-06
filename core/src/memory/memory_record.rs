use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    content::MemoryContent,
    memory_id::MemoryId,
    metadata::MemoryMetadata,
    modality::MemoryModality,
    provenance::MemorySource,
    timeline::MemoryTemporalPosition,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: MemoryId,

    pub content: MemoryContent,

    pub modality: MemoryModality,

    pub metadata: MemoryMetadata,

    pub sources: Vec<MemorySource>,

    pub temporal: MemoryTemporalPosition,

    pub embedding_reference:
        Option<super::embedding::EmbeddingReference>,

    pub embedding: Option<Vec<f32>>,

    pub correlation_id: Option<Uuid>,

    pub parent_memory: Option<MemoryId>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

impl MemoryRecord {
    pub fn validate(&self) -> Result<(), String> {
        self.content.validate()?;
        self.metadata.validate()?;

        for source in &self.sources {
            source.validate()?;
        }

        if let Some(reference) = &self.embedding_reference {
            if reference.dimensions == 0 {
                return Err(
                    "embedding reference dimensions cannot be zero"
                        .into(),
                );
            }
        }

        if let (Some(reference), Some(embedding)) =
            (&self.embedding_reference, &self.embedding)
        {
            if embedding.len() != reference.dimensions {
                return Err(
                    "stored embedding dimensions do not match reference"
                        .into(),
                );
            }
        }

        Ok(())
    }
}

pub struct MemoryRecordBuilder {
    content: MemoryContent,
    metadata: MemoryMetadata,
    sources: Vec<MemorySource>,
    temporal: MemoryTemporalPosition,
    embedding_reference:
        Option<super::embedding::EmbeddingReference>,
    embedding: Option<Vec<f32>>,
    correlation_id: Option<Uuid>,
    parent_memory: Option<MemoryId>,
}

impl MemoryRecordBuilder {
    pub fn new(content: MemoryContent) -> Self {
        Self {
            content,
            metadata: MemoryMetadata::default(),
            sources: Vec::new(),
            temporal: MemoryTemporalPosition::default(),
            embedding_reference: None,
            embedding: None,
            correlation_id: None,
            parent_memory: None,
        }
    }

    pub fn metadata(
        mut self,
        metadata: MemoryMetadata,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn source(
        mut self,
        source: MemorySource,
    ) -> Self {
        self.sources.push(source);
        self
    }

    pub fn temporal(
        mut self,
        temporal: MemoryTemporalPosition,
    ) -> Self {
        self.temporal = temporal;
        self
    }

    pub fn embedding(
        mut self,
        reference: super::embedding::EmbeddingReference,
        values: Vec<f32>,
    ) -> Self {
        self.embedding_reference = Some(reference);
        self.embedding = Some(values);
        self
    }

    pub fn correlation_id(
        mut self,
        correlation_id: Uuid,
    ) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn parent_memory(
        mut self,
        parent: MemoryId,
    ) -> Self {
        self.parent_memory = Some(parent);
        self
    }

    pub fn build(self) -> Result<MemoryRecord, String> {
        let now = Utc::now();

        let record = MemoryRecord {
            id: MemoryId::new(),
            modality: self.content.modality,
            content: self.content,
            metadata: self.metadata,
            sources: self.sources,
            temporal: self.temporal,
            embedding_reference: self.embedding_reference,
            embedding: self.embedding,
            correlation_id: self.correlation_id,
            parent_memory: self.parent_memory,
            created_at: now,
            updated_at: now,
        };

        record.validate()?;

        Ok(record)
    }
}
