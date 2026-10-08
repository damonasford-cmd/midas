use std::sync::Arc;

use tokio::sync::RwLock;

use super::errors::{
    FoundationError,
    FoundationResult,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum LifecycleState {
    Created,
    Starting,
    Operational,
    Degraded,
    Pausing,
    Paused,
    Stopping,
    Stopped,
    Failed,
}

impl LifecycleState {
    pub fn can_transition_to(
        self,
        target: Self,
    ) -> bool {
        use LifecycleState::*;

        matches!(
            (self, target),
            (Created, Starting)
                | (Starting, Operational)
                | (Starting, Failed)
                | (Operational, Degraded)
                | (Operational, Pausing)
                | (Operational, Stopping)
                | (Degraded, Operational)
                | (Degraded, Pausing)
                | (Degraded, Stopping)
                | (Pausing, Paused)
                | (Pausing, Stopping)
                | (Paused, Operational)
                | (Paused, Stopping)
                | (Stopping, Stopped)
                | (Stopping, Failed)
        )
    }
}

#[derive(Debug, Clone)]
pub struct LifecycleController {
    state: Arc<RwLock<LifecycleState>>,
}

impl LifecycleController {
    pub fn new() -> Self {
        Self {
            state: Arc::new(
                RwLock::new(LifecycleState::Created)
            ),
        }
    }

    pub async fn state(&self) -> LifecycleState {
        *self.state.read().await
    }

    pub async fn transition(
        &self,
        target: LifecycleState,
    ) -> FoundationResult<()> {
        let mut current = self.state.write().await;

        if *current == target {
            return Ok(());
        }

        if !current.can_transition_to(target) {
            return Err(
                FoundationError::InvalidStateTransition {
                    from: format!("{:?}", *current),
                    to: format!("{:?}", target),
                },
            );
        }

        *current = target;

        Ok(())
    }

    pub async fn is_operational(&self) -> bool {
        matches!(
            self.state().await,
            LifecycleState::Operational
                | LifecycleState::Degraded
        )
    }
}

impl Default for LifecycleController {
    fn default() -> Self {
        Self::new()
    }
}
