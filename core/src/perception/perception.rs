use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use super::{
    input::{
        PerceptionInput,
        PerceptionInputState,
    },
    modality::PerceptionModality,
    observation::{
        ObservationStatus,
        PerceptualObservation,
    },
    perception_cycle::{
        PerceptionCycle,
        PerceptionCycleStatus,
    },
    health::{
        PerceptionHealth,
        PerceptionHealthState,
    },
};

#[derive(Debug, Clone)]
pub struct PerceptionConfig {
    pub minimum_quality:
        f32,

    pub minimum_source_reliability:
        f32,

    pub enable_multimodal_fusion:
        bool,

    pub enable_uncertainty:
        bool,

    pub enable_anomaly_detection:
        bool,

    pub reject_empty_payloads:
        bool,
}

impl Default for PerceptionConfig {
    fn default() -> Self {
        Self {
            minimum_quality: 0.2,

            minimum_source_reliability:
                0.2,

            enable_multimodal_fusion:
                true,

            enable_uncertainty:
                true,

            enable_anomaly_detection:
                true,

            reject_empty_payloads:
                true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerceptionRequest {
    pub request_id:
        Uuid,

    pub inputs:
        Vec<PerceptionInput>,

    pub objective:
        Option<String>,

    pub required_modalities:
        Vec<PerceptionModality>,

    pub require_fusion:
        bool,
}

impl PerceptionRequest {
    pub fn new(
        inputs: Vec<PerceptionInput>,
    ) -> Self {
        Self {
            request_id:
                Uuid::new_v4(),

            inputs,

            objective: None,

            required_modalities:
                Vec::new(),

            require_fusion: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerceptionResult {
    pub request_id:
        Uuid,

    pub cycle:
        PerceptionCycle,

    pub observations:
        Vec<PerceptualObservation>,

    pub partial:
        bool,

    pub errors:
        Vec<String>,
}

pub struct PerceptionEngine {
    config:
        PerceptionConfig,

    active:
        Arc<RwLock<bool>>,

    inputs_received:
        Arc<RwLock<u64>>,

    inputs_processed:
        Arc<RwLock<u64>>,

    inputs_failed:
        Arc<RwLock<u64>>,

    observations_created:
        Arc<RwLock<u64>>,

    anomalies_detected:
        Arc<RwLock<u64>>,

    degraded_sources:
        Arc<RwLock<u64>>,
}

impl PerceptionEngine {
    pub fn new(
        config: PerceptionConfig,
    ) -> Self {
        Self {
            config,

            active:
                Arc::new(
                    RwLock::new(false),
                ),

            inputs_received:
                Arc::new(
                    RwLock::new(0),
                ),

            inputs_processed:
                Arc::new(
                    RwLock::new(0),
                ),

            inputs_failed:
                Arc::new(
                    RwLock::new(0),
                ),

            observations_created:
                Arc::new(
                    RwLock::new(0),
                ),

            anomalies_detected:
                Arc::new(
                    RwLock::new(0),
                ),

            degraded_sources:
                Arc::new(
                    RwLock::new(0),
                ),
        }
    }

    pub async fn start(&self) {
        *self.active.write().await = true;
    }

    pub async fn stop(&self) {
        *self.active.write().await = false;
    }

    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }

    pub async fn process(
        &self,
        request: PerceptionRequest,
    ) -> Result<PerceptionResult, String> {
        if !self.is_active().await {
            return Err(
                "perception subsystem is not active"
                    .into(),
            );
        }

        let mut cycle =
            PerceptionCycle::new();

        cycle.input_count =
            request.inputs.len();

        cycle.status =
            PerceptionCycleStatus::Acquiring;

        {
            let mut value =
                self.inputs_received
                    .write()
                    .await;

            *value +=
                request.inputs.len() as u64;
        }

        let mut observations =
            Vec::new();

        let mut errors =
            Vec::new();

        for mut input
            in request.inputs
        {
            input.state =
                PerceptionInputState::Processing;

            if self.config
                .reject_empty_payloads
                && input.payload.data.is_empty()
            {
                input.state =
                    PerceptionInputState::Rejected;

                let error =
                    format!(
                        "empty perception payload from source {}",
                        input.source.name
                    );

                errors.push(error);

                let mut failed =
                    self.inputs_failed
                        .write()
                        .await;

                *failed += 1;

                continue;
            }

            if input.source.reliability
                < self.config
                    .minimum_source_reliability
            {
                let mut degraded =
                    self.degraded_sources
                        .write()
                        .await;

                *degraded += 1;
            }

            let content =
                payload_summary(
                    &input,
                );

            match PerceptualObservation::new(
                input.modality,
                content,
            ) {
                Ok(mut observation) => {
                    observation.status =
                        ObservationStatus::Processed;

                    observation.quality =
                        super::quality::DataQuality::calculate(
                            1.0,
                            1.0,
                            1.0,
                            1.0,
                            input.source.reliability,
                        );

                    observation
                        .provenance
                        .push(
                            super::provenance::PerceptionProvenance::direct(
                                input.source.id,
                                input.source.name.clone(),
                            ),
                        );

                    if self.config.enable_uncertainty {
                        let uncertainty =
                            1.0
                                - input
                                    .source
                                    .reliability
                                    .clamp(
                                        0.0,
                                        1.0,
                                    );

                        if let Ok(value) =
                            super::uncertainty::Uncertainty::new(
                                super::uncertainty::UncertaintyKind::Source,
                                uncertainty,
                            )
                        {
                            observation
                                .uncertainty
                                .push(value);
                        }
                    }

                    observations.push(
                        observation,
                    );

                    let mut processed =
                        self.inputs_processed
                            .write()
                            .await;

                    *processed += 1;

                    let mut created =
                        self.observations_created
                            .write()
                            .await;

                    *created += 1;
                }

                Err(error) => {
                    errors.push(error);

                    let mut failed =
                        self.inputs_failed
                            .write()
                            .await;

                    *failed += 1;
                }
            }
        }

        cycle.status =
            PerceptionCycleStatus::Processing;

        cycle.observation_count =
            observations.len();

        if request.require_fusion
            && self.config.enable_multimodal_fusion
        {
            cycle.status =
                PerceptionCycleStatus::Fusing;

            self.mark_fused(
                &mut observations,
            );
        }

        cycle.status =
            PerceptionCycleStatus::Interpreting;

        self.mark_interpreted(
            &mut observations,
        );

        let partial =
            !errors.is_empty();

        cycle.errors =
            errors.clone();

        cycle.complete(partial);

        Ok(PerceptionResult {
            request_id:
                request.request_id,

            cycle,

            observations,

            partial,

            errors,
        })
    }

    fn mark_fused(
        &self,
        observations:
            &mut [PerceptualObservation],
    ) {
        if observations.len() < 2 {
            return;
        }

        for observation
            in observations.iter_mut()
        {
            observation.status =
                ObservationStatus::Fused;
        }
    }

    fn mark_interpreted(
        &self,
        observations:
            &mut [PerceptualObservation],
    ) {
        for observation
            in observations.iter_mut()
        {
            observation.status =
                ObservationStatus::Interpreted;
        }
    }

    pub async fn health(
        &self,
    ) -> PerceptionHealth {
        let active =
            self.is_active().await;

        let inputs_received =
            *self.inputs_received
                .read()
                .await;

        let inputs_processed =
            *self.inputs_processed
                .read()
                .await;

        let inputs_failed =
            *self.inputs_failed
                .read()
                .await;

        let observations_created =
            *self.observations_created
                .read()
                .await;

        let anomalies_detected =
            *self.anomalies_detected
                .read()
                .await;

        let degraded_sources =
            *self.degraded_sources
                .read()
                .await;

        let state =
            if !active {
                PerceptionHealthState::Unavailable
            } else if inputs_failed > 0
                && inputs_processed == 0
            {
                PerceptionHealthState::Blocked
            } else if degraded_sources > 0 {
                PerceptionHealthState::Degraded
            } else {
                PerceptionHealthState::Healthy
            };

        PerceptionHealth {
            state,

            active,

            inputs_received,

            inputs_processed,

            inputs_failed,

            observations_created,

            anomalies_detected,

            degraded_sources,

            fusion_available:
                self.config
                    .enable_multimodal_fusion,

            multimodal_available:
                true,

            provenance_available:
                true,

            uncertainty_available:
                self.config
                    .enable_uncertainty,
        }
    }

    pub fn config(
        &self,
    ) -> &PerceptionConfig {
        &self.config
    }
}

fn payload_summary(
    input: &PerceptionInput,
) -> String {
    format!(
        "perception input: modality={:?}, payload={:?}, bytes={}, source={}",
        input.modality,
        input.payload.kind,
        input.payload.size_bytes,
        input.source.name
    )
}
