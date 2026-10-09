use anyhow::{bail, Context, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use super::{
    evidence::{Evidence, EvidencePolarity, NewEvidence},
    knowledge_graph::{
        KnowledgeRelation, KnowledgeRelationType, NewKnowledgeRelation,
    },
    knowledge_item::{
        KnowledgeItem, KnowledgeStatus, KnowledgeVersion, NewKnowledgeItem,
    },
    uncertainty::{
        NewUncertainty, UncertaintyKind, UncertaintyRecord,
    },
};

#[derive(Clone)]
pub struct KnowledgeService {
    pool: PgPool,
}

impl KnowledgeService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn initialize_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .context("knowledge database connection check failed")?;
        Ok(())
    }

    pub async fn create(&self, input: NewKnowledgeItem) -> Result<KnowledgeItem> {
        input.validate()?;

        let id = Uuid::new_v4();
        let hash = content_hash(&input)?;
        let snapshot = serde_json::to_value(&input)?;

        let mut tx = self.pool.begin().await?;

        let inserted = sqlx::query(
            r#"
            INSERT INTO knowledge_items (
                id, subject, predicate, object, statement,
                source, source_reference, status, confidence,
                version, valid_from, valid_until, metadata,
                content_hash, idempotency_key
            )
            VALUES (
                $1,$2,$3,$4,$5,$6,$7,$8,$9,1,$10,$11,$12,$13,$14
            )
            ON CONFLICT (idempotency_key) DO NOTHING
            RETURNING id
            "#,
        )
        .bind(id)
        .bind(input.subject.trim())
        .bind(input.predicate.trim())
        .bind(input.object.clone())
        .bind(input.statement.trim())
        .bind(input.source.trim())
        .bind(input.source_reference.clone())
        .bind(input.status.as_str())
        .bind(input.confidence)
        .bind(input.valid_from)
        .bind(input.valid_until)
        .bind(input.metadata.clone())
        .bind(&hash)
        .bind(input.idempotency_key.clone())
        .fetch_optional(&mut *tx)
        .await?;

        if inserted.is_none() {
            if let Some(key) = &input.idempotency_key {
                let existing = sqlx::query(
                    "SELECT id, content_hash FROM knowledge_items WHERE idempotency_key = $1",
                )
                .bind(key)
                .fetch_optional(&mut *tx)
                .await?
                .context("idempotency conflict occurred but existing item was not found")?;

                let existing_hash: String = existing.try_get("content_hash")?;
                if existing_hash != hash {
                    bail!("idempotency key was reused with different knowledge content");
                }

                let existing_id: Uuid = existing.try_get("id")?;
                tx.commit().await?;
                return self.get(existing_id).await?
                    .context("idempotent knowledge item disappeared");
            }
            bail!("knowledge item insert returned no row");
        }

        sqlx::query(
            r#"
            INSERT INTO knowledge_versions (
                knowledge_id, version, snapshot, content_hash, change_reason
            ) VALUES ($1, 1, $2, $3, 'initial creation')
            "#,
        )
        .bind(id)
        .bind(snapshot)
        .bind(&hash)
        .execute(&mut *tx)
        .await?;

        insert_outbox(
            &mut tx,
            "knowledge.created",
            json!({"knowledge_id": id, "version": 1}),
            format!("knowledge.created:{id}:1"),
        )
        .await?;

        tx.commit().await?;

        self.get(id).await?
            .context("created knowledge item could not be reloaded")
            .map_err(Into::into)
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<KnowledgeItem>> {
        let row = sqlx::query(
            r#"
            SELECT id, subject, predicate, object, statement,
                   source, source_reference, status, confidence,
                   version, valid_from, valid_until, metadata,
                   content_hash, created_at, updated_at
            FROM knowledge_items
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(map_knowledge_item).transpose()
    }

    pub async fn search_statement(
        &self,
        query: &str,
        limit: i64,
    ) -> Result<Vec<KnowledgeItem>> {
        if query.trim().is_empty() {
            bail!("knowledge search query cannot be empty");
        }
        if !(1..=500).contains(&limit) {
            bail!("knowledge search limit must be between 1 and 500");
        }

        let rows = sqlx::query(
            r#"
            SELECT id, subject, predicate, object, statement,
                   source, source_reference, status, confidence,
                   version, valid_from, valid_until, metadata,
                   content_hash, created_at, updated_at
            FROM knowledge_items
            WHERE to_tsvector('simple', statement)
                  @@ plainto_tsquery('simple', $1)
              AND status NOT IN ('rejected', 'superseded')
            ORDER BY ts_rank(
                to_tsvector('simple', statement),
                plainto_tsquery('simple', $1)
            ) DESC, updated_at DESC
            LIMIT $2
            "#,
        )
        .bind(query.trim())
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(map_knowledge_item).collect()
    }

    pub async fn add_evidence(
        &self,
        knowledge_id: Uuid,
        input: NewEvidence,
    ) -> Result<Evidence> {
        input.validate()?;
        let id = Uuid::new_v4();
        let mut tx = self.pool.begin().await?;

        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM knowledge_items WHERE id = $1)",
        )
        .bind(knowledge_id)
        .fetch_one(&mut *tx)
        .await?;

        if !exists {
            bail!("knowledge item does not exist");
        }

        let inserted = sqlx::query(
            r#"
            INSERT INTO knowledge_evidence (
                id, knowledge_id, polarity, description, source,
                source_reference, reliability, observed_at,
                metadata, idempotency_key
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            ON CONFLICT (idempotency_key) DO NOTHING
            RETURNING id
            "#,
        )
        .bind(id)
        .bind(knowledge_id)
        .bind(input.polarity.as_str())
        .bind(input.description.trim())
        .bind(input.source.trim())
        .bind(input.source_reference.clone())
        .bind(input.reliability)
        .bind(input.observed_at)
        .bind(input.metadata.clone())
        .bind(input.idempotency_key.clone())
        .fetch_optional(&mut *tx)
        .await?;

        let actual_id = match inserted {
            Some(_) => id,
            None => {
                let key = input.idempotency_key.as_ref()
                    .context("evidence insert unexpectedly conflicted")?;
                let row = sqlx::query(
                    "SELECT id, knowledge_id, polarity, description, source, source_reference, reliability, observed_at, metadata, created_at FROM knowledge_evidence WHERE idempotency_key = $1",
                )
                .bind(key)
                .fetch_one(&mut *tx)
                .await?;

                let evidence = map_evidence(row)?;
                if evidence.knowledge_id != knowledge_id
                    || evidence.description != input.description.trim()
                    || evidence.source != input.source.trim()
                    || evidence.polarity != input.polarity
                {
                    bail!("evidence idempotency key was reused with different content");
                }

                tx.commit().await?;
                return Ok(evidence);
            }
        };

        insert_outbox(
            &mut tx,
            "knowledge.evidence_added",
            json!({
                "evidence_id": actual_id,
                "knowledge_id": knowledge_id,
                "polarity": input.polarity.as_str()
            }),
            format!("knowledge.evidence_added:{actual_id}"),
        )
        .await?;

        let row = sqlx::query(
            r#"
            SELECT id, knowledge_id, polarity, description, source,
                   source_reference, reliability, observed_at, metadata, created_at
            FROM knowledge_evidence WHERE id = $1
            "#,
        )
        .bind(actual_id)
        .fetch_one(&mut *tx)
        .await?;

        let evidence = map_evidence(row)?;
        tx.commit().await?;
        Ok(evidence)
    }

    pub async fn add_uncertainty(
        &self,
        knowledge_id: Uuid,
        input: NewUncertainty,
    ) -> Result<UncertaintyRecord> {
        input.validate()?;
        let id = Uuid::new_v4();
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query(
            r#"
            INSERT INTO knowledge_uncertainties (
                id, knowledge_id, kind, description, severity, resolution_hint
            )
            SELECT $1, id, $3, $4, $5, $6
            FROM knowledge_items WHERE id = $2
            RETURNING id, knowledge_id, kind, description, severity,
                      resolution_hint, resolved_at, created_at
            "#,
        )
        .bind(id)
        .bind(knowledge_id)
        .bind(input.kind.as_str())
        .bind(input.description.trim())
        .bind(input.severity)
        .bind(input.resolution_hint.clone())
        .fetch_optional(&mut *tx)
        .await?
        .context("cannot add uncertainty: knowledge item does not exist")?;

        insert_outbox(
            &mut tx,
            "knowledge.uncertainty_added",
            json!({
                "uncertainty_id": id,
                "knowledge_id": knowledge_id,
                "kind": input.kind.as_str(),
                "severity": input.severity
            }),
            format!("knowledge.uncertainty_added:{id}"),
        )
        .await?;

        let uncertainty = map_uncertainty(row)?;
        tx.commit().await?;
        Ok(uncertainty)
    }

    pub async fn resolve_uncertainty(&self, uncertainty_id: Uuid) -> Result<bool> {
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query(
            r#"
            UPDATE knowledge_uncertainties
            SET resolved_at = NOW()
            WHERE id = $1 AND resolved_at IS NULL
            RETURNING id, knowledge_id
            "#,
        )
        .bind(uncertainty_id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(row) = row {
            let id: Uuid = row.try_get("id")?;
            let knowledge_id: Uuid = row.try_get("knowledge_id")?;
            insert_outbox(
                &mut tx,
                "knowledge.uncertainty_resolved",
                json!({"uncertainty_id": id, "knowledge_id": knowledge_id}),
                format!("knowledge.uncertainty_resolved:{id}"),
            )
            .await?;
            tx.commit().await?;
            Ok(true)
        } else {
            tx.commit().await?;
            Ok(false)
        }
    }

    pub async fn relate(
        &self,
        input: NewKnowledgeRelation,
    ) -> Result<KnowledgeRelation> {
        input.validate()?;
        let id = Uuid::new_v4();
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query(
            r#"
            INSERT INTO knowledge_relations (
                id, from_id, to_id, relation_type, explanation, metadata
            )
            VALUES ($1,$2,$3,$4,$5,$6)
            ON CONFLICT (from_id, to_id, relation_type) DO NOTHING
            RETURNING id, from_id, to_id, relation_type, explanation, metadata, created_at
            "#,
        )
        .bind(id)
        .bind(input.from_id)
        .bind(input.to_id)
        .bind(input.relation_type.as_str())
        .bind(input.explanation.trim())
        .bind(input.metadata.clone())
        .fetch_optional(&mut *tx)
        .await?;

        let relation = match row {
            Some(row) => map_relation(row)?,
            None => {
                let row = sqlx::query(
                    r#"
                    SELECT id, from_id, to_id, relation_type, explanation, metadata, created_at
                    FROM knowledge_relations
                    WHERE from_id = $1 AND to_id = $2 AND relation_type = $3
                    "#,
                )
                .bind(input.from_id)
                .bind(input.to_id)
                .bind(input.relation_type.as_str())
                .fetch_one(&mut *tx)
                .await?;
                map_relation(row)?
            }
        };

        insert_outbox(
            &mut tx,
            "knowledge.relation_added",
            json!({
                "relation_id": relation.id,
                "from_id": relation.from_id,
                "to_id": relation.to_id,
                "relation_type": relation.relation_type.as_str()
            }),
            format!("knowledge.relation_added:{}", relation.id),
        )
        .await?;

        tx.commit().await?;
        Ok(relation)
    }

    pub async fn list_evidence(&self, knowledge_id: Uuid) -> Result<Vec<Evidence>> {
        let rows = sqlx::query(
            r#"
            SELECT id, knowledge_id, polarity, description, source,
                   source_reference, reliability, observed_at, metadata, created_at
            FROM knowledge_evidence
            WHERE knowledge_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(knowledge_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(map_evidence).collect()
    }

    pub async fn list_uncertainties(
        &self,
        knowledge_id: Uuid,
        unresolved_only: bool,
    ) -> Result<Vec<UncertaintyRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT id, knowledge_id, kind, description, severity,
                   resolution_hint, resolved_at, created_at
            FROM knowledge_uncertainties
            WHERE knowledge_id = $1
              AND ($2 = FALSE OR resolved_at IS NULL)
            ORDER BY created_at DESC
            "#,
        )
        .bind(knowledge_id)
        .bind(unresolved_only)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(map_uncertainty).collect()
    }

    pub async fn list_relations(
        &self,
        knowledge_id: Uuid,
    ) -> Result<Vec<KnowledgeRelation>> {
        let rows = sqlx::query(
            r#"
            SELECT id, from_id, to_id, relation_type, explanation, metadata, created_at
            FROM knowledge_relations
            WHERE from_id = $1 OR to_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(knowledge_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(map_relation).collect()
    }

    pub async fn list_versions(&self, id: Uuid) -> Result<Vec<KnowledgeVersion>> {
        let rows = sqlx::query(
            r#"
            SELECT knowledge_id, version, snapshot, content_hash, changed_at, change_reason
            FROM knowledge_versions
            WHERE knowledge_id = $1
            ORDER BY version DESC
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(|row| {
            Ok(KnowledgeVersion {
                knowledge_id: row.try_get("knowledge_id")?,
                version: row.try_get("version")?,
                snapshot: row.try_get("snapshot")?,
                content_hash: row.try_get("content_hash")?,
                changed_at: row.try_get("changed_at")?,
                change_reason: row.try_get("change_reason")?,
            })
        }).collect()
    }
}

async fn insert_outbox(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event_type: &str,
    payload: Value,
    idempotency_key: String,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO knowledge_event_outbox (id, event_type, payload, idempotency_key)
        VALUES ($1,$2,$3,$4)
        ON CONFLICT (idempotency_key) DO NOTHING
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(event_type)
    .bind(payload)
    .bind(idempotency_key)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

fn content_hash(input: &NewKnowledgeItem) -> Result<String> {
    let bytes = serde_json::to_vec(input)?;
    let digest = Sha256::digest(bytes);
    Ok(format!("{digest:x}"))
}

fn map_knowledge_item(row: sqlx::postgres::PgRow) -> Result<KnowledgeItem> {
    let status: String = row.try_get("status")?;
    let status = match status.as_str() {
        "proposed" => KnowledgeStatus::Proposed,
        "supported" => KnowledgeStatus::Supported,
        "verified" => KnowledgeStatus::Verified,
        "disputed" => KnowledgeStatus::Disputed,
        "rejected" => KnowledgeStatus::Rejected,
        "superseded" => KnowledgeStatus::Superseded,
        _ => bail!("unknown knowledge status in database: {status}"),
    };

    Ok(KnowledgeItem {
        id: row.try_get("id")?,
        subject: row.try_get("subject")?,
        predicate: row.try_get("predicate")?,
        object: row.try_get("object")?,
        statement: row.try_get("statement")?,
        source: row.try_get("source")?,
        source_reference: row.try_get("source_reference")?,
        status,
        confidence: row.try_get("confidence")?,
        version: row.try_get("version")?,
        valid_from: row.try_get("valid_from")?,
        valid_until: row.try_get("valid_until")?,
        metadata: row.try_get("metadata")?,
        content_hash: row.try_get("content_hash")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_evidence(row: sqlx::postgres::PgRow) -> Result<Evidence> {
    let polarity: String = row.try_get("polarity")?;
    let polarity = match polarity.as_str() {
        "supports" => EvidencePolarity::Supports,
        "contradicts" => EvidencePolarity::Contradicts,
        "neutral" => EvidencePolarity::Neutral,
        _ => bail!("unknown evidence polarity in database: {polarity}"),
    };

    Ok(Evidence {
        id: row.try_get("id")?,
        knowledge_id: row.try_get("knowledge_id")?,
        polarity,
        description: row.try_get("description")?,
        source: row.try_get("source")?,
        source_reference: row.try_get("source_reference")?,
        reliability: row.try_get("reliability")?,
        observed_at: row.try_get("observed_at")?,
        metadata: row.try_get("metadata")?,
        created_at: row.try_get("created_at")?,
    })
}

fn map_uncertainty(row: sqlx::postgres::PgRow) -> Result<UncertaintyRecord> {
    let kind: String = row.try_get("kind")?;
    let kind = match kind.as_str() {
        "missing_evidence" => UncertaintyKind::MissingEvidence,
        "conflicting_evidence" => UncertaintyKind::ConflictingEvidence,
        "ambiguous_meaning" => UncertaintyKind::AmbiguousMeaning,
        "outdated_information" => UncertaintyKind::OutdatedInformation,
        "source_reliability" => UncertaintyKind::SourceReliability,
        "measurement_error" => UncertaintyKind::MeasurementError,
        "unknown" => UncertaintyKind::Unknown,
        _ => bail!("unknown uncertainty kind in database: {kind}"),
    };

    Ok(UncertaintyRecord {
        id: row.try_get("id")?,
        knowledge_id: row.try_get("knowledge_id")?,
        kind,
        description: row.try_get("description")?,
        severity: row.try_get("severity")?,
        resolution_hint: row.try_get("resolution_hint")?,
        resolved_at: row.try_get("resolved_at")?,
        created_at: row.try_get("created_at")?,
    })
}

fn map_relation(row: sqlx::postgres::PgRow) -> Result<KnowledgeRelation> {
    let relation_type: String = row.try_get("relation_type")?;
    let relation_type = match relation_type.as_str() {
        "supports" => KnowledgeRelationType::Supports,
        "contradicts" => KnowledgeRelationType::Contradicts,
        "depends_on" => KnowledgeRelationType::DependsOn,
        "derived_from" => KnowledgeRelationType::DerivedFrom,
        "refines" => KnowledgeRelationType::Refines,
        "supersedes" => KnowledgeRelationType::Supersedes,
        "related_to" => KnowledgeRelationType::RelatedTo,
        _ => bail!("unknown knowledge relation type in database: {relation_type}"),
    };

    Ok(KnowledgeRelation {
        id: row.try_get("id")?,
        from_id: row.try_get("from_id")?,
        to_id: row.try_get("to_id")?,
        relation_type,
        explanation: row.try_get("explanation")?,
        metadata: row.try_get("metadata")?,
        created_at: row.try_get("created_at")?,
    })
}
