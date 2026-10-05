use thiserror::Error;

#[derive(Debug, Error)]
pub enum FoundationError {
    #[error("runtime is not operational")]
    RuntimeNotOperational,

    #[error("runtime is stopping")]
    RuntimeStopping,

    #[error("system is stopped")]
    SystemStopped,

    #[error("execution is blocked")]
    ExecutionBlocked,

    #[error("authorization required")]
    AuthorizationRequired,

    #[error("resource unavailable: {0}")]
    ResourceUnavailable(String),

    #[error("invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("transaction error: {0}")]
    Transaction(String),

    #[error("concurrency error: {0}")]
    Concurrency(String),

    #[error("event bus error: {0}")]
    EventBus(String),

    #[error("invalid contract: {0}")]
    InvalidContract(String),

    #[error("internal foundation error: {0}")]
    Internal(String),
}

pub type FoundationResult<T> = Result<T, FoundationError>;
