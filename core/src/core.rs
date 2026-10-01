use crate::{
    action::{
        ActionEngine,
        ActionResult,
        ApprovalRequest,
    },
    approval::ApprovalManager,
    capabilities::{
        CapabilityAssessment,
        CapabilityPlan,
        CapabilityRegistry,
        CapabilityRequirement,
    },
    cognition::{
        Cognition,
        Understanding,
    },
    config::MidasConfig,
    connector::{
        ConnectorRegistry,
        LocalProcessConnector,
        ConnectorRequest,
        ConnectorResult,
    },
    correction::{
        Correction,
        CorrectionEngine,
    },
    decision::{
        Decision,
        DecisionEngine,
    },
    identity::Identity,
    learning::{
        Learning,
        LearningEngine,
    },
    model::{
        ChatMessage,
        ChatRequest,
        ModelClient,
    },
    observation::{
        Observation,
        ObservationEngine,
    },
    perception::Perception,
    reflection::{
        Reflection,
        ReflectionEngine,
    },
    state::CoreState,
};

use midas_memory::{
    Memory,
    MemoryKind,
    MemoryModality,
    MemoryStore,
};

use uuid::Uuid;

pub struct MidasCore {
    pub identity: Identity,
    pub state: CoreState,
    pub capabilities: CapabilityRegistry,
    pub memory: MemoryStore,
    pub model: ModelClient,
    pub config: MidasConfig,
    pub approvals: ApprovalManager,
    pub connectors: ConnectorRegistry,
}

pub struct CycleResult {
    pub understanding: Understanding,
    pub reflection: Reflection,
    pub decision: Decision,
    pub action: ActionResult,
    pub connector_result: Option<ConnectorResult>,
    pub observation: Observation,
    pub learning: Learning,
    pub correction: Correction,
}

impl MidasCore {
    pub fn new(
        config: MidasConfig,
    ) -> anyhow::Result<Self> {
        let mut capabilities =
            CapabilityRegistry::default();

        capabilities.register(
            "perception",
            "Perception des informations entrantes",
        );

        capabilities.register(
            "cognition",
            "Compréhension et analyse",
        );

        capabilities.register(
            "reflection",
            "Réflexion avant décision",
        );

        capabilities.register(
            "decision",
            "Prise de décision",
        );

        capabilities.register(
            "action",
            "Évaluation et préparation des actions",
        );

        capabilities.register(
            "action_authorization",
            "Contrôle d'autorisation avant exécution",
        );

        capabilities.register(
            "approval_protocol",
            "Gestion des demandes de validation critique",
        );

        capabilities.register(
            "approval_manager",
            "Gestion centralisée des validations en attente",
        );

        capabilities.register(
            "connector_registry",
            "Registre unifié des connecteurs d'exécution",
        );

        capabilities.register(
            "local_process_connector",
            "Connexion contrôlée aux processus locaux autorisés",
        );

        capabilities.register(
            "observation",
            "Observation des résultats",
        );

        capabilities.register(
            "learning",
            "Apprentissage à partir des résultats",
        );

        capabilities.register(
            "correction",
            "Évaluation et correction",
        );

        capabilities.register(
            "memory",
            "Persistance et récupération de la mémoire",
        );

        capabilities.register(
            "model_reasoning",
            "Raisonnement assisté par moteur de modèle",
        );

        let memory =
            MemoryStore::new(
                &config.memory_path,
            )?;

        let model =
            ModelClient::new(
                config.model_provider.clone(),
                &config.model_url,
            )?;

        let mut identity =
            Identity::default();

        for capability in capabilities.available() {
            identity.register_capability(
                capability.name,
            );
        }

        identity.set_environment(
            "MIDAS Core",
        );

        /*
         * Registre des connecteurs.
         *
         * Le programme local est volontairement
         * limité à une liste explicite.
         *
         * Cette liste sera remplacée/étendue
         * lorsque les vrais connecteurs seront
         * intégrés.
         */
        let mut connectors =
            ConnectorRegistry::new();

        connectors.register(
            Box::new(
                LocalProcessConnector::new(
                    vec![
                        "echo".to_string(),
                    ],
                ),
            ),
        );

        identity.register_connector(
            "local_process",
        );

        Ok(Self {
            identity,
            state: CoreState::default(),
            capabilities,
            memory,
            model,
            config,
            approvals:
                ApprovalManager::new(),
            connectors,
        })
    }

