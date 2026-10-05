use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::MidasIdentity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentityChangeKind {
    Initialization,
    ValidatedEvolution,
    PrincipleChange,
    ValueChange,
    SelfModelChange,
    ContinuityUpdate,
    Recovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityChange {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub kind: IdentityChangeKind,
    pub previous_version: u64,
    pub new_version: u64,
    pub reason: String,
    pub authorized: bool,
}

impl IdentityChange {
    pub fn new(
        kind: IdentityChangeKind,
        previous_version: u64,
        new_version: u64,
        reason: impl Into<String>,
        authorized: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            kind,
            previous_version,
            new_version,
            reason: reason.into(),
            authorized,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityEvolution {
    pub changes: Vec<IdentityChange>,
}

impl IdentityEvolution {
    pub fn record(&mut self, change: IdentityChange) {
        self.changes.push(change);
    }

    pub fn can_modify_immutable_principle(
        identity: &MidasIdentity,
        principle_id: &str,
    ) -> bool {
        match identity.principles.get(principle_id) {
            Some(principle) => !principle.immutable,
            None => false,
        }
    }

    pub fn validate_identity(identity: &MidasIdentity) -> Result<(), String> {
        identity.validate()?;
        identity.state.validate()?;

        if !identity.unified {
            return Err(
                "Une identité MIDAS non unifiée est interdite".to_string(),
            );
        }

        if identity.self_reference.canonical() != "MIDAS" {
            return Err(
                "L'auto-référence canonique doit rester MIDAS".to_string(),
            );
        }

        if identity.self_reference.public() != "Aeron Asford" {
            return Err(
                "L'identité publique doit rester Aeron Asford".to_string(),
            );
        }

        Ok(())
    }

    pub fn evolve(
        &mut self,
        identity: &mut MidasIdentity,
        kind: IdentityChangeKind,
        reason: impl Into<String>,
        authorized: bool,
    ) -> Result<(), String> {
        if !authorized {
            return Err(
                "Une évolution identitaire doit être explicitement autorisée."
                    .to_string(),
            );
        }

        let previous_version = identity.version;

        identity.increment_version();

        let new_version = identity.version;

        Self::validate_identity(identity)?;

        self.record(IdentityChange::new(
            kind,
            previous_version,
            new_version,
            reason,
            authorized,
        ));

        Ok(())
    }
}
