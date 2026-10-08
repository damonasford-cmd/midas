use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub event_type: String,
    pub source: String,
    pub schema_version: u32,
    pub occurred_at: DateTime<Utc>,
    pub recorded_at: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub idempotency_key: Option<String>,
    pub payload: Value,
}

impl EventEnvelope {
    pub fn new(
        event_type: impl Into<String>,
        source: impl Into<String>,
        payload: Value,
    ) -> Result<Self> {
        let now = Utc::now();

        let event = Self {
            id: Uuid::new_v4(),
            event_type: event_type.into(),
            source: source.into(),
            schema_version: 1,
            occurred_at: now,
            recorded_at: now,
            correlation_id: None,
            causation_id: None,
            idempotency_key: None,
            payload,
        };

        event.validate()?;
        Ok(event)
    }

    pub fn with_correlation(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn caused_by(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }

    pub fn with_idempotency_key(
        mut self,
        key: impl Into<String>,
    ) -> Result<Self> {
        let key = key.into();

        if key.trim().is_empty() {
            bail!("idempotency key cannot be empty");
        }

        self.idempotency_key = Some(key);
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<()> {
        if self.event_type.trim().is_empty() {
            bail!("event_type cannot be empty");
        }

        if self.source.trim().is_empty() {
            bail!("event source cannot be empty");
        }

        if self.schema_version == 0 {
            bail!("schema_version must be greater than zero");
        }

        if self.recorded_at < self.occurred_at {
            bail!("recorded_at cannot precede occurred_at");
        }

        if self
            .idempotency_key
            .as_ref()
            .is_some_and(|key| key.trim().is_empty())
        {
            bail!("idempotency key cannot be empty");
        }

        Ok(())
    }
}
