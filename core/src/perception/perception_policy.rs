use anyhow::{bail, Result};

use super::{
    observation::Observation,
    source::PerceptionSource,
};

#[derive(Debug, Clone)]
pub struct PerceptionPolicy {
    pub max_batch_size: usize,
    pub max_payload_bytes: usize,
    pub max_text_bytes: usize,
    pub allow_unknown_modality: bool,
}

impl Default for PerceptionPolicy {
    fn default() -> Self {
        Self {
            max_batch_size: 500,
            max_payload_bytes: 5 * 1024 * 1024,
            max_text_bytes: 2_000_000,
            allow_unknown_modality: false,
        }
    }
}

impl PerceptionPolicy {
    pub fn validate_source(&self, source: &PerceptionSource) -> Result<()> {
        source.validate()?;

        if !source.enabled {
            bail!("Perception source is disabled");
        }

        Ok(())
    }

    pub fn validate_observation(&self, observation: &Observation) -> Result<()> {
        observation.validate()?;

        if !self.allow_unknown_modality
            && observation.modality == super::modality::PerceptionModality::Unknown
        {
            bail!("Unknown modality is not accepted by the current policy");
        }

        if serde_json::to_vec(&observation.payload)?.len()
            > self.max_payload_bytes
        {
            bail!("Payload exceeds the configured policy limit");
        }

        if observation
            .text_content
            .as_ref()
            .is_some_and(|text| text.len() > self.max_text_bytes)
        {
            bail!("Text content exceeds the configured policy limit");
        }

        Ok(())
    }

    pub fn validate_batch_size(&self, count: usize) -> Result<()> {
        if count > self.max_batch_size {
            bail!(
                "Batch contains {count} observations; maximum is {}",
                self.max_batch_size
            );
        }

        Ok(())
    }
}
