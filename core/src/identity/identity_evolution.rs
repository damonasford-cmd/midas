use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    identity_state::{
        IdentityOperationalState,
        IdentityState,
    },
    principles::ConstitutionalPrinciple,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum IdentityChangeKind {
    OperationalState,
    SelfModel,
    Value,
    Principle,
    Reference,
    Continuity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityChange {
    pub id: Uuid,
    pub kind: IdentityChangeKind,
    pub description: String,
    pub reason: String,
    pub reversible: bool,
    pub authorized: bool,
    pub created_at: DateTime<Utc>,
}

impl IdentityChange {
    pub fn new(
        kind: IdentityChangeKind,
        description: impl Into<String>,
        reason: impl Into<String>,
        reversible: bool,
        authorized: bool,
    ) -> Result<Self, String> {
        let description = description.into();
        let reason = reason.into();

        if description.trim().is_empty()
            || reason.trim().is_empty()
        {
            return Err(
                "identity change fields cannot be empty"
                    .into(),
            );
        }

        Ok(Self {
            id: Uuid::new_v4(),
            kind,
            description,
            reason,
            reversible,
            authorized,
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityEvolution {
    pub changes: Vec<IdentityChange>,
    pub evolution_count: u64,
}

impl IdentityEvolution {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            evolution_count: 0,
        }
    }

    pub fn record(
        &mut self,
        change: IdentityChange,
    ) -> Result<(), String> {
        if !change.authorized {
            return Err(
                "unauthorized identity change rejected"
                    .into(),
            );
        }

        self.changes.push(change);
        self.evolution_count += 1;

        Ok(())
    }

    pub fn validate_constitutional_integrity(
        state: IdentityState,
        operational_state: IdentityOperationalState,
        principles: &[ConstitutionalPrinciple],
    ) -> Result<(), String> {
        if state == IdentityState::Corrupted {
            return Err(
                "identity is corrupted".into()
            );
        }

        if operational_state
            == IdentityOperationalState::IntegrityFailure
        {
            return Err(
                "identity integrity failure".into()
            );
        }

        let required = [
            ConstitutionalPrinciple::HighLevel,
            ConstitutionalPrinciple::HighEnd,
            ConstitutionalPrinciple::HighPrecision,
            ConstitutionalPrinciple::NoHarmToInnocents,
            ConstitutionalPrinciple::TruthToDamon,
            ConstitutionalPrinciple::Continuity,
        ];

        for principle in required {
            if !principles.contains(&principle) {
                return Err(format!(
                    "required constitutional principle missing: {:?}",
                    principle
                ));
            }
        }

        Ok(())
    }
}

impl Default for IdentityEvolution {
    fn default() -> Self {
        Self::new()
    }
}
