use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum KnowledgeType {
    Fact,
    Observation,
    Inference,
    Hypothesis,
    Definition,
    Rule,
    Principle,
    Procedure,
    Model,
    Estimate,
    Unknown,
}