    pub fn cycle(
        &mut self,
        perceptions: Vec<Perception>,
    ) -> anyhow::Result<CycleResult> {
        self.state.next_cycle();

        /*
         * 1. PERCEIVE
         * 2. UNDERSTAND
         */
        let understanding =
            Cognition::understand(
                &perceptions,
            );

        self.memory.write(
            &Memory::new(
                MemoryKind::Working,
                "core.perception",
                format!(
                    "Cycle {} : {} perception(s) reçue(s).",
                    self.state.cycle,
                    perceptions.len()
                ),
            )
            .with_modality(
                MemoryModality::Data,
            ),
        )?;

        /*
         * 3. REFLECT
         */
        let reflection =
            ReflectionEngine::reflect(
                &understanding,
            );

        self.memory.write(
            &Memory::new(
                MemoryKind::Episodic,
                "core.reflection",
                reflection.reasoning.clone(),
            )
            .with_modality(
                MemoryModality::Text,
            ),
        )?;

        /*
         * 4. DECIDE
         */
        let decision =
            DecisionEngine::decide(
                &reflection,
            );

        self.memory.write(
            &Memory::new(
                MemoryKind::Decision,
                "core.decision",
                format!("{decision:?}"),
            )
            .with_modality(
                MemoryModality::Data,
            ),
        )?;

        /*
         * 5. PREPARE / AUTHORIZE ACTION
         */
        let mut action =
            ActionEngine::execute(
                &decision,
            );

        /*
         * Une demande d'approbation est enregistrée
         * dans le gestionnaire central.
         */
        if let Some(request) =
            action.approval.take()
        {
            let request_id =
                self.approvals
                    .insert(request);

            let stored_request =
                self.approvals
                    .get(request_id)
                    .expect(
                        "La demande de validation vient d'être enregistrée.",
                    );

            action.approval =
                Some(
                    stored_request.clone(),
                );
        }

        let action_memory =
            if let Some(approval) =
                &action.approval
            {
                format!(
                    "{} | {} | demande_validation={} | {:?}",
                    action.description,
                    action.message,
                    approval.id,
                    approval.status,
                )
            } else {
                format!(
                    "{} | {} | {:?}",
                    action.description,
                    action.message,
                    action.authorization.decision,
                )
            };

        self.memory.write(
            &Memory::new(
                MemoryKind::Episodic,
                "core.action",
                action_memory,
            )
            .with_modality(
                MemoryModality::Event,
            ),
        )?;

        /*
         * 6. EXECUTION VIA CONNECTOR
         *
         * Pour l'instant, le Core ne transforme pas
         * automatiquement une phrase en commande système.
         *
         * Le connecteur est invoqué uniquement lorsqu'une
         * demande structurée lui est fournie.
         *
         * Cela prépare la vraie couche d'exécution.
         */
        let connector_result =
            self.execute_connector_if_requested(
                &decision,
                &action,
            )?;

        /*
         * Si un connecteur réel a exécuté quelque chose,
         * son résultat devient la matière première
         * de l'observation.
         */
        let observation =
            match &connector_result {
                Some(result) => {
                    let mut observation =
                        Observation::new(
                            &result.connector,
                            &result.message,
                        )
                        .with_success(
                            result.success,
                        )
                        .with_execution(
                            result.success,
                        );

                    if !result.stdout.is_empty() {
                        observation.add_change(
                            format!(
                                "stdout : {}",
                                result.stdout.trim()
                            ),
                        );
                    }

                    if !result.stderr.is_empty() {
                        observation.add_anomaly(
                            format!(
                                "stderr : {}",
                                result.stderr.trim()
                            ),
                        );
                    }

                    observation
                }

                None => {
                    ObservationEngine::observe(
                        &action,
                    )
                }
            };

        self.memory.write(
            &Memory::new(
                MemoryKind::Episodic,
                "core.observation",
                observation.result.clone(),
            )
            .with_modality(
                MemoryModality::Event,
            ),
        )?;

        /*
         * 7. LEARN
         */
        let learning =
            LearningEngine::learn(
                &observation,
            );

        self.memory.write(
            &Memory::new(
                MemoryKind::Learning,
                "core.learning",
                learning.lesson.clone(),
            )
            .with_modality(
                MemoryModality::Text,
            ),
        )?;

        /*
         * 8. CORRECT
         */
        let correction =
            CorrectionEngine::evaluate(
                &learning,
            );

        self.memory.write(
            &Memory::new(
                MemoryKind::System,
                "core.correction",
                correction.action.clone(),
            )
            .with_modality(
                MemoryModality::Text,
            ),
        )?;

        /*
         * Mise à jour de l'état du Core.
         */
        if observation.success {
            self.state.record_success();
        } else {
            self.state.record_failure(
                observation.result.clone(),
            );
        }

        Ok(CycleResult {
            understanding,
            reflection,
            decision,
            action,
            connector_result,
            observation,
            learning,
            correction,
        })
    }

