use crate::{
    action::{
        ActionEngine,
        ActionResult,
    },
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

pub struct MidasCore {
    pub identity: Identity,
    pub state: CoreState,
    pub capabilities: CapabilityRegistry,
    pub memory: MemoryStore,
    pub model: ModelClient,
    pub config: MidasConfig,
}

pub struct CycleResult {
    pub understanding: Understanding,
    pub reflection: Reflection,
    pub decision: Decision,
    pub action: ActionResult,
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

        Ok(Self {
            identity: Identity::default(),
            state: CoreState::default(),
            capabilities,
            memory,
            model,
            config,
        })
    }

    pub fn cycle(
        &mut self,
        perceptions: Vec<Perception>,
    ) -> anyhow::Result<CycleResult> {
        self.state.next_cycle();

        /*
         * 1. PERCEPTION → COMPRÉHENSION
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
         * 2. RÉFLEXION
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
         * 3. DÉCISION
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
         * 4. ACTION + GATE + PROTOCOLE DE VALIDATION
         */

        let action =
            ActionEngine::execute(
                &decision,
            );

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
         * 5. OBSERVATION
         */

        let observation =
            ObservationEngine::observe(
                &action,
            );

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
         * 6. APPRENTISSAGE
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
         * 7. CORRECTION
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
         * 8. RÉSULTAT DU CYCLE
         */

        Ok(CycleResult {
            understanding,
            reflection,
            decision,
            action,
            observation,
            learning,
            correction,
        })
    }

    /*
     * 9. ÉVALUATION DES CAPACITÉS
     */

    pub fn assess_capabilities(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> CapabilityAssessment {
        self.capabilities
            .assess(requirements)
    }

    /*
     * 10. PLANIFICATION DES CAPACITÉS
     */

    pub fn plan_capabilities(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> Vec<CapabilityPlan> {
        self.capabilities
            .build_plan(requirements)
    }

    /*
     * 11. ENREGISTREMENT D'UNE NOUVELLE CAPACITÉ
     */

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

    /*
     * 12. DÉCLARATION D'UNE CAPACITÉ MANQUANTE
     */

    pub fn register_missing_capability(
        &mut self,
        capability: impl Into<String>,
    ) {
        self.identity
            .register_missing_capability(
                capability,
            );
    }

    /*
     * 13. RAISONNEMENT AVEC LE MODÈLE
     */

    pub async fn reason_with_model(
        &self,
        context: &str,
    ) -> anyhow::Result<String> {
        let request = ChatRequest {
            model: self.config.model_name.clone(),

            messages: vec![
                ChatMessage::system(
                    "Tu es le moteur de raisonnement \
                     utilisé par MIDAS Core. \
                     Tu fournis une analyse structurée. \
                     Ta sortie constitue une information \
                     de raisonnement et non une autorisation \
                     d'exécution.",
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
