use crate::execution::physical::PhysicalDriver;
use crate::foundation::contracts::ActionIntent;
use anyhow::{anyhow, Result};

pub trait RobotController: PhysicalDriver {
    fn emergency_stop(&self) -> Result<()>;

    fn reset_safely(&self) -> Result<()>;

    fn status(&self) -> Result<String>;
}

#[derive(Debug, Clone, Default)]
pub struct RoboticsSafety;

impl RoboticsSafety {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_action(
        &self,
        action: &ActionIntent,
    ) -> Result<()> {
        if action.impact == crate::foundation::contracts::ActionImpact::Critical {
            return Err(anyhow!(
                "critical robotic action requires an authorized safety controller"
            ));
        }

        Ok(())
    }
}
