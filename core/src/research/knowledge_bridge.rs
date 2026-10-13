
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::research::evidence::{
    Evidence, VerificationLevel,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeCandidate {
    pub id: Uuid,
    pub statement: String,
    pub source: String,
    pub source_uri: Option<String>,
    pub verification: VerificationLevel,
    pub reliability: f64,
    pub limitations: Vec<String>,
    pub requires_review: bool,
}

impl From<&Evidence> for KnowledgeCandidate {
    fn from(evidence: &Evidence) -> Self {
        Self {
            id: evidence.id,
            statement: evidence.statement.clone(),
            source: evidence.source_name.clone(),
            source_uri: evidence.source_uri.clone(),
            verification: evidence.verification,
            reliability: evidence.reliability,
            limitations: evidence.limitations.clone(),
            requires_review: matches!(
                evidence.verification,
                VerificationLevel::Unchecked
                    | VerificationLevel::SingleSource
                    | VerificationLevel::Disputed
            ),
        }
    }
}

#[async_trait]
pub trait KnowledgePublisher: Send + Sync {
    async fn publish_candidate(
        &self,
        candidate: KnowledgeCandidate,
    ) -> Result<()>;
}

pub async fn publish_verified_candidates(
    publisher: &dyn KnowledgePublisher,
    evidence: &[Evidence],
) -> Result<usize> {
    let mut published = 0;

    for item in evidence {
        item.validate()?;

        let candidate = KnowledgeCandidate::from(item);
        publisher.publish_candidate(candidate).await?;
        published += 1;
    }

    Ok(published)
}
