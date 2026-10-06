use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatementValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    Structured(serde_json::Value),
    EntityReference(String),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeStatement {
    pub subject: String,
    pub predicate: String,
    pub object: StatementValue,

    pub normalized: Option<String>,

    pub language: Option<String>,
}

impl KnowledgeStatement {
    pub fn new(
        subject: impl Into<String>,
        predicate: impl Into<String>,
        object: StatementValue,
    ) -> Result<Self, String> {
        let subject = subject.into();
        let predicate = predicate.into();

        if subject.trim().is_empty() {
            return Err(
                "knowledge subject cannot be empty".into()
            );
        }

        if predicate.trim().is_empty() {
            return Err(
                "knowledge predicate cannot be empty".into()
            );
        }

        Ok(Self {
            subject,
            predicate,
            object,
            normalized: None,
            language: None,
        })
    }
}
