use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    action::{
        ActionAuthorization,
        ActionResult,
    },
    connector::{
        ConnectorRegistry,
        ConnectorRequest,
        ConnectorResult,
    },
    decision::Decision,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub id: Uuid,
    pub connector: String,
    pub operation: String,
    pub arguments: Vec<String>,
}

impl ExecutionRequest {
    pub fn new(
        connector: impl Into<String>,
        operation: impl Into<String>,
        arguments: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            connector: connector.into(),
            operation: operation.into(),
            arguments,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub id: Uuid,
    pub success: bool,
    pub executed: bool,
    pub connector: Option<String>,
    pub operation: Option<String>,
    pub message: String,
    pub connector_result: Option<ConnectorResult>,
}

impl ExecutionResult {
    pub fn not_executed(
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            success: false,
            executed: false,
            connector: None,
            operation: None,
            message: message.into(),
            connector_result: None,
        }
    }

    pub fn executed(
        request: &ExecutionRequest,
        result: ConnectorResult,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            success: result.success,
            executed: true,
            connector: Some(
                request.connector.clone(),
            ),
            operation: Some(
                request.operation.clone(),
            ),
            message: result.message.clone(),
            connector_result: Some(result),
        }
    }
}

pub struct ExecutionEngine;

impl ExecutionEngine {
    pub fn execute(
        registry: &ConnectorRegistry,
        decision: &Decision,
        action: &ActionResult,
        request: &ExecutionRequest,
    ) -> Result<ExecutionResult> {
        if action.authorization.decision
            == crate::action::AuthorizationDecision::Denied
        {
            return Ok(
                ExecutionResult::not_executed(
                    "Exécution refusée par le contrôle d'autorisation.",
                ),
            );
        }

        if action.approval.is_some() {
            return Ok(
                ExecutionResult::not_executed(
                    "Exécution suspendue : une validation est requise.",
                ),
            );
        }

        if !decision.executable {
            return Ok(
                ExecutionResult::not_executed(
                    "La décision n'est pas exécutable.",
                ),
            );
        }

        if !Self::authorization_is_valid(
            action,
        ) {
            return Ok(
                ExecutionResult::not_executed(
                    "L'autorisation de l'action n'est pas valide.",
                ),
            );
        }

        let connector_result =
            registry
                .execute(
                    &ConnectorRequest::new(
                        request.connector.clone(),
                        request.operation.clone(),
                        request.arguments.clone(),
                        Default::default(),
                    ),
                )
                .with_context(|| {
                    format!(
                        "Échec d'exécution via le connecteur {}",
                        request.connector
                    )
                })?;

        Ok(
            ExecutionResult::executed(
                request,
                connector_result,
            ),
        )
    }

    fn authorization_is_valid(
        action: &ActionResult,
    ) -> bool {
        matches!(
            action.authorization.decision,
            crate::action::AuthorizationDecision::Allowed
        )
    }
}
