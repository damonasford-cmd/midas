use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Default)]
pub struct EncryptionService;

impl EncryptionService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_key(&self, key: &[u8]) -> Result<()> {
        if key.len() < 32 {
            return Err(anyhow!(
                "encryption key must contain at least 32 bytes"
            ));
        }

        Ok(())
    }

    pub fn fingerprint(&self, data: &[u8]) -> String {
        let mut state: u64 = 0xcbf29ce484222325;

        for byte in data {
            state ^= *byte as u64;
            state = state.wrapping_mul(0x100000001b3);
        }

        format!("{state:016x}")
    }
}
