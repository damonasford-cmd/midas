use crate::{
    action::{ActionEngine, ActionResult},
    capabilities::CapabilityRegistry,
    cognition::{Cognition, Understanding},
    config::MidasConfig,
    correction::{Correction, CorrectionEngine},
    decision::{Decision, DecisionEngine},
    identity::Identity,
    learning::{Learning, LearningEngine},
    memory::{Memory, MemoryKind, MemoryStore},
    model::{ChatMessage, ChatRequest, ModelClient},
    observation::{Observation, ObservationEngine},
    perception::Perception,
    reflection::{Reflection, ReflectionEngine},
    state::CoreState,
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
    pub fn new(config: MidasConfig) -> anyhow::Result<Self> {
        let mut capabilities = CapabilityRegistry::default();

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
            "Exécution des actions autorisées",
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
            MemoryStore::new(&config.memory_path)?;

        let model = ModelClient::new(
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

        let understanding =
            Cognition::understand(&perceptions);

        self.memory.write(&Memory::new(
            MemoryKind::Working,
            "core.perception",
            format!(
                "Cycle {} : {} perception(s) reçue(s).",
                self.state.cycle,
                perceptions.len()
            ),
        ))?;

        let reflection =
            ReflectionEngine::reflect(&understanding);

        self.memory.write(&Memory::new(
            MemoryKind::Episodic,
            "core.reflection",
            reflection.reasoning.clone(),
        ))?;

        let decision =
            DecisionEngine::decide(&reflection);

        self.memory.write(&Memory::new(
            MemoryKind::Decision,
            "core.decision",
            format!("{decision:?}"),
        ))?;

        let action =
            ActionEngine::execute(&decision);

        self.memory.write(&Memory::new(
            MemoryKind::Episodic,
            "core.action",
            action.description.clone(),
        ))?;

        let observation =
            ObservationEngine::observe(&action);

        self.memory.write(&Memory::new(
            MemoryKind::Episodic,
            "core.observation",
            observation.result.clone(),
        ))?;

        let learning =
            LearningEngine::learn(&observation);

        self.memory.write(&Memory::new(
            MemoryKind::Learning,
            "core.learning",
            learning.lesson.clone(),
        ))?;

        let correction =
            CorrectionEngine::evaluate(&learning);

        self.memory.write(&Memory::new(
            MemoryKind::System,
            "core.correction",
            correction.action.clone(),
        ))?;

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

                ChatMessage::user(context),
            ],

            temperature: Some(
                self.config.model_temperature
            ),
        };

        let response =
            self.model.chat(&request).await?;

        Ok(response.content)
    }
}
