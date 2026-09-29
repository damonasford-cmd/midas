#[derive(Debug, Clone)]
pub struct Identity {
    pub system_name: String,
    pub public_name: String,
    pub role: String,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            system_name: "MIDAS".into(),
            public_name: "Aeron Asford".into(),
            role: "Unified autonomous intelligence".into(),
        }
    }
}
