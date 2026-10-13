use serde_json::{json, Value};
use uuid::Uuid;

pub const CAPABILITY_REGISTERED: &str = "capability.registered";
pub const CAPABILITY_STATE_CHANGED: &str = "capability.state_changed";
pub const CAPABILITY_EVALUATED: &str = "capability.evaluated";
pub const CAPABILITY_DEPENDENCY_ADDED: &str = "capability.dependency_added";

pub fn registered_payload(
    id: Uuid,
    key: &str,
    revision: i64,
) -> Value {
    json!({
        "capability_id": id,
        "capability_key": key,
        "revision": revision
    })
}

pub fn state_changed_payload(
    id: Uuid,
    key: &str,
    previous: &str,
    next: &str,
    reason: &str,
    revision: i64,
) -> Value {
    json!({
        "capability_id": id,
        "capability_key": key,
        "previous_state": previous,
        "next_state": next,
        "reason": reason,
        "revision": revision
    })
}

pub fn evaluated_payload(
    evaluation_id: Uuid,
    capability_id: Uuid,
    successful: bool,
    score: f32,
    valid_until: &str,
) -> Value {
    json!({
        "evaluation_id": evaluation_id,
        "capability_id": capability_id,
        "successful": successful,
        "score": score,
        "valid_until": valid_until
    })
}

pub fn dependency_added_payload(
    capability_id: Uuid,
    capability_key: &str,
    dependency_key: &str,
) -> Value {
    json!({
        "capability_id": capability_id,
        "capability_key": capability_key,
        "dependency_key": dependency_key
    })
}
