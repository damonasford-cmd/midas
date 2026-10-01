use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub system_name: String,
    pub public_name: String,
    pub version: String,
    pub role: String,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            system_name: "MIDAS".to_string(),
            public_name: "Aeron Asford".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            role: "Unified autonomous intelligence system".to_string(),
        }
    }
}
