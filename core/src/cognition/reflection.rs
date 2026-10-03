use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::reasoning::ReasoningResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reflection {
    pub id: Uuid,
    pub reasoning_id: Uuid,
    pub complexity: Complexity,
    pub depth: ReflectionDepth,
    pub weaknesses: Vec<String>,
    pub missing_information: Vec<String>,
    pub recommended_checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Complexity {
    Simple,
    Moderate,
    Complex,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReflectionDepth {
    Short,
    Normal,
    Deep,
    Exhaustive,
}

pub struct ReflectionEngine;

impl Default for ReflectionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ReflectionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn reflect(
        &self,
        result: &ReasoningResult,
        complexity: Complexity,
    ) -> Reflection {
        let depth = match complexity {
            Complexity::Simple => ReflectionDepth::Short,
            Complexity::Moderate => ReflectionDepth::Normal,
            Complexity::Complex => ReflectionDepth::Deep,
            Complexity::Critical => ReflectionDepth::Exhaustive,
        };

        Reflection {
            id: Uuid::new_v4(),
            reasoning_id: result.context_id,
            complexity,
            depth,
            weaknesses: Vec::new(),
            missing_information: result.unresolved_questions.clone(),
            recommended_checks: Vec::new(),
        }
    }
}
