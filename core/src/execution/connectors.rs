use crate::foundation::contracts::ActionIntent;
use anyhow::Result;

pub trait Connector: Send + Sync {
    fn name(&self) -> &str;

    fn is_available(&self) -> bool;

    fn execute(
        &self,
        action: &ActionIntent,
    ) -> Result<String>;
}

#[derive(Default)]
pub struct ConnectorRegistry {
    connectors: Vec<Box<dyn Connector>>,
}

impl ConnectorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        connector: Box<dyn Connector>,
    ) {
        self.connectors.push(connector);
    }

    pub fn available(&self) -> Vec<String> {
        self.connectors
            .iter()
            .filter(|connector| connector.is_available())
            .map(|connector| connector.name().to_string())
            .collect()
    }

    pub fn find(
        &self,
        name: &str,
    ) -> Option<&dyn Connector> {
        self.connectors
            .iter()
            .find(|connector| connector.name() == name)
            .map(|connector| connector.as_ref())
    }
}
