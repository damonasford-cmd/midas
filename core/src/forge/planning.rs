use anyhow::Result;
use async_trait::async_trait;

use crate::forge::types::{
    ForgeObjective, ForgePlan, ForgeReport,
};

#[derive(Debug, Clone)]
pub struct ForgePlanningContext {
    pub identity_snapshot: serde_json::Value,
    pub active_goals: serde_json::Value,
    pub world_state: serde_json::Value,
    pub available_capabilities: serde_json::Value,
    pub resource_state: serde_json::Value,
    pub previous_attempts: serde_json::Value,
}

#[async_trait]
pub trait ForgePlanner: Send + Sync {
    async fn create_plan(
        &self,
        objective: &ForgeObjective,
        context: &ForgePlanningContext,
    ) -> Result<ForgePlan>;

    async fn revise_plan(
        &self,
        objective: &ForgeObjective,
        previous_plan: &ForgePlan,
        report: &ForgeReport,
        context: &ForgePlanningContext,
    ) -> Result<ForgePlan>;
}
