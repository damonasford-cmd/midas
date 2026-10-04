use crate::foundation::contracts::{
    ActionIntent,
    ActionResult,
};
use anyhow::{anyhow, Result};
use chrono::Utc;
use std::collections::HashMap;

pub trait ActionAdapter: Send + Sync {
    fn name(&self) -> &str;

    fn execute(
        &self,
        action: &ActionIntent,
    ) -> Result<String>;
}

#[derive(Default)]
pub struct Executor {
    adapters: HashMap<String, Box<dyn ActionAdapter>>,
}

impl Executor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_adapter(
        &mut self,
        adapter: Box<dyn ActionAdapter>,
    ) {
        self.adapters
            .insert(adapter.name().to_string(), adapter);
    }

    pub fn execute(
        &self,
        action: &ActionIntent,
    ) -> Result<ActionResult> {
        let started_at = Utc::now();

        let adapter = self
            .adapters
            .get(&action.domain.to_string())
            .ok_or_else(|| {
                anyhow!(
                    "no execution adapter registered for domain {:?}",
                    action.domain
                )
            })?;

        match adapter.execute(action) {
            Ok(output) => Ok(ActionResult {
                action_id: action.id,
                started_at,
                completed_at: Utc::now(),
                success: true,
                output: Some(output),
                error: None,
                observations: Vec::new(),
            }),

            Err(error) => Ok(ActionResult {
                action_id: action.id,
                started_at,
                completed_at: Utc::now(),
                success: false,
                output: None,
                error: Some(error.to_string()),
                observations: Vec::new(),
            }),
        }
    }
}
