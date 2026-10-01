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
        Connector,
        ConnectorRegistry,
        ConnectorResult,
        LocalProcessConnector,
    },
    correction::{
        Correction,
        CorrectionEngine,
    },
    decision::{
        Decision,
        DecisionEngine,
    },
    execution::{
        ExecutionEngine,
        ExecutionRequest,
        ExecutionResult,
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
    pub execution: Option<ExecutionResult>,
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
            "execution_engine",
            "Moteur d'exécution des actions autorisées",
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
         * ============================================================
         * CONNECTEURS NATIFS
         * ============================================================
         *
         * La liste des programmes autorisés est volontairement
         * explicite.
         *
         * On pourra ensuite ajouter d'autres connecteurs :
         *
         * API
         * Web
         * Telegram
         * Git
         * fichiers
         * logiciels
         * machines
         * finance
         * etc.
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
        /*
         * ============================================================
         * 1. PERCEIVE
         * ============================================================
         */

        self.state.next_cycle();

        /*
         * ============================================================
         * 2. UNDERSTAND
         * ============================================================
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
         * ============================================================
         * 3. REFLECT
         * ============================================================
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
         * ============================================================
         * 4. DECIDE
         * ============================================================
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
         * ============================================================
         * 5. ACTION / AUTHORIZATION
         * ============================================================
         */

        let mut action =
            ActionEngine::execute(
                &decision,
            );

        /*
         * Une demande de validation est enregistrée
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
         * ============================================================
         * 6. EXECUTION
         * ============================================================
         *
         * Le Core ne décide pas lui-même comment exécuter.
         *
         * ExecutionEngine est maintenant la frontière officielle
         * entre l'action autorisée et son exécution réelle.
         */

        let execution =
            self.build_and_execute(
                &decision,
                &action,
            )?;

        /*
         * ============================================================
         * 7. CONNECTOR RESULT
         * ============================================================
         */

        let connector_result =
            execution
                .as_ref()
                .and_then(|result| {
                    result
                        .connector_result
                        .clone()
                });

        /*
         * ============================================================
         * 8. OBSERVATION
         * ============================================================
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
                            true,
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

                    if let Some(code) =
                        result.exit_code
                    {
                        observation.add_change(
                            format!(
                                "code de sortie : {}",
                                code
                            ),
                        );
                    }

                    observation
                }

                None => {
                    if let Some(execution) =
                        &execution
                    {
                        Observation::new(
                            "execution",
                            execution.message.clone(),
                        )
                        .with_success(
                            execution.success,
                        )
                        .with_execution(
                            execution.executed,
                        )
                    } else {
                        ObservationEngine::observe(
                            &action,
                        )
                    }
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
         * ============================================================
         * 9. LEARNING
         * ============================================================
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
         * ============================================================
         * 10. CORRECTION
         * ============================================================
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
         * ============================================================
         * 11. CORE STATE
         * ============================================================
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
            execution,
            connector_result,
            observation,
            learning,
            correction,
        })
    }

    fn build_and_execute(
        &self,
        decision: &Decision,
        action: &ActionResult,
    ) -> anyhow::Result<Option<ExecutionResult>> {
        /*
         * Aucune exécution si l'action n'est pas autorisée.
         */

        if !ExecutionEngine::can_execute(
            action,
            decision,
        ) {
            return Ok(None);
        }

        /*
         * ============================================================
         * PROTOCOLE STRUCTURÉ ACTUEL
         * ============================================================
         *
         * Pour l'instant, une action explicite peut utiliser :
         *
         * action:
         *     connector:<nom>
         *
         * rationale:
         *     operation:<programme>
         *
         * Les arguments seront transportés par la future couche
         * d'action structurée.
         */

        let connector =
            match decision
                .action
                .strip_prefix("connector:")
            {
                Some(value) =>
                    value.trim(),

                None =>
                    return Ok(None),
            };

        if connector.is_empty() {
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
            ExecutionRequest::new(
                connector,
                operation,
                Vec::new(),
                action.profile.clone(),
            );

        let execution =
            ExecutionEngine::execute(
                &self.connectors,
                decision,
                action,
                &request,
            )?;

        Ok(Some(execution))
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
        connector: Box<dyn Connector>,
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
