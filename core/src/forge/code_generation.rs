use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSpecification {
    pub id: Uuid,
    pub language: String,
    pub purpose: String,
    pub requirements: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCode {
    pub id: Uuid,
    pub specification_id: Uuid,
    pub language: String,
    pub source: String,
    pub files: Vec<GeneratedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn specification(
        &self,
        language: impl Into<String>,
        purpose: impl Into<String>,
        requirements: Vec<String>,
    ) -> CodeSpecification {
        CodeSpecification {
            id: Uuid::new_v4(),
            language: language.into(),
            purpose: purpose.into(),
            requirements,
            constraints: Vec::new(),
        }
    }

    pub fn create_artifact(
        &self,
        specification: &CodeSpecification,
        source: impl Into<String>,
    ) -> GeneratedCode {
        GeneratedCode {
            id: Uuid::new_v4(),
            specification_id: specification.id,
            language: specification.language.clone(),
            source: source.into(),
            files: Vec::new(),
        }
    }
}
