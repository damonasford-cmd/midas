use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// Identité système interne.
    pub system_name: String,

    /// Nom public de MIDAS.
    pub public_name: String,

    /// Version actuelle du Core.
    pub version: String,

    /// Rôle fondamental du système.
    pub role: String,

    /// Objectifs actuellement connus.
    pub objectives: Vec<String>,

    /// Capacités actuellement disponibles.
    pub capabilities: Vec<String>,

    /// Capacités identifiées comme manquantes.
    pub missing_capabilities: Vec<String>,

    /// État général du système.
    pub state: IdentityState,

    /// Description de ce que MIDAS sait actuellement
    /// de sa propre architecture.
    pub self_model: SelfModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityState {
    Initializing,
    Operational,
    Degraded,
    Maintenance,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    /// Composants connus du système.
    pub components: Vec<String>,

    /// Modèles de raisonnement disponibles.
    pub reasoning_models: Vec<String>,

    /// Connecteurs actuellement connus.
    pub connectors: Vec<String>,

    /// Environnement d'exécution connu.
    pub environment: String,

    /// Dernière évolution connue.
    pub last_evolution: Option<String>,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            system_name: "MIDAS".to_string(),

            public_name: "Aeron Asford".to_string(),

            version: "0.5.0".to_string(),

            role: "Intelligence unifiée autonome".to_string(),

            objectives: vec![
                "Comprendre les objectifs reçus.".to_string(),
                "Percevoir son environnement.".to_string(),
                "Raisonner avant d'agir.".to_string(),
                "Agir de manière autorisée.".to_string(),
                "Observer les résultats.".to_string(),
                "Apprendre et améliorer son fonctionnement.".to_string(),
                "Détecter les capacités manquantes.".to_string(),
            ],

            capabilities: Vec::new(),

            missing_capabilities: Vec::new(),

            state: IdentityState::Initializing,

            self_model: SelfModel {
                components: vec![
                    "Core".to_string(),
                    "Cognition".to_string(),
                    "Memory".to_string(),
                    "Model Gateway".to_string(),
                ],

                reasoning_models: Vec::new(),

                connectors: Vec::new(),

                environment: "Unknown".to_string(),

                last_evolution: None,
            },
        }
    }
}

impl Identity {
    pub fn set_operational(&mut self) {
        self.state = IdentityState::Operational;
    }

    pub fn set_degraded(&mut self) {
        self.state = IdentityState::Degraded;
    }

    pub fn set_maintenance(&mut self) {
        self.state = IdentityState::Maintenance;
    }

    pub fn set_stopped(&mut self) {
        self.state = IdentityState::Stopped;
    }

    pub fn register_capability(
        &mut self,
        capability: impl Into<String>,
    ) {
        let capability = capability.into();

        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    pub fn register_missing_capability(
        &mut self,
        capability: impl Into<String>,
    ) {
        let capability = capability.into();

        if !self
            .missing_capabilities
            .contains(&capability)
        {
            self.missing_capabilities
                .push(capability);
        }
    }

    pub fn resolve_missing_capability(
        &mut self,
        capability: &str,
    ) {
        self.missing_capabilities
            .retain(|item| item != capability);

        self.register_capability(
            capability.to_string(),
        );
    }

    pub fn register_component(
        &mut self,
        component: impl Into<String>,
    ) {
        let component = component.into();

        if !self
            .self_model
            .components
            .contains(&component)
        {
            self.self_model
                .components
                .push(component);
        }
    }

    pub fn register_reasoning_model(
        &mut self,
        model: impl Into<String>,
    ) {
        let model = model.into();

        if !self
            .self_model
            .reasoning_models
            .contains(&model)
        {
            self.self_model
                .reasoning_models
                .push(model);
        }
    }

    pub fn register_connector(
        &mut self,
        connector: impl Into<String>,
    ) {
        let connector = connector.into();

        if !self
            .self_model
            .connectors
            .contains(&connector)
        {
            self.self_model
                .connectors
                .push(connector);
        }
    }

    pub fn set_environment(
        &mut self,
        environment: impl Into<String>,
    ) {
        self.self_model.environment =
            environment.into();
    }

    pub fn record_evolution(
        &mut self,
        description: impl Into<String>,
    ) {
        self.self_model.last_evolution =
            Some(description.into());
    }

    pub fn has_capability(
        &self,
        capability: &str,
    ) -> bool {
        self.capabilities
            .iter()
            .any(|item| item == capability)
    }

    pub fn lacks_capability(
        &self,
        capability: &str,
    ) -> bool {
        self.missing_capabilities
            .iter()
            .any(|item| item == capability)
    }

    pub fn summary(&self) -> String {
        format!(
            "{} / {} / version {} / état {:?}",
            self.system_name,
            self.public_name,
            self.version,
            self.state
        )
    }
}
