use crate::action::ActionResult;

#[derive(Debug, Clone)]
pub struct Observation {
    pub success: bool,
    pub result: String,
}

pub struct ObservationEngine;

impl ObservationEngine {
    pub fn observe(result: &ActionResult) -> Observation {
        Observation {
            success: result.success,
            result: result.description.clone(),
        }
    }
}
