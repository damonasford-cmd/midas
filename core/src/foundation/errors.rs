use std::fmt;

#[derive(Debug)]
pub enum FoundationError {
    RuntimeNotOperational,
    RuntimeStopping,
    SystemStopped,
    InvalidStateTransition {
        from: String,
        to: String,
    },
    ExecutionBlocked(String),
    AuthorizationRequired(String),
    ResourceUnavailable(String),
    Transaction(String),
    Concurrency(String),
    EventBus(String),
    InvalidContract(String),
    Internal(String),
}

impl fmt::Display for FoundationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RuntimeNotOperational => {
                write!(f, "MIDAS runtime is not operational")
            }

            Self::RuntimeStopping => {
                write!(f, "MIDAS runtime is stopping")
            }

            Self::SystemStopped => {
                write!(f, "MIDAS system is stopped")
            }

            Self::InvalidStateTransition { from, to } => {
                write!(
                    f,
                    "invalid lifecycle transition: {} -> {}",
                    from, to
                )
            }

            Self::ExecutionBlocked(reason) => {
                write!(f, "execution blocked: {}", reason)
            }

            Self::AuthorizationRequired(reason) => {
                write!(
                    f,
                    "authorization required: {}",
                    reason
                )
            }

            Self::ResourceUnavailable(resource) => {
                write!(
                    f,
                    "resource unavailable: {}",
                    resource
                )
            }

            Self::Transaction(reason) => {
                write!(f, "transaction error: {}", reason)
            }

            Self::Concurrency(reason) => {
                write!(f, "concurrency error: {}", reason)
            }

            Self::EventBus(reason) => {
                write!(f, "event bus error: {}", reason)
            }

            Self::InvalidContract(reason) => {
                write!(f, "invalid contract: {}", reason)
            }

            Self::Internal(reason) => {
                write!(f, "internal foundation error: {}", reason)
            }
        }
    }
}

impl std::error::Error for FoundationError {}

pub type FoundationResult<T> = Result<T, FoundationError>;