    fn execute_connector_if_requested(
        &self,
        decision: &Decision,
        action: &ActionResult,
    ) -> anyhow::Result<Option<ConnectorResult>> {
        /*
         * Pas de connecteur si :
         *
         * - l'action n'est pas exécutable ;
         * - une approbation est encore en attente ;
         * - l'action a été refusée.
         */
        if !decision.executable {
            return Ok(None);
        }

        if action.approval.is_some() {
            return Ok(None);
        }

        if action.authorization.is_denied() {
            return Ok(None);
        }

        /*
         * Pour cette première intégration, nous n'exécutons
         * automatiquement qu'une opération explicitement
         * structurée comme :
         *
         * connector:<nom>
         * operation:<programme>
         *
         * Les arguments seront ajoutés lorsque la couche
         * d'intention/action structurée sera branchée.
         */
        let connector_name =
            match decision
                .action
                .strip_prefix("connector:")
            {
                Some(value) =>
                    value.trim(),

                None =>
                    return Ok(None),
            };

        if connector_name.is_empty() {
            return Ok(None);
        }

        let operation =
            match decision
                .rationale
                .strip_prefix("operation:")
            {
                Some(value) =>
                    value.trim(),

                None =>
                    return Ok(None),
            };

        if operation.is_empty() {
            return Ok(None);
        }

        let request =
            ConnectorRequest::new(
                connector_name,
                operation,
                Vec::new(),
                Default::default(),
            );

        let result =
            self.connectors
                .execute(&request)?;

        Ok(Some(result))
    }

    pub fn assess_capabilities(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> CapabilityAssessment {
        self.capabilities
            .assess(requirements)
    }

    pub fn plan_capabilities(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> Vec<CapabilityPlan> {
        self.capabilities
            .build_plan(requirements)
    }

    pub fn register_capability(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) {
        let name = name.into();

        self.capabilities.register(
            name.clone(),
            description,
        );

        self.identity
            .register_capability(name);
    }

    pub fn register_missing_capability(
        &mut self,
        capability: impl Into<String>,
    ) {
        self.identity
            .register_missing_capability(
                capability,
            );
    }

    pub fn register_connector(
        &mut self,
        connector: Box<dyn crate::connector::Connector>,
    ) {
        let name =
            connector.name().to_string();

        self.connectors.register(
            connector,
        );

        self.identity
            .register_connector(name);
    }

    pub fn connector_names(
        &self,
    ) -> Vec<String> {
        self.connectors.names()
    }

    pub fn pending_approvals(
        &self,
    ) -> Vec<&ApprovalRequest> {
        self.approvals.pending()
    }

    pub fn pending_approval_count(
        &self,
    ) -> usize {
        self.approvals.count_pending()
    }

    pub fn approve_action(
        &mut self,
        id: Uuid,
    ) -> bool {
        self.approvals.approve(id)
    }

    pub fn refuse_action(
        &mut self,
        id: Uuid,
    ) -> bool {
        self.approvals.refuse(id)
    }

    pub fn cancel_approval(
        &mut self,
        id: Uuid,
    ) -> bool {
        self.approvals.cancel(id)
    }

    pub async fn reason_with_model(
        &self,
        context: &str,
    ) -> anyhow::Result<String> {
        let request = ChatRequest {
            model: self.config.model_name.clone(),

            messages: vec![
                ChatMessage::system(
                    "Tu es le moteur de raisonnement utilisé par MIDAS Core. \
                     Tu fournis une analyse structurée. \
                     Ta sortie constitue une information de raisonnement \
                     et non une autorisation d'exécution.",
                ),

                ChatMessage::user(
                    context,
                ),
            ],

            temperature: Some(
                self.config.model_temperature,
            ),
        };

        let response =
            self.model
                .chat(&request)
                .await?;

        Ok(response.content)
    }
}
