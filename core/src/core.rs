use crate::{
    action::{ActionEngine, ActionResult},
    capabilities::CapabilityRegistry,
    cognition::{Cognition, Understanding},
    correction::{Correction, CorrectionEngine},
    decision::{Decision, DecisionEngine},
    identity::Identity,
    learning::{Learning, LearningEngine},
    memory::{Memory, MemoryKind, MemoryStore},
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
    pub fn new(memory_path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
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

        let memory = MemoryStore::new(memory_path)?;

        Ok(Self {
            identity: Identity::default(),
            state: CoreState::default(),
            capabilities,
            memory,
        })
    }

    pub fn cycle(
        &mut self,
        perceptions: Vec<Perception>,
    ) -> anyhow::Result<CycleResult> {
        self.state.next_cycle();

        let understanding = Cognition::understand(&perceptions);

        self.memory.write(&Memory::new(
            MemoryKind::Working,
            "core.perception",
            format!(
                "Cycle {} : {} perception(s) reçue(s).",
                self.state.cycle,
                perceptions.len()
            ),
        ))?;

        let reflection = ReflectionEngine::reflect(&understanding);

        self.memory.write(&Memory::new(
            MemoryKind::Episodic,
            "core.reflection",
            reflection.reasoning.clone(),
        ))?;

        let decision = DecisionEngine::decide(&reflection);

        self.memory.write(&Memory::new(
            MemoryKind::Decision,
            "core.decision",
            format!("{decision:?}"),
        ))?;

        let action = ActionEngine::execute(&decision);

        self.memory.write(&Memory::new(
            MemoryKind::Episodic,
            "core.action",
            action.description.clone(),
        ))?;

        let observation = ObservationEngine::observe(&action);

        self.memory.write(&Memory::new(
            MemoryKind::Episodic,
            "core.observation",
            observation.result.clone(),
        ))?;

        let learning = LearningEngine::learn(&observation);

        self.memory.write(&Memory::new(
            MemoryKind::Learning,
            "core.learning",
            learning.lesson.clone(),
        ))?;

        let correction = CorrectionEngine::evaluate(&learning);

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
}
