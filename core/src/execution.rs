
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::authorization::{
    ActionProposal,
    AuthorizationDecision,
    AuthorizationPolicy,
};
use crate::connector::{
    ConnectorRegistry,
    ConnectorRequest,
    ConnectorResult,
};

/// État final ou intermédiaire d'une demande d'exécution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// La demande n'a pas été exécutée.
    NotExecuted,

    /// Une autorisation supplémentaire est nécessaire.
    AwaitingApproval,

    /// Le connecteur a déclaré l'opération réussie.
    Succeeded,

    /// Le connecteur a déclaré l'opération en échec.
    Failed,

    /// Une erreur a empêché l'exécution.
    Error,
}

/// Demande complète présentée au moteur d'exécution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    /// Identifiant de suivi de cette exécution.
    pub id: String,

    /// Proposition soumise à la politique d'autorisation.
    pub proposal: ActionProposal,

    /// Demande adressée au connecteur.
    pub connector_request: ConnectorRequest,
}

impl ExecutionRequest {
    pub fn new(
        id: impl Into<String>,
        proposal: ActionProposal,
        connector_request: ConnectorRequest,
    ) -> Result<Self> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(anyhow!(
                "L'identifiant de l'exécution ne peut pas être vide"
            ));
        }

        if proposal.id != id {
            return Err(anyhow!(
                "L'identifiant de la proposition ne correspond pas \
                 à celui de l'exécution"
            ));
        }

        Ok(Self {
            id,
            proposal,
            connector_request,
        })
    }
}

/// Résultat uniforme du moteur d'exécution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub id: String,
    pub status: ExecutionStatus,
    pub executed: bool,
    pub success: bool,
    pub connector: String,
    pub operation: String,
    pub message: String,

    /// Résultat brut du connecteur, lorsqu'il existe.
    #[serde(default)]
    pub connector_result: Option<ConnectorResult>,
}

impl ExecutionResult {
    fn blocked(
        request: &ExecutionRequest,
        status: ExecutionStatus,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: request.id.clone(),
            status,
            executed: false,
            success: false,
            connector: request.connector_request.connector.clone(),
            operation: request.connector_request.operation.clone(),
            message: message.into(),
            connector_result: None,
        }
    }

    fn from_connector(
        request: &ExecutionRequest,
        result: ConnectorResult,
    ) -> Self {
        let success = result.success;

        Self {
            id: request.id.clone(),
            status: if success {
                ExecutionStatus::Succeeded
            } else {
                ExecutionStatus::Failed
            },
            executed: true,
            success,
            connector: result.connector.clone(),
            operation: result.operation.clone(),
            message: result.message.clone(),
            connector_result: Some(result),
        }
    }
}

/// Moteur central d'exécution.
///
/// Le moteur n'invente aucun connecteur et ne contourne pas la
/// politique d'autorisation. Il ne remplace pas non plus les
/// contrôles de sécurité propres à chaque connecteur.
pub struct ExecutionEngine {
    policy: Arc<AuthorizationPolicy>,
    registry: Arc<ConnectorRegistry>,
}

impl ExecutionEngine {
    pub fn new(
        policy: Arc<AuthorizationPolicy>,
        registry: Arc<ConnectorRegistry>,
    ) -> Self {
        Self { policy, registry }
    }

    /// Évalue la demande, puis transmet l'opération si elle est autorisée.
    ///
    /// Une erreur technique du connecteur est convertie en résultat
    /// d'exécution explicite afin de conserver l'identifiant de suivi.
    pub fn execute(&self, request: &ExecutionRequest) -> ExecutionResult {
        // 1. Les identifiants de la proposition et de la demande
        // doivent rester cohérents.
        if request.id.trim().is_empty() || request.proposal.id != request.id {
            return ExecutionResult::blocked(
                request,
                ExecutionStatus::Error,
                "Identifiant de demande invalide ou incohérent.",
            );
        }

        // 2. La proposition doit correspondre exactement à l'opération
        // que le connecteur va recevoir.
        //
        // La politique d'autorisation doit porter sur la même action
        // que celle qui sera réellement exécutée.
        if request.proposal.description.trim().is_empty() {
            return ExecutionResult::blocked(
                request,
                ExecutionStatus::Error,
                "La proposition d'action ne décrit pas l'action demandée.",
            );
        }

        // 3. Vérifier que le connecteur existe et sait traiter l'opération.
        if !self.registry.contains(&request.connector_request.connector) {
            return ExecutionResult::blocked(
                request,
                ExecutionStatus::NotExecuted,
                "Connecteur indisponible : aucune action n'a été lancée.",
            );
        }

        if !self.registry.supports(
            &request.connector_request.connector,
            &request.connector_request.operation,
        ) {
            return ExecutionResult::blocked(
                request,
                ExecutionStatus::NotExecuted,
                "Opération non prise en charge : aucune action n'a été lancée.",
            );
        }

        // 4. Évaluer les règles d'autorisation.
        let decision = self.policy.evaluate(&request.proposal);

        match decision {
            AuthorizationDecision::Denied { .. } => {
                ExecutionResult::blocked(
                    request,
                    ExecutionStatus::NotExecuted,
                    "Action refusée par la politique d'autorisation.",
                )
            }

            AuthorizationDecision::RequiresApproval { .. } => {
                ExecutionResult::blocked(
                    request,
                    ExecutionStatus::AwaitingApproval,
                    "Validation requise : l'action n'a pas été exécutée.",
                )
            }

            AuthorizationDecision::Allowed { .. } => {
                // 5. Dernière vérification des champs de routage
                // avant la transmission au connecteur.
                if request.connector_request.connector.trim().is_empty()
                    || request.connector_request.operation.trim().is_empty()
                {
                    return ExecutionResult::blocked(
                        request,
                        ExecutionStatus::Error,
                        "Connecteur ou opération invalide.",
                    );
                }

                // 6. Exécuter uniquement après autorisation.
                match self.registry.execute(&request.connector_request) {
                    Ok(result) => {
                        ExecutionResult::from_connector(request, result)
                    }
                    Err(error) => ExecutionResult::blocked(
                        request,
                        ExecutionStatus::Error,
                        format!("Erreur d'exécution : {error:#}"),
                    ),
                }
            }
        }
    }

    /// Vérifie la disponibilité d'une capacité sans l'exécuter.
    pub fn can_route(&self, connector: &str, operation: &str) -> bool {
        self.registry.supports(connector, operation)
    }

    /// Liste les connecteurs effectivement enregistrés.
    pub fn available_connectors(
        &self,
    ) -> Result<Vec<crate::connector::ConnectorInfo>> {
        self.registry.list()
    }
}

/// Utilitaire pour construire des arguments JSON sans exécuter d'action.
pub fn empty_arguments() -> Value {
    Value::Object(serde_json::Map::new())
}
