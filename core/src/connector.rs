
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Demande normalisée transmise à un connecteur réel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorRequest {
    /// Nom du connecteur ciblé.
    pub connector: String,

    /// Opération demandée au connecteur.
    pub operation: String,

    /// Arguments structurés de l'opération.
    pub arguments: Value,
}

impl ConnectorRequest {
    pub fn new(
        connector: impl Into<String>,
        operation: impl Into<String>,
        arguments: Value,
    ) -> Result<Self> {
        let connector = connector.into();
        let operation = operation.into();

        validate_identifier(&connector, "connector")?;
        validate_identifier(&operation, "operation")?;

        Ok(Self {
            connector,
            operation,
            arguments,
        })
    }
}

/// Résultat structuré renvoyé par un connecteur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorResult {
    /// Connecteur ayant traité la demande.
    pub connector: String,

    /// Opération concernée.
    pub operation: String,

    /// Indique si le connecteur déclare l'opération réussie.
    ///
    /// Cette valeur ne constitue pas, à elle seule, une preuve
    /// indépendante que le résultat annoncé est exact.
    pub success: bool,

    /// Message lisible décrivant le résultat.
    pub message: String,

    /// Données produites par l'opération, si disponibles.
    #[serde(default)]
    pub data: Option<Value>,
}

impl ConnectorResult {
    pub fn success(
        connector: impl Into<String>,
        operation: impl Into<String>,
        message: impl Into<String>,
        data: Option<Value>,
    ) -> Self {
        Self {
            connector: connector.into(),
            operation: operation.into(),
            success: true,
            message: message.into(),
            data,
        }
    }

    pub fn failure(
        connector: impl Into<String>,
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            connector: connector.into(),
            operation: operation.into(),
            success: false,
            message: message.into(),
            data: None,
        }
    }
}

/// Contrat commun des connecteurs de MIDAS.
///
/// Chaque implémentation doit vérifier elle-même les arguments,
/// les permissions et les limites propres à l'opération.
///
/// L'autorisation globale doit être contrôlée par le moteur
/// d'exécution avant tout appel à `execute`.
pub trait Connector: Send + Sync {
    /// Identifiant stable, unique et sensible à la casse.
    fn name(&self) -> &str;

    /// Description des capacités réelles du connecteur.
    fn description(&self) -> &str;

    /// Indique si cette opération est prise en charge.
    fn supports(&self, operation: &str) -> bool;

    /// Exécute l'opération.
    ///
    /// Une erreur technique est renvoyée sous forme de `Err`.
    /// Une opération traitée mais échouée peut renvoyer
    /// `Ok(ConnectorResult::failure(...))`.
    fn execute(&self, request: &ConnectorRequest) -> Result<ConnectorResult>;
}

/// Métadonnées publiques d'un connecteur enregistré.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorInfo {
    pub name: String,
    pub description: String,
}

/// Registre thread-safe des connecteurs disponibles.
///
/// Le registre ne crée pas de capacités fictives : seuls les
/// connecteurs explicitement enregistrés sont disponibles.
#[derive(Default)]
pub struct ConnectorRegistry {
    connectors: RwLock<HashMap<String, Arc<dyn Connector>>>,
}

impl ConnectorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre un connecteur.
    ///
    /// Les doublons sont refusés pour éviter qu'une nouvelle
    /// implémentation remplace silencieusement une capacité existante.
    pub fn register<C>(&self, connector: C) -> Result<()>
    where
        C: Connector + 'static,
    {
        self.register_shared(Arc::new(connector))
    }

    /// Enregistre un connecteur déjà partagé.
    pub fn register_shared(
        &self,
        connector: Arc<dyn Connector>,
    ) -> Result<()> {
        let name = connector.name().to_owned();

        validate_identifier(&name, "connector name")?;

        if connector.description().trim().is_empty() {
            return Err(anyhow!(
                "Impossible d'enregistrer le connecteur '{name}' : \
                 sa description est vide"
            ));
        }

        let mut connectors = self
            .connectors
            .write()
            .map_err(|_| anyhow!("Verrou du registre des connecteurs empoisonné"))?;

        if connectors.contains_key(&name) {
            return Err(anyhow!(
                "Le connecteur '{name}' est déjà enregistré"
            ));
        }

        connectors.insert(name, connector);
        Ok(())
    }

    /// Récupère un connecteur enregistré.
    ///
    /// `None` signifie que ce nom n'est pas disponible.
    pub fn get(&self, name: &str) -> Option<Arc<dyn Connector>> {
        let connectors = self.connectors.read().ok()?;
        connectors.get(name).cloned()
    }

    /// Vérifie la présence d'un connecteur.
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// Renvoie les informations des connecteurs enregistrés.
    pub fn list(&self) -> Result<Vec<ConnectorInfo>> {
        let connectors = self
            .connectors
            .read()
            .map_err(|_| anyhow!("Verrou du registre des connecteurs empoisonné"))?;

        let mut items = connectors
            .values()
            .map(|connector| ConnectorInfo {
                name: connector.name().to_owned(),
                description: connector.description().to_owned(),
            })
            .collect::<Vec<_>>();

        items.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(items)
    }

    /// Vérifie si une opération est prise en charge.
    pub fn supports(&self, connector_name: &str, operation: &str) -> bool {
        self.get(connector_name)
            .map(|connector| connector.supports(operation))
            .unwrap_or(false)
    }

    /// Transmet une demande au connecteur correspondant.
    ///
    /// Cette méthode ne remplace pas l'autorisation du moteur
    /// d'exécution : les appels doivent être autorisés en amont.
    pub fn execute(&self, request: &ConnectorRequest) -> Result<ConnectorResult> {
        validate_identifier(&request.connector, "connector")?;
        validate_identifier(&request.operation, "operation")?;

        let connector = self
            .get(&request.connector)
            .ok_or_else(|| {
                anyhow!(
                    "Connecteur indisponible : '{}'",
                    request.connector
                )
            })?;

        if !connector.supports(&request.operation) {
            return Err(anyhow!(
                "L'opération '{}' n'est pas prise en charge par '{}'",
                request.operation,
                request.connector
            ));
        }

        let result = connector
            .execute(request)
            .with_context(|| {
                format!(
                    "Échec technique du connecteur '{}' pour '{}'",
                    request.connector, request.operation
                )
            })?;

        // Empêche un connecteur de renvoyer silencieusement un résultat
        // attribué à un autre connecteur ou à une autre opération.
        if result.connector != request.connector {
            return Err(anyhow!(
                "Résultat incohérent : connecteur attendu '{}', reçu '{}'",
                request.connector,
                result.connector
            ));
        }

        if result.operation != request.operation {
            return Err(anyhow!(
                "Résultat incohérent : opération attendue '{}', reçue '{}'",
                request.operation,
                result.operation
            ));
        }

        Ok(result)
    }

    /// Nombre de connecteurs actuellement enregistrés.
    pub fn len(&self) -> Result<usize> {
        let connectors = self
            .connectors
            .read()
            .map_err(|_| anyhow!("Verrou du registre des connecteurs empoisonné"))?;

        Ok(connectors.len())
    }

    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }
}

