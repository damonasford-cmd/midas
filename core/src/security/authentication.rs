use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCredential {
    pub id: Uuid,
    pub subject: String,
    pub method: AuthenticationMethod,
    pub issued_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    Password,
    Token,
    ApiKey,
    Biometric,
    Voice,
    HardwareKey,
    ExternalIdentity,
}

#[derive(Debug, Clone, Default)]
pub struct AuthenticationEngine;

impl AuthenticationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn credential(
        &self,
        subject: impl Into<String>,
        method: AuthenticationMethod,
    ) -> IdentityCredential {
        IdentityCredential {
            id: Uuid::new_v4(),
            subject: subject.into(),
            method,
            issued_at: Utc::now(),
        }
    }
}
