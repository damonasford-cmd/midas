use serde::{Deserialize, Serialize};

use super::errors::{FoundationError, FoundationResult};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct LifecycleController {
    state: LifecycleState,
}

impl Default for LifecycleController {
    fn default() -> Self {
        Self {
            state: LifecycleState::Created,
        }
    }
}

impl LifecycleController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> LifecycleState {
        self.state
    }

    pub fn start(&mut self) -> FoundationResult<()> {
        match self.state {
            LifecycleState::Created
            | LifecycleState::Stopped => {
                self.state = LifecycleState::Starting;
                self.state = LifecycleState::Operational;
                Ok(())
            }

            LifecycleState::Starting
            | LifecycleState::Operational => Ok(()),

            LifecycleState::Degraded => {
                self.state = LifecycleState::Operational;
                Ok(())
            }

            _ => Err(FoundationError::InvalidStateTransition(
                format!("cannot start from {:?}", self.state),
            )),
        }
    }

    pub fn degrade(&mut self) {
        self.state = LifecycleState::Degraded;
    }

    pub fn pause(&mut self) -> FoundationResult<()> {
        if self.state != LifecycleState::Operational {
            return Err(FoundationError::InvalidStateTransition(
                format!("cannot pause from {:?}", self.state),
            ));
        }

        self.state = LifecycleState::Pausing;
        self.state = LifecycleState::Paused;

        Ok(())
    }

    pub fn resume(&mut self) -> FoundationResult<()> {
        match self.state {
            LifecycleState::Paused | LifecycleState::Degraded => {
                self.state = LifecycleState::Operational;
                Ok(())
            }

            LifecycleState::Operational => Ok(()),

            _ => Err(FoundationError::InvalidStateTransition(
                format!("cannot resume from {:?}", self.state),
            )),
        }
    }

    pub fn stop(&mut self) -> FoundationResult<()> {
        match self.state {
            LifecycleState::Stopped => Ok(()),

            LifecycleState::Created => {
                self.state = LifecycleState::Stopped;
                Ok(())
            }

            LifecycleState::Stopping => Ok(()),

            _ => {
                self.state = LifecycleState::Stopping;
                self.state = LifecycleState::Stopped;
                Ok(())
            }
        }
    }

    pub fn fail(&mut self) {
        self.state = LifecycleState::Failed;
    }

    pub fn is_operational(&self) -> bool {
        self.state == LifecycleState::Operational
    }

    pub fn can_accept_work(&self) -> bool {
        matches!(
            self.state,
            LifecycleState::Operational | LifecycleState::Degraded
        )
    }
}
