use anyhow::Result;

use crate::foundation::contracts::{
    Event,
    EventKind,
    Observation,
    ObservationModality,
};
use crate::foundation::event_bus::EventBus;
use crate::foundation::world_state::WorldState;

pub struct PerceptionEngine {
    event_bus: EventBus,
}

impl PerceptionEngine {
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }

    pub fn observe(
        &self,
        world: &mut WorldState,
        source: impl Into<String>,
        modality: ObservationModality,
        content: serde_json::Value,
        confidence: f64,
        provenance: Vec<String>,
    ) -> Result<Observation> {
        let observation = Observation {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            source: source.into(),
            modality,
            content,
            confidence: confidence.clamp(0.0, 1.0),
            provenance,
        };

        world.record_observation(observation.clone());

        self.event_bus.publish(Event::new(
            EventKind::WorldObservation,
            "cognition.perception",
            serde_json::to_value(&observation)?,
        ))?;

        Ok(observation)
    }
}
