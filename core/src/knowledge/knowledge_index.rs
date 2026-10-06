use std::collections::HashMap;

use super::knowledge_id::KnowledgeId;

#[derive(Debug, Clone)]
pub struct KnowledgeIndexEntry {
    pub knowledge_id: KnowledgeId,
    pub normalized_statement: String,
}

#[derive(Debug, Default)]
pub struct KnowledgeIndex {
    entries: HashMap<String, Vec<KnowledgeId>>,
}

impl KnowledgeIndex {
    pub fn index(
        &mut self,
        knowledge_id: KnowledgeId,
        statement: &str,
    ) {
        let key = statement.to_lowercase();

        let values =
            self.entries.entry(key).or_default();

        if !values.contains(&knowledge_id) {
            values.push(knowledge_id);
        }
    }

    pub fn search(
        &self,
        statement: &str,
    ) -> Vec<KnowledgeId> {
        self.entries
            .get(&statement.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
