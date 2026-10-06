use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::foundation::{
    Goal,
    RiskLevel,
};

use super::{
    action_plan::ActionPlan,
    cognitive_cycle::{
        CognitiveCycle,
        CognitiveCycleStatus,
    },
    cognitive_depth::{
        CognitiveDepth,
        CognitiveDepthPolicy,
    },
    cognitive_phase::{
        CognitivePhase,
        CognitivePhaseResult,
    },
    cognitive_state::{
        CognitiveState,
        CognitiveStateStatus,
    },
    decision::{
        Decision,
        DecisionKind,
    },
    goal_context::GoalContext,
    health::{
        CognitionHealth,
        CognitionHealthState,
    },
    problem::Problem,
};

#[derive(Debug, Clone)]
pub struct CognitionConfig {
    pub depth_policy:
        CognitiveDepthPolicy,

    pub minimum_decision_confidence:
        f32,

    pub maximum_iterations:
        u64,

    pub require_verification_for_high_risk:
        bool,

    pub require_simulation_for_irreversible:
        bool,
}

impl Default for CognitionConfig {
    fn default() -> Self {
        Self {
            depth_policy:
                CognitiveDepthPolicy::default(),

            minimum_decision_confidence:
                0.7,

            maximum_iterations:
                10_000,

            require_verification_for_high_risk:
                true,

            require_simulation_for_irreversible:
                true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CognitionRequest {
    pub request_id:
        Uuid,

    pub goal:
        Goal,

    pub priority:
        super::goal_context::GoalPriority,

    pub complexity:
        f32,

    pub expected_impact:
        f32,

    pub risk:
        RiskLevel,

    pub irreversibility:
        f32,

    pub constraints:
        Vec<String>,
}

impl CognitionRequest {
    pub fn new(goal: Goal) -> Self {
        Self {
            request_id:
                Uuid::new_v4(),

            goal,

            priority:
                super::goal_context::GoalPriority::Normal,

            complexity: 0.5,

            expected_impact: 0.5,

            risk: RiskLevel::Low,

            irreversibility: 0.0,

            constraints: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CognitionResult {
    pub request_id:
        Uuid,

    pub cycle:
        CognitiveCycle,

    pub problem:
        Problem,

    pub decision:
        Option<Decision>,

    pub action_plan:
        Option<ActionPlan>,

    pub completed:
        bool,

    pub blocked:
        bool,

    pub reason:
        Option<String>,
}

pub struct CognitionEngine {
    config:
        CognitionConfig,

    active:
        Arc<RwLock<bool>>,

    state:
        Arc<RwLock<CognitiveState>>,

    cycles_started:
        Arc<RwLock<u64>>,

    cycles_completed:
        Arc<RwLock<u64>>,

    cycles_failed:
        Arc<RwLock<u64>>,

    cycles_blocked:
        Arc<RwLock<u64>>,
}

impl CognitionEngine {
    pub fn new(
        config: CognitionConfig,
    ) -> Self {
        Self {
            config,

            active:
                Arc::new(
                    RwLock::new(false),
                ),

            state:
                Arc::new(
                    RwLock::new(
                        CognitiveState::default(),
                    ),
                ),

            cycles_started:
                Arc::new(
                    RwLock::new(0),
                ),

            cycles_completed:
                Arc::new(
                    RwLock::new(0),
                ),

            cycles_failed:
                Arc::new(
                    RwLock::new(0),
                ),

            cycles_blocked:
                Arc::new(
                    RwLock::new(0),
                ),
        }
    }

    pub async fn start(&self) {
        *self.active.write().await = true;

        let mut state =
            self.state.write().await;

        state.status =
            CognitiveStateStatus::Idle;
    }

    pub async fn stop(&self) {
        *self.active.write().await = false;

        let mut state =
            self.state.write().await;

        state.status =
            CognitiveStateStatus::Idle;

        state.cycle_id = None;
        state.phase = None;
        state.progress = 0.0;
    }

    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }

    pub fn select_depth(
        &self,
        request: &CognitionRequest,
    ) -> CognitiveDepth {
        self.config.depth_policy.select_depth(
            request.complexity,
            request.risk_score(),
            request.expected_impact,
            request.irreversibility,
        )
    }

    pub async fn begin_cycle(
        &self,
        request: &CognitionRequest,
    ) -> Result<CognitiveCycle, String> {
        if !self.is_active().await {
            return Err(
                "cognition subsystem is not active"
                    .into()
            );
        }

        let depth =
            self.select_depth(request);

        let cycle =
            CognitiveCycle::new(depth);

        {
            let mut state =
                self.state.write().await;

            state.cycle_id =
                Some(cycle.id.uuid());

            state.status =
                CognitiveStateStatus::Processing;

            state.depth = depth;

            state.phase =
                Some(CognitivePhase::Objective);

            state.progress = 0.0;

            state.blocking_reasons.clear();

            state.unresolved_unknowns.clear();

            state.unresolved_critiques = 0;
        }

        {
            let mut value =
                self.cycles_started
                    .write()
                    .await;

            *value += 1;
        }

        Ok(cycle)
    }

    pub fn create_goal_context(
        &self,
        request: &CognitionRequest,
    ) -> GoalContext {
        let mut context =
            GoalContext::new(
                request.goal.clone(),
            );

        context.priority =
            request.priority;

        context.complexity =
            request.complexity;

        context.expected_impact =
            request.expected_impact;

        context.risk =
            request.risk;

        context.irreversibility =
            request.irreversibility;

        context.constraints =
            request.constraints.clone();

        context
    }

    pub fn create_problem(
        &self,
        context: &GoalContext,
    ) -> Result<Problem, String> {
        let mut problem =
            Problem::new(
                context.goal.description.clone(),
            )?;

        problem.complexity =
            context.complexity;

        for constraint
            in &context.constraints
        {
            problem.constraints.push(
                super::problem::ProblemConstraint::new(
                    constraint.clone(),
                    true,
                )?,
            );
        }

        problem.state =
            super::problem::ProblemState::Understood;

        Ok(problem)
    }

    pub async fn record_phase(
        &self,
        cycle: &mut CognitiveCycle,
        result: CognitivePhaseResult,
    ) {
        let phase =
            result.phase;

        let success =
            result.success;

        let blocking =
            result.blocking;

        cycle.record_phase(result);

        {
            let mut state =
                self.state.write().await;

            state.phase =
                Some(phase);

            state.progress =
                cycle_progress(
                    cycle.completed_phases.len(),
                );

            if blocking {
                state.status =
                    CognitiveStateStatus::Blocked;
            } else if success {
                state.status =
                    CognitiveStateStatus::Processing;
            }
        }

        if blocking {
            cycle.block();
        }
    }

    pub async fn finalize_decision(
        &self,
        cycle: &mut CognitiveCycle,
        decision: Decision,
    ) -> CognitionResult {
        if !decision.validated {
            cycle.block();

            {
                let mut state =
                    self.state.write().await;

                state.status =
                    CognitiveStateStatus::Blocked;

                state.blocking_reasons.push(
                    "decision has not been validated"
                        .into(),
                );
            }

            {
                let mut value =
                    self.cycles_blocked
                        .write()
                        .await;

                *value += 1;
            }

            return CognitionResult {
                request_id:
                    Uuid::nil(),

                cycle:
                    cycle.clone(),

                problem:
                    Problem {
                        id: Uuid::nil(),
                        statement:
                            "decision validation failed"
                                .into(),
                        state:
                            super::problem::ProblemState::Blocked,
                        complexity: 1.0,
                        constraints: Vec::new(),
                        unknowns: Vec::new(),
                        required_capabilities:
                            Vec::new(),
                        success_conditions:
                            Vec::new(),
                    },

                decision:
                    Some(decision),

                action_plan:
                    None,

                completed: false,

                blocked: true,

                reason:
                    Some(
                        "decision has not been validated"
                            .into(),
                    ),
            };
        }

        if decision.confidence.score
            < self.config
                .minimum_decision_confidence
        {
            cycle.block();

            {
                let mut state =
                    self.state.write().await;

                state.status =
                    CognitiveStateStatus::Blocked;

                state.blocking_reasons.push(
                    "decision confidence below configured threshold"
                        .into(),
                );
            }

            {
                let mut value =
                    self.cycles_blocked
                        .write()
                        .await;

                *value += 1;
            }

            return CognitionResult {
                request_id:
                    Uuid::nil(),

                cycle:
                    cycle.clone(),

                problem:
                    Problem {
                        id: Uuid::nil(),
                        statement:
                            "decision confidence insufficient"
                                .into(),
                        state:
                            super::problem::ProblemState::Blocked,
                        complexity: 1.0,
                        constraints: Vec::new(),
                        unknowns: Vec::new(),
                        required_capabilities:
                            Vec::new(),
                        success_conditions:
                            Vec::new(),
                    },

                decision:
                    Some(decision),

                action_plan:
                    None,

                completed: false,

                blocked: true,

                reason:
                    Some(
                        "decision confidence below configured threshold"
                            .into(),
                    ),
            };
        }

        cycle.complete();

        {
            let mut state =
                self.state.write().await;

            state.status =
                CognitiveStateStatus::Completed;

            state.progress = 1.0;

            state.phase =
                Some(CognitivePhase::Evolve);
        }

        {
            let mut value =
                self.cycles_completed
                    .write()
                    .await;

            *value += 1;
        }

        CognitionResult {
            request_id:
                Uuid::nil(),

            cycle:
                cycle.clone(),

            problem:
                Problem {
                    id: Uuid::nil(),
                    statement:
                        "cognitive decision finalized"
                            .into(),
                    state:
                        super::problem::ProblemState::Solved,
                    complexity: 0.0,
                    constraints: Vec::new(),
                    unknowns: Vec::new(),
                    required_capabilities:
                        Vec::new(),
                    success_conditions:
                        Vec::new(),
                },

            decision:
                Some(decision),

            action_plan:
                None,

            completed: true,

            blocked: false,

            reason: None,
        }
    }

    pub async fn state(
        &self,
    ) -> CognitiveState {
        self.state.read().await.clone()
    }

    pub async fn health(
        &self,
    ) -> CognitionHealth {
        let active =
            self.is_active().await;

        let started =
            *self.cycles_started
                .read()
                .await;

        let completed =
            *self.cycles_completed
                .read()
                .await;

        let failed =
            *self.cycles_failed
                .read()
                .await;

        let blocked =
            *self.cycles_blocked
                .read()
                .await;

        let state =
            self.state.read().await;

        let health_state =
            if !active {
                CognitionHealthState::Unavailable
            } else if state.status
                == CognitiveStateStatus::Blocked
            {
                CognitionHealthState::Blocked
            } else {
                CognitionHealthState::Healthy
            };

        CognitionHealth {
            state:
                health_state,

            active,

            cycles_started:
                started,

            cycles_completed:
                completed,

            cycles_failed:
                failed,

            blocked_cycles:
                blocked,

            unresolved_unknowns:
                state.unresolved_unknowns.len(),

            unresolved_critiques:
                state.unresolved_critiques,

            decision_quality:
                if completed > 0 {
                    1.0
                } else {
                    0.0
                },

            reasoning_available:
                active,

            verification_available:
                active,

            simulation_available:
                active,
        }
    }

    pub fn config(
        &self,
    ) -> &CognitionConfig {
        &self.config
    }
}

impl CognitionRequest {
    fn risk_score(&self) -> f32 {
        match self.risk {
            RiskLevel::Low => 0.2,
            RiskLevel::Medium => 0.5,
            RiskLevel::High => 0.8,
            RiskLevel::Critical => 1.0,
        }
    }
}

fn cycle_progress(
    completed_phases: usize,
) -> f32 {
    let total =
        19.0_f32;

    (completed_phases as f32 / total)
        .clamp(0.0, 1.0)
}
