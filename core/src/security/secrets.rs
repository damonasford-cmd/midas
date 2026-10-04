use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct SecretStore {
    secrets: Arc<RwLock<HashMap<String, String>>>,
}

impl SecretStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(
        &self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<()> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(anyhow!("secret name cannot be empty"));
        }

        let mut secrets = self
            .secrets
            .write()
            .map_err(|_| anyhow!("secret store lock poisoned"))?;

        secrets.insert(name, value.into());

        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Option<String>> {
        let secrets = self
            .secrets
            .read()
            .map_err(|_| anyhow!("secret store lock poisoned"))?;

        Ok(secrets.get(name).cloned())
    }

    pub fn contains(&self, name: &str) -> Result<bool> {
        let secrets = self
            .secrets
            .read()
            .map_err(|_| anyhow!("secret store lock poisoned"))?;

        Ok(secrets.contains_key(name))
    }

    pub fn remove(&self, name: &str) -> Result<bool> {
        let mut secrets = self
            .secrets
            .write()
            .map_err(|_| anyhow!("secret store lock poisoned"))?;

        Ok(secrets.remove(name).is_some())
    }

    pub fn names(&self) -> Result<Vec<String>> {
        let secrets = self
            .secrets
            .read()
            .map_err(|_| anyhow!("secret store lock poisoned"))?;

        Ok(secrets.keys().cloned().collect())
    }
}
