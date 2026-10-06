use std::collections::HashMap;

use super::memory_id::MemoryId;

#[derive(Debug, Clone)]
pub struct MemoryIndexEntry {
    pub memory_id: MemoryId,
    pub tokens: Vec<String>,
}

#[derive(Debug, Default)]
pub struct MemoryIndex {
    inverted: HashMap<String, Vec<MemoryId>>,
}

impl MemoryIndex {
    pub fn index_text(
        &mut self,
        memory_id: MemoryId,
        text: &str,
    ) {
        let tokens = text
            .split_whitespace()
            .map(|token| {
                token
                    .to_lowercase()
                    .trim_matches(
                        |character: char| {
                            !character.is_alphanumeric()
                        },
                    )
                    .to_string()
            })
            .filter(|token| !token.is_empty());

        for token in tokens {
            let entry =
                self.inverted.entry(token).or_default();

            if !entry.contains(&memory_id) {
                entry.push(memory_id);
            }
        }
    }

    pub fn search(
        &self,
        token: &str,
    ) -> Vec<MemoryId> {
        self.inverted
            .get(&token.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    pub fn clear(&mut self) {
        self.inverted.clear();
    }
}
