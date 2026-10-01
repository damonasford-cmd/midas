use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::learning::Learning;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correction {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub required: bool,
    pub action: String,
    pub reason: String,
    pub priority: CorrectionPriority,
    pub status: CorrectionStatus,
    pub changes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrectionPriority {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrectionStatus {
    NotRequired,
    Planned,
    Ready,
    Applied,
    Failed,
    Deferred,
}

impl Correction {
    pub fn none(
        source: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: source.into(),
            required: false,
            action:
                "Aucune correction nécessaire."
                    .to_string(),
            reason:
                "Le résultat observé ne nécessite pas de correction."
                    .to_string(),
            priority:
                CorrectionPriority::None,
            status:
                CorrectionStatus::NotRequired,
            changes: Vec::new(),
        }
    }

    pub fn required(
        source: impl Into<String>,
        action: impl Into<String>,
        reason: impl Into<String>,
        priority: CorrectionPriority,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: source.into(),
            required: true,
            action: action.into(),
            reason: reason.into(),
            priority,
            status:
                CorrectionStatus::Planned,
            changes: Vec::new(),
        }
    }

    pub fn add_change(
        &mut self,
        change: impl Into<String>,
    ) {
        self.changes.push(
            change.into(),
        );
    }

    pub fn mark_ready(
        &mut self,
    ) {
        self.status =
            CorrectionStatus::Ready;
    }

    pub fn mark_applied(
        &mut self,
    ) {
        self.status =
            CorrectionStatus::Applied;
    }

    pub fn mark_failed(
        &mut self,
    ) {
        self.status =
            CorrectionStatus::Failed;
    }

    pub fn defer(
        &mut self,
    ) {
        self.status =
            CorrectionStatus::Deferred;
    }

    pub fn summary(&self) -> String {
        format!(
            "Correction [{}] requise={} priorité={:?} \
             statut={:?} action={}",
            self.source,
            self.required,
            self.priority,
            self.status,
            self.action,
        )
    }
}

#[derive(Debug, Default)]
pub struct CorrectionEngine;

impl CorrectionEngine {
    pub fn evaluate(
        learning: &Learning,
    ) -> Correction {
        if learning.successful
            && learning.improvements.len() <= 1
        {
            return Correction::none(
                &learning.source,
            );
        }

        let priority =
            Self::determine_priority(
                learning,
            );

        let reason =
            if !learning.successful {
                "Le résultat observé indique un échec ou une divergence par rapport au résultat attendu."
                    .to_string()
            } else {
                "Le résultat est exploitable mais des améliorations ont été identifiées."
                    .to_string()
            };

        let action =
            if !learning.improvements.is_empty() {
                format!(
                    "Appliquer les améliorations identifiées : {}",
                    learning.improvements.join(" | ")
                )
            } else {
                "Analyser le comportement et déterminer une correction."
                    .to_string()
            };

        let mut correction =
            Correction::required(
                &learning.source,
                action,
                reason,
                priority,
            );

        for improvement in
            &learning.improvements
        {
            correction.add_change(
                improvement.clone(),
            );
        }

        correction.mark_ready();

        correction
    }

    fn determine_priority(
        learning: &Learning,
    ) -> CorrectionPriority {
        if !learning.successful
            && learning.confidence < 0.5
        {
            CorrectionPriority::Critical
        } else if !learning.successful
            && learning.confidence < 0.75
        {
            CorrectionPriority::High
        } else if !learning.successful {
            CorrectionPriority::Medium
        } else if !learning.improvements.is_empty() {
            CorrectionPriority::Low
        } else {
            CorrectionPriority::None
        }
    }

    pub fn should_retry(
        correction: &Correction,
    ) -> bool {
        matches!(
            correction.status,
            CorrectionStatus::Ready
                | CorrectionStatus::Applied
        )
    }

    pub fn can_apply(
        correction: &Correction,
    ) -> bool {
        correction.required
            && matches!(
                correction.status,
                CorrectionStatus::Planned
                    | CorrectionStatus::Ready
            )
    }
}
