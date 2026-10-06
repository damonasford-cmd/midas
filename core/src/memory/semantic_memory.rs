use std::collections::HashMap;

use super::memory_id::MemoryId;

#[derive(Debug, Clone)]
pub struct SemanticMemoryEntry {
    pub concept: String,
    pub supporting_memories: Vec<MemoryId>,
    pub confidence: f32,
}

#[derive(Debug, Default)]
pub struct SemanticMemory {
    concepts: HashMap<String, SemanticMemoryEntry>,
}

impl SemanticMemory {
    pub fn add(
        &mut self,
        concept: impl Into<String>,
        memory_id: MemoryId,
        confidence: f32,
    ) -> Result<(), String> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(
                "semantic confidence must be between 0 and 1"
                    .into(),
            );
        }

        let concept = concept.into();

        if concept.trim().is_empty() {
            return Err(
                "semantic concept cannot be empty".into()
            );
        }

        let entry =
            self.concepts
                .entry(concept.clone())
                .or_insert_with(|| {
                    SemanticMemoryEntry {
                        concept,
                        supporting_memories: Vec::new(),
                        confidence,
                    }
                });

        if !entry.supporting_memories.contains(&memory_id) {
            entry.supporting_memories.push(memory_id);
        }

        entry.confidence =
            entry.confidence.max(confidence);

        Ok(())
    }

    pub fn get(
        &self,
        concept: &str,
    ) -> Option<&SemanticMemoryEntry> {
        self.concepts.get(concept)
    }
}
