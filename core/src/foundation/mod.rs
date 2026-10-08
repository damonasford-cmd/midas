pub mod clock;
pub mod concurrency;
pub mod contracts;
pub mod errors;
pub mod event_bus;
pub mod lifecycle;
pub mod runtime;
pub mod transaction;
pub mod world_state;

pub use clock::MidasClock;

pub use concurrency::{
    ConcurrencyGuard,
    ExecutionLock,
};

pub use contracts::{
    ActionDomain,
    ActionIntent,
    ActionResult,
    ActionStatus,
    AuthorizationRequirement,
    Capability,
    Decision,
    DecisionStatus,
    Event,
    EventKind,
    Goal,
    GoalStatus,
    Observation,
    ObservationModality,
    Project,
    ProjectStatus,
    Risk,
    RiskLevel,
    Task,
    TaskStatus,
};

pub use errors::{
    FoundationError,
    FoundationResult,
};

pub use event_bus::{
    EventBus,
    EventSubscription,
};

pub use lifecycle::{
    LifecycleController,
    LifecycleState,
};

pub use runtime::{
    MidasRuntime,
    RuntimeHealth,
};

pub use transaction::{
    Transaction,
    TransactionManager,
    TransactionOperation,
    TransactionStatus,
};

pub use world_state::{
    SharedWorldState,
    WorldState,
};
