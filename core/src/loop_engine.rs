use crate::{
    action,
    capabilities::CapabilityRegistry,
    cognition,
    correction,
    decision,
    identity::Identity,
    learning,
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
}

impl Default for MidasCore {
    fn default() -> Self {
        let mut capabilities =
            CapabilityRegistry::default();

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
            "self-inspection",
        ] {
            capabilities.register(capability);
        }

        Self {
            identity: Identity::default(),
            state: CoreState::default(),
            capabilities,
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

        self.state.status = CoreStatus::Reasoning;

        let understanding =
            cognition::understand(
                &goal,
                &[perception],
            );

        let reflection =
            reflection::reflect(
                &goal,
                &understanding,
            );

        let decision =
            decision::decide(
                &goal,
                &reflection,
            );

        self.state.last_decision =
            Some(decision.action.clone());

        self.state.status = CoreStatus::Acting;

        let action_result =
            action::execute(&decision);

        self.state.status =
            CoreStatus::Observing;

        let observation =
            observation::observe(
                &action_result,
            );

        self.state.last_result =
            Some(observation.result.clone());

        self.state.status =
            CoreStatus::Learning;

        let learning =
            learning::learn(&observation);

        self.state.status =
            CoreStatus::Correcting;

        let correction =
            correction::correct(&learning);

        self.state.status =
            CoreStatus::Ready;

        format!(
            "Cycle {} terminé.\n\
             Objectif: {}\n\
             Décision: {}\n\
             Résultat: {}\n\
             Apprentissage: {}\n\
             Correction: {}",
            self.state.cycle,
            goal,
            decision.action,
            observation.result,
            learning.lesson,
            correction.instruction
        )
    }
}
