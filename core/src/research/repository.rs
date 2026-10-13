
use anyhow::{Context, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::research::discovery::ResearchReport;

#[derive(Clone)]
pub struct ResearchRepository {
    pool: PgPool,
}

impl ResearchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, report: &ResearchReport) -> Result<()> {
        let payload = serde_json::to_value(report)
            .context("Sérialisation du rapport de recherche impossible")?;

        sqlx::query(
            r#"
            INSERT INTO research_runs (
                id, objective_id, title, status, report, created_at, completed_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                report = EXCLUDED.report,
                completed_at = EXCLUDED.completed_at
            "#,
        )
        .bind(report.run.id)
        .bind(report.run.objective.id)
        .bind(&report.run.objective.title)
        .bind(format!("{:?}", report.run.status).to_lowercase())
        .bind(payload)
        .bind(report.run.started_at)
        .bind(report.run.completed_at)
        .execute(&self.pool)
        .await
        .context("Enregistrement du rapport impossible")?;

        Ok(())
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<ResearchReport>> {
        let payload: Option<serde_json::Value> = sqlx::query_scalar(
            "SELECT report FROM research_runs WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .context("Lecture du rapport impossible")?;

        payload
            .map(|value| serde_json::from_value(value)
                .context("Rapport enregistré invalide"))
            .transpose()
    }
}
