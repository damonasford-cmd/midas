use crate::decision::Decision;

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub action: String,
    pub executed: bool,
    pub message: String,
}

pub fn execute(decision: &Decision) -> ActionResult {
    ActionResult {
        action: decision.action.clone(),
        executed: true,
        message: "Action interne simulée avec succès.".into(),
    }
}
