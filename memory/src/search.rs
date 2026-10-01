use anyhow::Result;

use super::{
    store::MemoryStore,
    types::{Memory, MemoryKind, MemoryModality},
};

pub struct MemorySearch;

impl MemorySearch {
    pub fn text(
        memories: &[Memory],
        query: &str,
    ) -> Vec<Memory> {
        let query = query.to_lowercase();

        memories
            .iter()
            .filter(|memory| {
                memory
                    .content
                    .to_lowercase()
                    .contains(&query)
                    || memory
                        .source
                        .to_lowercase()
                        .contains(&query)
                    || memory.tags.iter().any(|tag| {
                        tag.to_lowercase()
                            .contains(&query)
                    })
            })
            .cloned()
            .collect()
    }

    pub fn by_kind(
        memories: &[Memory],
        kind: &MemoryKind,
    ) -> Vec<Memory> {
        memories
            .iter()
            .filter(|memory| {
                std::mem::discriminant(
                    &memory.kind,
                )
                    == std::mem::discriminant(kind)
            })
            .cloned()
            .collect()
    }

    pub fn by_modality(
        memories: &[Memory],
        modality: &MemoryModality,
    ) -> Vec<Memory> {
        memories
            .iter()
            .filter(|memory| {
                std::mem::discriminant(
                    &memory.modality,
                )
                    == std::mem::discriminant(modality)
            })
            .cloned()
            .collect()
    }

    pub fn important(
        memories: &[Memory],
        minimum: f32,
    ) -> Vec<Memory> {
        memories
            .iter()
            .filter(|memory| {
                memory.importance >= minimum
            })
            .cloned()
            .collect()
    }

    pub fn search_store(
        store: &MemoryStore,
        query: &str,
    ) -> Result<Vec<Memory>> {
        let memories = store.read_all()?;

        Ok(Self::text(&memories, query))
    }
}
