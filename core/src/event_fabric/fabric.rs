use std::sync::Arc;

use super::{
    dead_letter::DeadLetterQueue,
    deduplication::{DeduplicationDecision, DeduplicationStore},
    dispatcher::{
        DispatchError,
        DispatchOutcome,
        EventHandlerRegistry,
    },
    envelope::EventEnvelope,
    health::{
        EventFabricHealth,
        EventFabricHealthState,
    },
    metrics::EventFabricMetrics,
    retry::{RetryDecision, RetryPolicy},
    router::EventRouter,
    store::EventStore,
    subscription::{
        EventSubscription,
        SubscriptionRegistry,
    },
};

#[derive(Debug, Clone)]
pub struct EventFabricConfig {
    pub max_in_flight: usize,
    pub retry_policy: RetryPolicy,
    pub persist_events: bool,
}

impl Default for EventFabricConfig {
    fn default() -> Self {
        Self {
            max_in_flight: 1024,
            retry_policy: RetryPolicy::default(),
            persist_events: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventFabricState {
    Created,
    Running,
    Degraded,
    Stopped,
}

pub struct EventFabric {
    config: EventFabricConfig,

    state: EventFabricState,

    router: EventRouter,

    subscriptions: SubscriptionRegistry,

    handlers: EventHandlerRegistry,

    deduplication: DeduplicationStore,

    store: EventStore,

    dead_letters: DeadLetterQueue,

    metrics: Arc<EventFabricMetrics>,
}

impl EventFabric {
    pub fn new(config: EventFabricConfig) -> Self {
        Self {
            config,
            state: EventFabricState::Created,
            router: EventRouter::default(),
            subscriptions: SubscriptionRegistry::default(),
            handlers: EventHandlerRegistry::default(),
            deduplication: DeduplicationStore::default(),
            store: EventStore::default(),
            dead_letters: DeadLetterQueue::default(),
            metrics: Arc::new(EventFabricMetrics::default()),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        match self.state {
            EventFabricState::Created
            | EventFabricState::Stopped
            | EventFabricState::Degraded => {
                self.state = EventFabricState::Running;
                Ok(())
            }

            EventFabricState::Running => Ok(()),
        }
    }

    pub fn stop(&mut self) {
        self.state = EventFabricState::Stopped;
    }

    pub fn state(&self) -> EventFabricState {
        self.state
    }

    pub fn subscribe(
        &mut self,
        subscription: EventSubscription,
    ) {
        self.subscriptions.register(subscription);
    }

    pub async fn publish(
        &self,
        envelope: EventEnvelope,
    ) -> Result<PublishResult, EventFabricError> {
        if self.state != EventFabricState::Running {
            return Err(EventFabricError::NotRunning);
        }

        if envelope.event.validate().is_err() {
            return Err(EventFabricError::InvalidEvent);
        }

        if envelope.attempt as usize >= self.config.max_in_flight {
            return Err(EventFabricError::CapacityExceeded);
        }

        match self
            .deduplication
            .check_and_record(envelope.id())
            .await
        {
            DeduplicationDecision::Duplicate => {
                self.metrics.record_duplicate();

                return Ok(PublishResult::Duplicate);
            }

            DeduplicationDecision::New => {}
        }

        if self.config.persist_events {
            self.store.append(envelope.clone()).await;
        }

        self.metrics.record_published();

        let routing = self
            .router
            .route(&envelope, &self.subscriptions);

        if routing.routes.is_empty() {
            return Ok(PublishResult::Accepted {
                dispatched: 0,
            });
        }

        let mut dispatched = 0usize;

        for route in routing.routes {
            match self
                .handlers
                .dispatch(&route.subscriber, envelope.clone())
                .await
            {
                Ok(DispatchOutcome::Handled)
                | Ok(DispatchOutcome::Ignored) => {
                    dispatched += 1;
                    self.metrics.record_dispatched();
                }

                Err(error) => {
                    self.metrics.record_failure();

                    match self
                        .config
                        .retry_policy
                        .should_retry(envelope.attempt)
                    {
                        true => {
                            let retry =
                                envelope.retry();

                            let delay = self
                                .config
                                .retry_policy
                                .delay_ms(retry.attempt);

                            tokio::time::sleep(
                                std::time::Duration::from_millis(delay),
                            )
                            .await;

                            match self
                                .handlers
                                .dispatch(
                                    &route.subscriber,
                                    retry,
                                )
                                .await
                            {
                                Ok(_) => {
                                    dispatched += 1;
                                    self.metrics.record_dispatched();
                                }

                                Err(retry_error) => {
                                    self.handle_failure(
                                        envelope.clone(),
                                        retry_error,
                                    )
                                    .await;
                                }
                            }
                        }

                        false => {
                            self.handle_failure(
                                envelope.clone(),
                                error,
                            )
                            .await;
                        }
                    }
                }
            }
        }

        Ok(PublishResult::Accepted { dispatched })
    }

    async fn handle_failure(
        &self,
        envelope: EventEnvelope,
        error: DispatchError,
    ) {
        self.metrics.record_dead_letter();

        self.dead_letters
            .push(
                envelope,
                error.to_string(),
            )
            .await;
    }

    pub fn handlers(&self) -> EventHandlerRegistry {
        self.handlers.clone()
    }

    pub fn store(&self) -> EventStore {
        self.store.clone()
    }

    pub fn dead_letters(&self) -> DeadLetterQueue {
        self.dead_letters.clone()
    }

    pub fn metrics(&self) -> Arc<EventFabricMetrics> {
        self.metrics.clone()
    }

    pub async fn health(&self) -> EventFabricHealth {
        let stored = self.store.len().await;
        let dead_letters = self.dead_letters.len().await;
        let metrics = self.metrics.snapshot();

        let state = if self.state == EventFabricState::Stopped {
            EventFabricHealthState::Blocked
        } else if metrics.failures > 0 || dead_letters > 0 {
            EventFabricHealthState::Degraded
        } else {
            EventFabricHealthState::Healthy
        };

        EventFabricHealth {
            state,
            queued_events: 0,
            stored_events: stored,
            dead_letters,
            duplicate_events: metrics.duplicates as usize,
            failed_dispatches: metrics.failures as usize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishResult {
    Accepted {
        dispatched: usize,
    },
    Duplicate,
}

#[derive(Debug, thiserror::Error)]
pub enum EventFabricError {
    #[error("event fabric is not running")]
    NotRunning,

    #[error("event is invalid")]
    InvalidEvent,

    #[error("event fabric capacity exceeded")]
    CapacityExceeded,
}

impl From<EventFabricError> for anyhow::Error {
    fn from(error: EventFabricError) -> Self {
        anyhow::anyhow!(error.to_string())
    }
}
