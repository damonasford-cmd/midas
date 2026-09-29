use crate::action::ActionResult;

#[derive(Debug, Clone)]
pub struct Observation {
    pub result: String,
    pub success: bool,
}

pub fn observe(result: &ActionResult) -> Observation {
    Observation {
        result: result.message.clone(),
        success: result.executed,
    }
}
