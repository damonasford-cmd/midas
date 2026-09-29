#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreStatus {
    Ready,
    Perceiving,
    Reasoning,
    Acting,
    Observing,
    Learning,
    Correcting,
}

#[derive(Debug, Clone)]
pub struct CoreState {
    pub status: CoreStatus,
    pub cycle: u64,
    pub last_goal: Option<String>,
    pub last_decision: Option<String>,
    pub last_result: Option<String>,
}

impl Default for CoreState {
    fn default() -> Self {
        Self {
            status: CoreStatus::Ready,
            cycle: 0,
            last_goal: None,
            last_decision: None,
            last_result: None,
        }
    }
}
