use std::sync::Arc;

use tokio::sync::RwLock;

use super::{
    certainty::{
        CertaintyLevel,
        CertaintyStatus,
    },
    contradiction::{
        Contradiction,
        ContradictionResolution,
        ContradictionSet,
    },
    confidence::ConfidenceScore,
    evidence::Evidence,
    freshness::{
        FreshnessPolicy,
        FreshnessStatus,
    },
    health::{
        KnowledgeHealth,
        KnowledgeHealthState,
    },
    hypothesis::Hypothesis,
    inference::Inference,
    knowledge_id::KnowledgeId,
    knowledge_index::KnowledgeIndex,
    knowledge_store::KnowledgeStore,
    knowledge_type::KnowledgeType,
    provenance::KnowledgeProvenance,
    relation::KnowledgeRelation,
    statement::KnowledgeStatement,
    temporal::KnowledgeTemporalScope,
    unknown::Unknown,
    validation::{
        KnowledgeValidation,
        ValidationStatus,
    },
};

#[derive(Debug, Clone)]
pub struct KnowledgeConfig {
    pub freshness_policy: FreshnessPolicy,

    pub minimum_confidence_for_established:
        f32,

    pub maximum_contradictions_before_degraded:
        usize,
}

impl Default for KnowledgeConfig {
    fn default() -> Self {
        Self {
            freshness_policy:
                FreshnessPolicy::default(),

            minimum_confidence_for_established:
                0.9,

            maximum_contradictions_before_degraded:
                0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeOperationResult {
    Stored(KnowledgeId),
    AlreadyExists(KnowledgeId),
    Removed(KnowledgeId),
    NotFound,
}

#[derive(Debug, Clone)]
pub struct Knowledge {
    pub id: KnowledgeId,

    pub kind: KnowledgeType,

    pub statement: KnowledgeStatement,

    pub certainty: CertaintyStatus,

    pub confidence: ConfidenceScore,

    pub provenance: Vec<KnowledgeProvenance>,

    pub evidence: Vec<Evidence>,

    pub temporal:
        KnowledgeTemporalScope,

    pub hypotheses: Vec<Hypothesis>,

    pub inferences: Vec<Inference>,

    pub validation: KnowledgeValidation,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Knowledge {
    pub fn new(
        kind: KnowledgeType,
        statement: KnowledgeStatement,
    ) -> Self {
        let id = KnowledgeId::new();

        Self {
            id,

            kind,

            statement,

            certainty:
                CertaintyStatus {
                    level: CertaintyLevel::Unknown,
                    score: 0.0,
                    reason: None,
                },

            confidence:
                ConfidenceScore::default(),

            provenance: Vec::new(),

            evidence: Vec::new(),

            temporal:
                KnowledgeTemporalScope::default(),

            hypotheses: Vec::new(),

            inferences: Vec::new(),

            validation:
                KnowledgeValidation::new(id),

            created_at:
                chrono::Utc::now(),

            updated_at:
                chrono::Utc::now(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0)
            .contains(&self.confidence.value)
        {
            return Err(
                "knowledge confidence must be between 0 and 1"
                    .into()
            );
        }

        if !(0.0..=1.0)
            .contains(&self.certainty.score)
        {
            return Err(
                "knowledge certainty must be between 0 and 1"
                    .into()
            );
        }

        for provenance in &self.provenance {
            provenance.validate()?;
        }

        for evidence in &self.evidence {
            evidence.validate()?;
        }

        for hypothesis in &self.hypotheses {
            hypothesis.validate()?;
        }

        for inference in &self.inferences {
            inference.validate()?;
        }

        Ok(())
    }

    pub fn is_established(
        &self,
        threshold: f32,
    ) -> bool {
        self.confidence.value >= threshold
            && matches!(
                self.certainty.level,
                CertaintyLevel::Established
                    | CertaintyLevel::Strong
            )
            && self.validation.status
                == ValidationStatus::Validated
    }
}

pub struct KnowledgeEngine {
    config: KnowledgeConfig,

    store: KnowledgeStore,

    index:
        Arc<RwLock<KnowledgeIndex>>,

    contradictions:
        Arc<RwLock<ContradictionSet>>,

    relations:
        Arc<RwLock<Vec<KnowledgeRelation>>>,

    unknowns:
        Arc<RwLock<Vec<Unknown>>>,

    active:
        Arc<RwLock<bool>>,
}

impl KnowledgeEngine {
    pub fn new(
        config: KnowledgeConfig,
    ) -> Self {
        Self {
            config,

            store:
                KnowledgeStore::default(),

            index:
                Arc::new(
                    RwLock::new(
                        KnowledgeIndex::default(),
                    ),
                ),

            contradictions:
                Arc::new(
                    RwLock::new(
                        ContradictionSet::default(),
                    ),
                ),

            relations:
                Arc::new(
                    RwLock::new(Vec::new()),
                ),

            unknowns:
                Arc::new(
                    RwLock::new(Vec::new()),
                ),

            active:
                Arc::new(
                    RwLock::new(false),
                ),
        }
    }

    pub async fn start(&self) {
        *self.active.write().await = true;
    }

    pub async fn stop(&self) {
        *self.active.write().await = false;
    }

    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }

    pub async fn store(
        &self,
        knowledge: Knowledge,
    ) -> Result<
        KnowledgeOperationResult,
        String,
    > {
        if !self.is_active().await {
            return Err(
                "knowledge subsystem is not active"
                    .into()
            );
        }

        if self.store
            .get(knowledge.id)
            .await
            .is_some()
        {
            return Ok(
                KnowledgeOperationResult::AlreadyExists(
                    knowledge.id,
                )
            );
        }

        let id = knowledge.id;

        let normalized =
            format!(
                "{} {}",
                knowledge.statement.subject,
                knowledge.statement.predicate
            );

        self.index
            .write()
            .await
            .index(id, &normalized);

        self.store
            .insert(knowledge)
            .await?;

        Ok(
            KnowledgeOperationResult::Stored(id)
        )
    }

    pub async fn get(
        &self,
        id: KnowledgeId,
    ) -> Option<Knowledge> {
        self.store
            .get(id)
            .await
            .map(|entry| entry.knowledge)
    }

    pub async fn add_evidence(
        &self,
        id: KnowledgeId,
        evidence: Evidence,
    ) -> Result<(), String> {
        evidence.validate()?;

        let mut knowledge =
            self.get(id)
                .await
                .ok_or_else(|| {
                    "knowledge item not found"
                        .to_string()
                })?;

        knowledge.evidence.push(evidence);

        self.recalculate_confidence(
            &mut knowledge,
        );

        knowledge.updated_at =
            chrono::Utc::now();

        self.store.insert(knowledge).await
    }

    fn recalculate_confidence(
        &self,
        knowledge: &mut Knowledge,
    ) {
        if knowledge.evidence.is_empty() {
            knowledge.confidence.value = 0.0;
            knowledge.certainty.level =
                CertaintyLevel::Unknown;
            return;
        }

        let total: f32 =
            knowledge
                .evidence
                .iter()
                .map(|evidence| {
                    let strength =
                        match evidence.strength {
                            super::evidence::EvidenceStrength::VeryWeak => 0.1,
                            super::evidence::EvidenceStrength::Weak => 0.3,
                            super::evidence::EvidenceStrength::Moderate => 0.5,
                            super::evidence::EvidenceStrength::Strong => 0.75,
                            super::evidence::EvidenceStrength::VeryStrong => 1.0,
                        };

                    strength
                        * evidence.reliability
                })
                .sum();

        let count =
            knowledge.evidence.len() as f32;

        let value =
            (total / count).clamp(0.0, 1.0);

        knowledge.confidence.value =
            value;

        knowledge.confidence.evidence_weight =
            value;

        knowledge.certainty =
            if value >= self.config
                .minimum_confidence_for_established
            {
                CertaintyStatus {
                    level:
                        CertaintyLevel::Established,
                    score: value,
                    reason:
                        Some(
                            "strong accumulated evidence"
                                .into(),
                        ),
                }
            } else if value >= 0.75 {
                CertaintyStatus {
                    level:
                        CertaintyLevel::Strong,
                    score: value,
                    reason: None,
                }
            } else if value >= 0.5 {
                CertaintyStatus {
                    level:
                        CertaintyLevel::Probable,
                    score: value,
                    reason: None,
                }
            } else if value >= 0.25 {
                CertaintyStatus {
                    level:
                        CertaintyLevel::Possible,
                    score: value,
                    reason: None,
                }
            } else {
                CertaintyStatus {
                    level:
                        CertaintyLevel::Speculative,
                    score: value,
                    reason: None,
                }
            };
    }

    pub async fn register_contradiction(
        &self,
        first: KnowledgeId,
        second: KnowledgeId,
    ) -> Result<(), String> {
        let contradiction =
            Contradiction::new(
                first,
                second,
            )?;

        self.contradictions
            .write()
            .await
            .add(contradiction);

        Ok(())
    }

    pub async fn resolve_contradiction(
        &self,
        index: usize,
        resolution:
            ContradictionResolution,
    ) -> Result<(), String> {
        let mut contradictions =
            self.contradictions
                .write()
                .await;

        let all =
            contradictions
                .all()
                .to_vec();

        if index >= all.len() {
            return Err(
                "contradiction index out of bounds"
                    .into()
            );
        }

        let mut replacement =
            all[index].clone();

        replacement.resolution =
            resolution;

        if let Some(reason) =
            contradictions
                .all()
                .get(index)
                .and_then(|item| {
                    item.explanation.clone()
                })
        {
            replacement.explanation =
                Some(reason);
        }

        let mut rebuilt =
            ContradictionSet::default();

        for (position, item)
            in all.into_iter().enumerate()
        {
            if position == index {
                rebuilt.add(replacement.clone());
            } else {
                rebuilt.add(item);
            }
        }

        *contradictions = rebuilt;

        Ok(())
    }

    pub async fn add_relation(
        &self,
        relation: KnowledgeRelation,
    ) -> Result<(), String> {
        relation.validate()?;

        self.relations
            .write()
            .await
            .push(relation);

        Ok(())
    }

    pub async fn add_unknown(
        &self,
        unknown: Unknown,
    ) {
        self.unknowns
            .write()
            .await
            .push(unknown);
    }

    pub async fn freshness(
        &self,
        id: KnowledgeId,
    ) -> Option<FreshnessStatus> {
        let knowledge =
            self.get(id).await?;

        Some(
            self.config
                .freshness_policy
                .evaluate(
                    knowledge
                        .provenance
                        .first()
                        .map(|item| {
                            item.acquired_at
                        })
                        .unwrap_or(
                            knowledge.created_at
                        ),
                ),
        )
    }

    pub async fn health(
        &self,
    ) -> KnowledgeHealth {
        let all =
            self.store.all().await;

        let contradictions =
            self.contradictions
                .read()
                .await
                .unresolved()
                .len();

        let unknowns =
            self.unknowns
                .read()
                .await
                .len();

        let facts =
            all.iter()
                .filter(|item| {
                    item.knowledge.kind
                        == KnowledgeType::Fact
                })
                .count();

        let hypotheses =
            all.iter()
                .filter(|item| {
                    item.knowledge.kind
                        == KnowledgeType::Hypothesis
                })
                .count();

        let validated =
            all.iter()
                .filter(|item| {
                    item.knowledge
                        .validation
                        .status
                        == ValidationStatus::Validated
                })
                .count();

        let state =
            if !self.is_active().await {
                KnowledgeHealthState::Unavailable
            } else if contradictions
                > self.config
                    .maximum_contradictions_before_degraded
            {
                KnowledgeHealthState::Contradictory
            } else {
                KnowledgeHealthState::Healthy
            };

        KnowledgeHealth {
            state,

            total_items:
                all.len(),

            facts,
            hypotheses,
            unknowns,

            unresolved_contradictions:
                contradictions,

            validated_items:
                validated,

            integrity_errors:
                0,

            retrieval_available:
                true,
        }
    }

    pub fn store(
        &self,
    ) -> KnowledgeStore {
        self.store.clone()
    }

    pub fn index(
        &self,
    ) -> Arc<RwLock<KnowledgeIndex>> {
        self.index.clone()
    }
}
