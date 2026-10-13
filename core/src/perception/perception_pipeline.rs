use anyhow::{Context, Result};
use chrono::Utc;

use super::{
    modality::ObservationStatus,
    observation::Observation,
    perception_policy::PerceptionPolicy,
    source::{AcquiredObservation, ObservationDraft, PerceptionConnector},
};

#[derive(Debug, Clone)]
pub struct PerceptionPipeline {
    policy: PerceptionPolicy,
}

impl PerceptionPipeline {
    pub fn new(policy: PerceptionPolicy) -> Self {
        Self { policy }
    }

    pub fn policy(&self) -> &PerceptionPolicy {
        &self.policy
    }

    pub async fn acquire(
        &self,
        connector: &dyn PerceptionConnector,
    ) -> Result<Vec<Observation>> {
        self.policy.validate_source(connector.source())?;

        let drafts = connector
            .acquire()
            .await
            .with_context(|| {
                format!(
                    "Perception acquisition failed for source '{}'",
                    connector.source().id
                )
            })?;

        self.policy.validate_batch_size(drafts.len())?;

        let mut observations = Vec::with_capacity(drafts.len());

        for draft in drafts {
            if draft.source_id != connector.source().id {
                anyhow::bail!(
                    "Connector '{}' returned data attributed to source '{}'",
                    connector.source().id,
                    draft.source_id
                );
            }

            observations.push(Self::normalize(draft)?);
        }

        Ok(observations)
    }

    pub fn normalize(draft: ObservationDraft) -> Result<Observation> {
        let acquired = AcquiredObservation::new(draft)?;

        let observation = Observation {
            id: acquired.id,
            source_id: acquired.draft.source_id,
            modality: acquired.draft.modality,
            content_type: acquired.draft.content_type,
            payload: acquired.draft.payload,
            text_content: acquired.draft.text_content,
            source_uri: acquired.draft.source_uri,
            source_record_id: acquired.draft.source_record_id,
            deduplication_key: acquired.draft.deduplication_key,
            occurred_at: acquired.draft.occurred_at,
            acquired_at: acquired.acquired_at,
            source_confidence: acquired.draft.source_confidence,
            metadata: acquired.draft.metadata,
            status: ObservationStatus::Accepted,
        };

        observation.validate()?;
        Ok(observation)
    }

    pub fn create_manual_text(
        source_id: String,
        text: String,
    ) -> Result<Observation> {
        if text.trim().is_empty() {
            anyhow::bail!("Manual text observation cannot be empty");
        }

        Self::normalize(ObservationDraft {
            source_id,
            modality: super::modality::PerceptionModality::Text,
            content_type: "text/plain; charset=utf-8".to_owned(),
            payload: serde_json::json!({ "text": text }),
            text_content: Some(text),
            source_uri: None,
            source_record_id: None,
            deduplication_key: None,
            occurred_at: Utc::now(),
            source_confidence: None,
            metadata: serde_json::json!({
                "ingestion_method": "manual_text"
            }),
        })
    }
}
