use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    process::Command,
};

use crate::action::ActionRiskProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorRequest {
    pub connector: String,
    pub operation: String,
    pub arguments: Vec<String>,
    pub profile: ActionRiskProfile,
}

impl ConnectorRequest {
    pub fn new(
        connector: impl Into<String>,
        operation: impl Into<String>,
        arguments: Vec<String>,
        profile: ActionRiskProfile,
    ) -> Self {
        Self {
            connector: connector.into(),
            operation: operation.into(),
            arguments,
            profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorResult {
    pub connector: String,
    pub operation: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub message: String,
}

impl ConnectorResult {
    pub fn success(
        connector: impl Into<String>,
        operation: impl Into<String>,
        stdout: impl Into<String>,
        exit_code: Option<i32>,
    ) -> Self {
        Self {
            connector: connector.into(),
            operation: operation.into(),
            success: true,
            exit_code,
            stdout: stdout.into(),
            stderr: String::new(),
            message: "Connecteur exécuté avec succès.".to_string(),
        }
    }

    pub fn failure(
        connector: impl Into<String>,
        operation: impl Into<String>,
        message: impl Into<String>,
        stderr: impl Into<String>,
        exit_code: Option<i32>,
    ) -> Self {
        Self {
            connector: connector.into(),
            operation: operation.into(),
            success: false,
            exit_code,
            stdout: String::new(),
            stderr: stderr.into(),
            message: message.into(),
        }
    }
}

pub trait Connector: Send + Sync {
    fn name(&self) -> &str;

    fn can_handle(
        &self,
        request: &ConnectorRequest,
    ) -> bool;

    fn execute(
        &self,
        request: &ConnectorRequest,
    ) -> Result<ConnectorResult>;
}

pub struct ConnectorRegistry {
    connectors: HashMap<String, Box<dyn Connector>>,
}

impl Default for ConnectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectorRegistry {
    pub fn new() -> Self {
        Self {
            connectors: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        connector: Box<dyn Connector>,
    ) {
        let name =
            connector.name().to_string();

        self.connectors.insert(
            name,
            connector,
        );
    }

    pub fn contains(
        &self,
        name: &str,
    ) -> bool {
        self.connectors.contains_key(name)
    }

    pub fn names(&self) -> Vec<String> {
        self.connectors
            .keys()
            .cloned()
            .collect()
    }

    pub fn execute(
        &self,
        request: &ConnectorRequest,
    ) -> Result<ConnectorResult> {
        let connector =
            self.connectors
                .get(&request.connector)
                .with_context(|| {
                    format!(
                        "Connecteur introuvable : {}",
                        request.connector
                    )
                })?;

        if !connector.can_handle(request) {
            anyhow::bail!(
                "Le connecteur {} ne peut pas gérer l'opération {}",
                request.connector,
                request.operation
            );
        }

        connector.execute(request)
    }
}

pub struct LocalProcessConnector {
    name: String,
    allowed_programs: Vec<String>,
}

impl LocalProcessConnector {
    pub fn new(
        allowed_programs: Vec<String>,
    ) -> Self {
        Self {
            name: "local_process".to_string(),
            allowed_programs,
        }
    }

    fn is_allowed(
        &self,
        program: &str,
    ) -> bool {
        self.allowed_programs
            .iter()
            .any(|allowed| allowed == program)
    }
}

impl Connector for LocalProcessConnector {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_handle(
        &self,
        request: &ConnectorRequest,
    ) -> bool {
        if request.operation.trim().is_empty() {
            return false;
        }

        self.is_allowed(
            &request.operation,
        )
    }

    fn execute(
        &self,
        request: &ConnectorRequest,
    ) -> Result<ConnectorResult> {
        if !self.can_handle(request) {
            anyhow::bail!(
                "Programme non autorisé : {}",
                request.operation
            );
        }

        /*
         * Les arguments sont transmis directement
         * au programme.
         *
         * Aucun shell n'est utilisé.
         */
        let output =
            Command::new(
                &request.operation,
            )
            .args(&request.arguments)
            .output()
            .with_context(|| {
                format!(
                    "Impossible de lancer le programme {}",
                    request.operation
                )
            })?;

        let stdout =
            String::from_utf8_lossy(
                &output.stdout,
            )
            .to_string();

        let stderr =
            String::from_utf8_lossy(
                &output.stderr,
            )
            .to_string();

        let exit_code =
            output.status.code();

        if output.status.success() {
            Ok(
                ConnectorResult::success(
                    &self.name,
                    &request.operation,
                    stdout,
                    exit_code,
                ),
            )
        } else {
            Ok(
                ConnectorResult::failure(
                    &self.name,
                    &request.operation,
                    "Le programme s'est terminé avec une erreur.",
                    stderr,
                    exit_code,
                ),
            )
        }
    }
}