fn validate_identifier(value: &str, field: &str) -> Result<()> {
    let value = value.trim();

    if value.is_empty() {
        return Err(anyhow!("{field} ne peut pas être vide"));
    }

    if value.len() > 128 {
        return Err(anyhow!(
            "{field} dépasse la longueur maximale de 128 octets"
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(anyhow!(
            "{field} contient des caractères de contrôle interdits"
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct TestConnector;

    impl Connector for TestConnector {
        fn name(&self) -> &str {
            "test"
        }

        fn description(&self) -> &str {
            "Connecteur utilisé uniquement pour les tests"
        }

        fn supports(&self, operation: &str) -> bool {
            operation == "echo"
        }

        fn execute(&self, request: &ConnectorRequest) -> Result<ConnectorResult> {
            Ok(ConnectorResult::success(
                self.name(),
                request.operation.clone(),
                "Opération de test exécutée",
                Some(request.arguments.clone()),
            ))
        }
    }

    #[test]
    fn registers_and_finds_connector() {
        let registry = ConnectorRegistry::new();

        registry.register(TestConnector).unwrap();

        assert!(registry.contains("test"));
        assert!(!registry.contains("unknown"));
        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn rejects_duplicate_connector_names() {
        let registry = ConnectorRegistry::new();

        registry.register(TestConnector).unwrap();

        let result = registry.register(TestConnector);

        assert!(result.is_err());
        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn executes_supported_operation() {
        let registry = ConnectorRegistry::new();
        registry.register(TestConnector).unwrap();

        let request = ConnectorRequest::new(
            "test",
            "echo",
            json!({"message": "MIDAS"}),
        )
        .unwrap();

        let result = registry.execute(&request).unwrap();

        assert!(result.success);
        assert_eq!(result.connector, "test");
        assert_eq!(result.operation, "echo");
        assert_eq!(result.data, Some(json!({"message": "MIDAS"})));
    }

    #[test]
    fn refuses_unsupported_operation() {
        let registry = ConnectorRegistry::new();
        registry.register(TestConnector).unwrap();

        let request = ConnectorRequest::new(
            "test",
            "delete_everything",
            json!({}),
        )
        .unwrap();

        assert!(registry.execute(&request).is_err());
    }

    #[test]
    fn refuses_unknown_connector() {
        let registry = ConnectorRegistry::new();

        let request = ConnectorRequest::new(
            "missing",
            "echo",
            json!({}),
        )
        .unwrap();

        assert!(registry.execute(&request).is_err());
    }

    #[test]
    fn rejects_empty_identifiers() {
        assert!(ConnectorRequest::new("", "echo", json!({})).is_err());
        assert!(ConnectorRequest::new("test", "", json!({})).is_err());
    }

    #[test]
    fn lists_connectors_in_stable_order() {
        struct AnotherConnector;

        impl Connector for AnotherConnector {
            fn name(&self) -> &str {
                "alpha"
            }

            fn description(&self) -> &str {
                "Second connecteur de test"
            }

            fn supports(&self, _operation: &str) -> bool {
                false
            }

            fn execute(
                &self,
                _request: &ConnectorRequest,
            ) -> Result<ConnectorResult> {
                Err(anyhow!("Aucune opération n'est implémentée"))
            }
        }

        let registry = ConnectorRegistry::new();
        registry.register(TestConnector).unwrap();
        registry.register(AnotherConnector).unwrap();

        let items = registry.list().unwrap();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "alpha");
        assert_eq!(items[1].name, "test");
    }
}
