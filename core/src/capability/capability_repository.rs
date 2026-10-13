use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::{
    capability::{Capability, NewCapability},
    capability_evaluation::CapabilityEvaluation,
    capability_events::CapabilityEvent,
    capability_state::CapabilityState,
    capability_policy::CapabilityPolicy,
};

#[derive(Debug, Clone)]
pub struct CapabilityRepository {
    pool: PgPool,
}

#[derive(Debug, FromRow)]
struct CapabilityRow {
    id: Uuid,
    name: String,
    description: String,
    version: String,
    state: String,
    required_permissions: Value,
    metadata: Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    evaluation_valid_until: Option<DateTime<Utc>>,
}

impl CapabilityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn register(&self, input: NewCapability) -> Result<Capability> {
        input.validate()?;

        let mut tx = self.pool.begin().await?;
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO capabilities (
                id, name, description, version, state,
                required_permissions, metadata
            )
            VALUES ($1, $2, $3, $4, 'declared', $5, $6)
            "#,
        )
        .bind(id)
        .bind(input.name.trim())
        .bind(input.description.trim())
        .bind(input.version.trim())
        .bind(json!(input.required_permissions))
        .bind(input.metadata)
        .execute(&mut *tx)
        .await
        .context("Could not insert capability")?;

        for dependency_id in &input.dependencies {
            if *dependency_id == id {
                bail!("A capability cannot depend on itself");
            }

            sqlx::query(
                r#"
                INSERT INTO capability_dependencies
                    (capability_id, dependency_id)
                VALUES ($1, $2)
                "#,
            )
            .bind(id)
            .bind(dependency_id)
            .execute(&mut *tx)
            .await
            .context("Could not insert capability dependency")?;
        }

        let row = Self::fetch_row_tx(&mut tx, id).await?;
        let mut capability = Self::row_to_capability(row)?;
        capability.dependencies = input.dependencies;

        Self::insert_event_tx(
            &mut tx,
            &CapabilityEvent::registered(&capability),
        )
        .await?;

        tx.commit().await?;
        Ok(capability)
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Capability>> {
        let row = sqlx::query_as::<_, CapabilityRow>(
            r#"
            SELECT id, name, description, version, state,
                   required_permissions, metadata, created_at,
                   updated_at, evaluation_valid_until
            FROM capabilities
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let mut capability = Self::row_to_capability(row)?;

        capability.dependencies = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT dependency_id
            FROM capability_dependencies
            WHERE capability_id = $1
            ORDER BY dependency_id
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(capability))
    }

    pub async fn save_evaluation(
        &self,
        evaluation: CapabilityEvaluation,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM capabilities WHERE id = $1)",
        )
        .bind(evaluation.capability_id)
        .fetch_one(&mut *tx)
        .await?;

        if !exists {
            bail!("Cannot evaluate an unknown capability");
        }

        sqlx::query(
            r#"
            INSERT INTO capability_evaluations (
                id, capability_id, successful, score, checks, evidence,
                started_at, completed_at, valid_until
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(evaluation.id)
        .bind(evaluation.capability_id)
        .bind(evaluation.successful)
        .bind(evaluation.score)
        .bind(json!(evaluation.checks))
        .bind(evaluation.evidence.clone())
        .bind(evaluation.started_at)
        .bind(evaluation.completed_at)
        .bind(evaluation.valid_until)
        .execute(&mut *tx)
        .await
        .context("Could not persist capability evaluation")?;

        // Only the latest evaluation determines the current validity.
        let latest = Self::latest_evaluation_tx(
            &mut tx,
            evaluation.capability_id,
        )
        .await?;

        sqlx::query(
            r#"
            UPDATE capabilities
            SET evaluation_valid_until = $2,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(evaluation.capability_id)
        .bind(latest.as_ref().map(|e| e.valid_until))
        .execute(&mut *tx)
        .await?;

        Self::insert_event_tx(
            &mut tx,
            &CapabilityEvent::evaluated(&evaluation),
        )
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn set_state(
        &self,
        id: Uuid,
        next: CapabilityState,
    ) -> Result<Capability> {
        let mut tx = self.pool.begin().await?;

        // Serialize state transitions for this capability.
        let row = sqlx::query_as::<_, CapabilityRow>(
            r#"
            SELECT id, name, description, version, state,
                   required_permissions, metadata, created_at,
                   updated_at, evaluation_valid_until
            FROM capabilities
            WHERE id = $1
            FOR UPDATE
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow!("Capability not found: {id}"))?;

        let previous = CapabilityState::try_from(row.state.as_str())
            .map_err(anyhow::Error::msg)?;

        let latest = Self::latest_evaluation_tx(&mut tx, id).await?;
        let now = Utc::now();

        let latest_is_current = latest
            .as_ref()
            .map(|evaluation| evaluation.is_current(now))
            .unwrap_or(false);

        CapabilityPolicy::validate_transition(
            previous,
            next,
            latest_is_current,
        )?;

        // Dependencies must also be currently available before this
        // capability can be marked available.
        if next == CapabilityState::Available {
            let unavailable_dependency: bool = sqlx::query_scalar(
                r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM capability_dependencies d
                    JOIN capabilities dep ON dep.id = d.dependency_id
                    LEFT JOIN LATERAL (
                        SELECT e.successful, e.valid_until
                        FROM capability_evaluations e
                        WHERE e.capability_id = dep.id
                        ORDER BY e.completed_at DESC
                        LIMIT 1
                    ) latest ON TRUE
                    WHERE d.capability_id = $1
                      AND (
                          dep.state <> 'available'
                          OR latest.successful IS DISTINCT FROM TRUE
                          OR latest.valid_until IS NULL
                          OR latest.valid_until <= NOW()
                      )
                )
                "#,
            )
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;

            if unavailable_dependency {
                bail!(
                    "Cannot mark capability available while a dependency is unavailable or unevaluated"
                );
            }
        }

        sqlx::query(
            r#"
            UPDATE capabilities
            SET state = $2, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(next.as_str())
        .execute(&mut *tx)
        .await?;

        Self::insert_event_tx(
            &mut tx,
            &CapabilityEvent::state_changed(id, previous, next),
        )
        .await?;

        tx.commit().await?;

        self.get(id)
            .await?
            .ok_or_else(|| anyhow!("Capability disappeared after state update"))
    }

    pub async fn add_dependency(
        &self,
        capability_id: Uuid,
        dependency_id: Uuid,
    ) -> Result<()> {
        if capability_id == dependency_id {
            bail!("A capability cannot depend on itself");
        }

        let mut tx = self.pool.begin().await?;

        // Serialize graph mutations so concurrent insertions cannot
        // independently pass a cycle check and jointly create a cycle.
        sqlx::query("SELECT pg_advisory_xact_lock(734821)")
            .execute(&mut *tx)
            .await?;

        let path_exists: bool = sqlx::query_scalar(
            r#"
            WITH RECURSIVE reachable(id) AS (
                SELECT dependency_id
                FROM capability_dependencies
                WHERE capability_id = $2

                UNION

                SELECT d.dependency_id
                FROM capability_dependencies d
                JOIN reachable r ON d.capability_id = r.id
            )
            SELECT EXISTS(SELECT 1 FROM reachable WHERE id = $1)
            "#,
        )
        .bind(capability_id)
        .bind(dependency_id)
        .fetch_one(&mut *tx)
        .await?;

        if path_exists {
            bail!("Dependency would introduce a cycle");
        }

        sqlx::query(
            r#"
            INSERT INTO capability_dependencies (capability_id, dependency_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(capability_id)
        .bind(dependency_id)
        .execute(&mut *tx)
        .await
        .context("Could not persist capability dependency")?;

        sqlx::query(
            "UPDATE capabilities SET updated_at = NOW() WHERE id = $1",
        )
        .bind(capability_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn latest_evaluation_tx(
        tx: &mut Transaction<'_, Postgres>,
        capability_id: Uuid,
    ) -> Result<Option<CapabilityEvaluation>> {
        #[derive(FromRow)]
        struct EvaluationRow {
            id: Uuid,
            capability_id: Uuid,
            successful: bool,
            score: f64,
            checks: Value,
            evidence: Value,
            started_at: DateTime<Utc>,
            completed_at: DateTime<Utc>,
            valid_until: DateTime<Utc>,
        }

        let row = sqlx::query_as::<_, EvaluationRow>(
            r#"
            SELECT id, capability_id, successful, score, checks, evidence,
                   started_at, completed_at, valid_until
            FROM capability_evaluations
            WHERE capability_id = $1
            ORDER BY completed_at DESC, id DESC
            LIMIT 1
            "#,
        )
        .bind(capability_id)
        .fetch_optional(&mut **tx)
        .await?;

        row.map(|r| {
            Ok(CapabilityEvaluation {
                id: r.id,
                capability_id: r.capability_id,
                successful: r.successful,
                score: r.score,
                checks: serde_json::from_value(r.checks)?,
                evidence: r.evidence,
                started_at: r.started_at,
                completed_at: r.completed_at,
                valid_until: r.valid_until,
            })
        })
        .transpose()
    }

    async fn fetch_row_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<CapabilityRow> {
        sqlx::query_as::<_, CapabilityRow>(
            r#"
            SELECT id, name, description, version, state,
                   required_permissions, metadata, created_at,
                   updated_at, evaluation_valid_until
            FROM capabilities
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&mut **tx)
        .await
        .map_err(Into::into)
    }

    fn row_to_capability(row: CapabilityRow) -> Result<Capability> {
        Ok(Capability {
            id: row.id,
            name: row.name,
            description: row.description,
            version: row.version,
            state: CapabilityState::try_from(row.state.as_str())
                .map_err(anyhow::Error::msg)?,
            required_permissions: serde_json::from_value(
                row.required_permissions,
            )?,
            dependencies: Vec::new(),
            metadata: row.metadata,
            created_at: row.created_at,
            updated_at: row.updated_at,
            evaluation_valid_until: row.evaluation_valid_until,
        })
    }

    async fn insert_event_tx(
        tx: &mut Transaction<'_, Postgres>,
        event: &CapabilityEvent,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO capability_event_outbox (
                id, event_type, source, payload, occurred_at
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(event.event_id)
        .bind(&event.event_type)
        .bind(&event.source)
        .bind(&event.payload)
        .bind(event.occurred_at)
        .execute(&mut **tx)
        .await
        .context("Could not persist capability outbox event")?;

        Ok(())
    }
}
