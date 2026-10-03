pub mod autonomy;
pub mod backup;
pub mod capability;
pub mod cognition;
pub mod communication;
pub mod creation;
pub mod execution;
pub mod forge;
pub mod foundation;
pub mod identity;
pub mod identity_runtime;
pub mod infrastructure;
pub mod intelligence;
pub mod memory;
pub mod model_runtime;
pub mod observability;
pub mod perception;
pub mod research;
pub mod security;
pub mod world;

pub use foundation::contracts::{
    ActionIntent,
    ActionResult,
    Capability,
    Event,
    EventKind,
    Goal,
    Observation,
    Project,
    Risk,
    Task,
};

pub use foundation::runtime::MidasRuntime;
pub use identity::identity::MidasIdentity;
