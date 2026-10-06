use serde::{Deserialize, Serialize};

use super::{
    memory_id::MemoryId,
    memory_record::MemoryRecord,
    modality::MemoryModality,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryFilter {
    pub modality: Option<MemoryModality>,
    pub min_confidence: Option<f32>,
    pub min_importance: Option<f32>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub text: Option<String>,
    pub filter: MemoryFilter,
    pub limit: usize,
}

#[derive(Debug, Clone)]
pub struct MemoryQueryResult {
    pub memory_id: MemoryId,
    pub score: f32,
    pub record: MemoryRecord,
}

#[derive(Debug, Default)]
pub struct MemoryRetrieval;

impl MemoryRetrieval {
    pub fn filter(
        &self,
        records: Vec<MemoryRecord>,
        filter: &MemoryFilter,
    ) -> Vec<MemoryRecord> {
        records
            .into_iter()
            .filter(|record| {
                if let Some(modality) = filter.modality {
                    if record.modality != modality {
                        return false;
                    }
                }

                if let Some(min_confidence) =
                    filter.min_confidence
                {
                    if record.metadata.confidence
                        < min_confidence
                    {
                        return false;
                    }
                }

                if let Some(min_importance) =
                    filter.min_importance
                {
                    if record.metadata.importance
                        < min_importance
                    {
                        return false;
                    }
                }

                if !filter.tags.is_empty()
                    && !filter.tags.iter().all(|tag| {
                        record.metadata.tags.contains(tag)
                    })
                {
                    return false;
                }

                true
            })
            .collect()
    }

    pub fn rank(
        &self,
        records: Vec<MemoryRecord>,
        query: &MemoryQuery,
    ) -> Vec<MemoryQueryResult> {
        let mut results = records
            .into_iter()
            .map(|record| {
                let mut score =
                    record.metadata.importance
                        * record.metadata.confidence;

                if query.text.is_some() {
                    score += 0.1;
                }

                MemoryQueryResult {
                    memory_id: record.id,
                    score,
                    record,
                }
            })
            .collect::<Vec<_>>();

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(
                    std::cmp::Ordering::Equal,
                )
        });

        if query.limit > 0 {
            results.truncate(query.limit);
        }

        results
    }
}
