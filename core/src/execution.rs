use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    action::{
        ActionResult,
        ActionRiskProfile,
        AuthorizationDecision,
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
    pub profile: ActionRiskProfile,
}

impl ExecutionRequest {
    pub fn new(
        connector: impl Into<String>,
        operation: impl Into<String>,
        arguments: Vec<String>,
        profile: ActionRiskProfile,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            connector: connector.into(),
            operation: operation.into(),
            arguments,
            profile,
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

    pub fn failed(
        request: &ExecutionRequest,
        message: impl Into<String>,
        result: Option<ConnectorResult>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            success: false,
            executed: false,
            connector: Some(
                request.connector.clone(),
            ),
            operation: Some(
                request.operation.clone(),
            ),
            message: message.into(),
            connector_result: result,
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
        /*
         * ============================================================
         * 1. VÉRIFICATION DE L'AUTORISATION
         * ============================================================
         */

        match action.authorization.decision {
            AuthorizationDecision::Denied => {
                return Ok(
                    ExecutionResult::not_executed(
                        "Exécution refusée par le contrôle d'autorisation.",
                    ),
                );
            }

            AuthorizationDecision::RequiresApproval => {
                return Ok(
                    ExecutionResult::not_executed(
                        "Exécution suspendue : validation requise.",
                    ),
                );
            }

            AuthorizationDecision::Allowed => {}
        }

        /*
         * ============================================================
         * 2. VÉRIFICATION DE L'APPROBATION
         * ============================================================
         */

        if action.approval.is_some() {
            return Ok(
                ExecutionResult::not_executed(
                    "Exécution suspendue : une demande d'approbation existe.",
                ),
            );
        }

        /*
         * ============================================================
         * 3. VÉRIFICATION DE LA DÉCISION
         * ============================================================
         */

        if !decision.executable {
            return Ok(
                ExecutionResult::not_executed(
                    "La décision n'est pas exécutable.",
                ),
            );
        }

        /*
         * ============================================================
         * 4. VÉRIFICATION DU PROFIL
         * ============================================================
         *
         * Le profil doit rester identique de la décision jusqu'au
         * connecteur. Toute modification entraîne l'arrêt.
         */

        if action.profile != request.profile {
            return Ok(
                ExecutionResult::not_executed(
                    "Le profil de risque de l'exécution ne correspond pas au profil de l'action.",
                ),
            );
        }

        /*
         * ============================================================
         * 5. VÉRIFICATION DU CONNECTEUR
         * ============================================================
         */

        if !registry.contains(
            &request.connector,
        ) {
            return Ok(
                ExecutionResult::not_executed(
                    format!(
                        "Connecteur introuvable : {}",
                        request.connector
                    ),
                ),
            );
        }

        /*
         * ============================================================
         * 6. CONSTRUCTION DE LA REQUÊTE CONNECTEUR
         * ============================================================
         */

        let connector_request =
            ConnectorRequest::new(
                request.connector.clone(),
                request.operation.clone(),
                request.arguments.clone(),
                request.profile.clone(),
            );

        /*
         * ============================================================
         * 7. EXÉCUTION RÉELLE
         * ============================================================
         */

        let connector_result =
            registry
                .execute(
                    &connector_request,
                )
                .with_context(|| {
                    format!(
                        "Échec d'exécution via le connecteur {}",
                        request.connector
                    )
                })?;

        /*
         * ============================================================
         * 8. RETOUR
         * ============================================================
         */

        if connector_result.success {
            Ok(
                ExecutionResult::executed(
                    request,
                    connector_result,
                ),
            )
        } else {
            let message =
                connector_result
                    .message
                    .clone();

            Ok(
                ExecutionResult::failed(
                    request,
                    message,
                    Some(connector_result),
                ),
            )
        }
    }

    pub fn can_execute(
        action: &ActionResult,
        decision: &Decision,
    ) -> bool {
        if !decision.executable {
            return false;
        }

        if action.approval.is_some() {
            return false;
        }

        matches!(
            action.authorization.decision,
            AuthorizationDecision::Allowed
        )
    }
}
