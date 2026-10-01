use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::action::ActionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub result: String,
    pub success: bool,
    pub action_executed: bool,
    pub changes_detected: Vec<String>,
    pub anomalies: Vec<String>,
    pub metadata: serde_json::Value,
}

impl Observation {
    pub fn new(
        source: impl Into<String>,
        result: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: source.into(),
            result: result.into(),
            success: false,
            action_executed: false,
            changes_detected: Vec::new(),
            anomalies: Vec::new(),
            metadata: serde_json::Value::Object(
                serde_json::Map::new(),
            ),
        }
    }

    pub fn with_success(
        mut self,
        success: bool,
    ) -> Self {
        self.success = success;
        self
    }

    pub fn with_execution(
        mut self,
        executed: bool,
    ) -> Self {
        self.action_executed = executed;
        self
    }

    pub fn add_change(
        &mut self,
        change: impl Into<String>,
    ) {
        self.changes_detected.push(
            change.into(),
        );
    }

    pub fn add_anomaly(
        &mut self,
        anomaly: impl Into<String>,
    ) {
        self.anomalies.push(
            anomaly.into(),
        );
    }

    pub fn with_metadata(
        mut self,
        metadata: serde_json::Value,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn has_anomalies(&self) -> bool {
        !self.anomalies.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "[{}] succès={} exécutée={} changements={} anomalies={}",
            self.source,
            self.success,
            self.action_executed,
            self.changes_detected.len(),
            self.anomalies.len(),
        )
    }
}

#[derive(Debug, Default)]
pub struct ObservationEngine;

impl ObservationEngine {
    pub fn observe(
        action: &ActionResult,
    ) -> Observation {
        let mut observation =
            Observation::new(
                "action",
                &action.message,
            )
            .with_success(action.success)
            .with_execution(action.executed);

        if action.executed {
            observation.add_change(
                "L'action a été exécutée.",
            );
        } else {
            observation.add_change(
                "L'action n'a pas été exécutée.",
            );
        }

        if !action.success {
            observation.add_anomaly(
                "L'action n'a pas produit un résultat considéré comme réussi.",
            );
        }

        observation
    }

    pub fn compare(
        before: &Observation,
        after: &Observation,
    ) -> Vec<String> {
        let mut changes =
            Vec::new();

        if before.success
            != after.success
        {
            changes.push(format!(
                "État de succès modifié : {} → {}",
                before.success,
                after.success
            ));
        }

        if before.action_executed
            != after.action_executed
        {
            changes.push(format!(
                "État d'exécution modifié : {} → {}",
                before.action_executed,
                after.action_executed
            ));
        }

        if before.result
            != after.result
        {
            changes.push(
                "Le résultat observé a changé."
                    .to_string(),
            );
        }

        changes
    }
}
