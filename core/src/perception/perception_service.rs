use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use super::{
    modality::ObservationStatus,
    observation::Observation,
    perception_pipeline::PerceptionPipeline,
    perception_repository::{IngestReceipt, PerceptionRepository},
    source::PerceptionConnector,
};

#[derive(Debug, Clone)]
pub struct PerceptionService {
    pipeline: PerceptionPipeline,
    repository: PerceptionRepository,
}

impl PerceptionService {
    pub fn new(
        pipeline: PerceptionPipeline,
        repository: PerceptionRepository,
    ) -> Self {
        Self {
            pipeline,
            repository,
        }
    }

    pub async fn acquire_and_ingest(
        &self,
        connector: &dyn PerceptionConnector,
    ) -> Result<Vec<IngestReceipt>> {
        let observations = self.pipeline.acquire(connector).await?;
        self.repository.ingest_batch(&observations).await
    }

    pub async fn ingest_manual_text(
        &self,
        source_id: String,
        text: String,
    ) -> Result<IngestReceipt> {
        let observation =
            PerceptionPipeline::create_manual_text(source_id, text)?;

        let mut receipts = self
            .repository
            .ingest_batch(&[observation])
            .await?;

        receipts
            .pop()
            .ok_or_else(|| anyhow::anyhow!("No ingestion receipt was returned"))
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Observation>> {
        self.repository.get(id).await
    }

    pub async fn list_recent(
        &self,
        limit: i64,
        before: Option<DateTime<Utc>>,
    ) -> Result<Vec<Observation>> {
        self.repository.list_recent(limit, before).await
    }

    pub async fn mark_processed(&self, id: Uuid) -> Result<bool> {
        self.repository
            .set_status(id, ObservationStatus::Processed)
            .await
    }

    pub async fn mark_failed(&self, id: Uuid) -> Result<bool> {
        self.repository
            .set_status(id, ObservationStatus::Failed)
            .await
    }
}
