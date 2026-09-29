use crate::{
    action,
    capabilities::CapabilityRegistry,
    cognition,
    correction,
    decision,
    identity::Identity,
    learning,
    memory::Memory,
    observation,
    perception::Perception,
    reflection,
    state::{CoreState, CoreStatus},
};

#[derive(Debug)]
pub struct MidasCore {
    pub identity: Identity,
    pub state: CoreState,
    pub capabilities: CapabilityRegistry,
    pub memory: Memory,
}

impl Default for MidasCore {
    fn default() -> Self {
        let mut capabilities = CapabilityRegistry::default();

        for capability in [
            "perception",
            "understanding",
            "reflection",
            "reasoning",
            "decision",
            "action",
            "observation",
            "learning",
            "correction",
            "memory",
            "self-inspection",
        ] {
            capabilities.register(capability);
        }

        Self {
            identity: Identity::default(),
            state: CoreState::default(),
            capabilities,
            memory: Memory::new(),
        }
    }
}

impl MidasCore {
    pub fn run_cycle(
        &mut self,
        goal: impl Into<String>,
        input: impl Into<String>,
    ) -> String {
        let goal = goal.into();
        let input = input.into();

        self.state.cycle += 1;
        self.state.last_goal = Some(goal.clone());

        self.state.status = CoreStatus::Perceiving;

        let perception =
            Perception::new("internal", input);

        self.memory.store(
            self.state.cycle,
            "perception",
            perception.content.clone(),
        );

        self.state.status = CoreStatus::Reasoning;

        let understanding =
            cognition::understand(
                &goal,
                &[perception],
            );

        self.memory.store(
            self.state.cycle,
            "understanding",
            understanding.summary.clone(),
        );

        let reflection =
            reflection::reflect(
                &goal,
                &understanding,
            );

        self.memory.store(
            self.state.cycle,
            "reflection",
            reflection.reasoning.clone(),
        );

        let decision =
            decision::decide(
                &goal,
                &reflection,
            );

        self.state.last_decision =
            Some(decision.action.clone());

        self.memory.store(
            self.state.cycle,
            "decision",
            decision.action.clone(),
        );

        self.state.status = CoreStatus::Acting;

        let action_result =
            action::execute(&decision);

        self.memory.store(
            self.state.cycle,
            "action",
            action_result.message.clone(),
        );

        self.state.status =
            CoreStatus::Observing;

        let observation =
            observation::observe(&action_result);

        self.state.last_result =
            Some(observation.result.clone());

        self.memory.store(
            self.state.cycle,
            "observation",
            observation.result.clone(),
        );

        self.state.status =
            CoreStatus::Learning;

        let learning =
            learning::learn(&observation);

        self.memory.store(
            self.state.cycle,
            "learning",
            learning.lesson.clone(),
        );

        self.state.status =
            CoreStatus::Correcting;

        let correction =
            correction::correct(&learning);

        self.memory.store(
            self.state.cycle,
            "correction",
            correction.instruction.clone(),
        );

        self.state.status = CoreStatus::Ready;

        format!(
            "Cycle {} terminé.\n\
             Objectif: {}\n\
             Décision: {}\n\
             Résultat: {}\n\
             Apprentissage: {}\n\
             Correction: {}\n\
             Souvenirs: {}",
            self.state.cycle,
            goal,
            decision.action,
            observation.result,
            learning.lesson,
            correction.instruction,
            self.memory.len(),
        )
    }
}
