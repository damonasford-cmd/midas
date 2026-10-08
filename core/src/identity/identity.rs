use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{
    Digest,
    Sha256,
};
use uuid::Uuid;

use super::{
    autobiography::Autobiography,
    continuity::IdentityContinuity,
    identity_evolution::IdentityEvolution,
    identity_state::{
        IdentityOperationalState,
        IdentityState,
    },
    principles::PrincipleSet,
    self_model::SelfModel,
    self_reference::{
        SelfReference,
        SelfReferenceKind,
    },
    values::ValueSet,
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
pub enum IdentityKind {
    UnifiedArtificialIntelligence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidasIdentity {
    pub id: Uuid,

    pub kind: IdentityKind,

    pub system_name: String,

    pub public_name: String,

    pub creator: String,

    pub unified: bool,

    pub state: IdentityState,

    pub operational_state:
        IdentityOperationalState,

    pub self_model: SelfModel,

    pub self_references:
        Vec<SelfReference>,

    pub values: ValueSet,

    pub principles: PrincipleSet,

    pub continuity: IdentityContinuity,

    pub autobiography: Autobiography,

    pub evolution: IdentityEvolution,

    pub version: u64,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    pub integrity_digest: String,
}

impl MidasIdentity {
    pub fn new() -> Result<Self, String> {
        let id = Uuid::new_v4();

        let mut identity = Self {
            id,
            kind:
                IdentityKind::UnifiedArtificialIntelligence,

            system_name:
                "MIDAS".into(),

            public_name:
                "Aeron Asford".into(),

            creator:
                "Damon".into(),

            unified:
                true,

            state:
                IdentityState::Initializing,

            operational_state:
                IdentityOperationalState::Stable,

            self_model:
                SelfModel::new(),

            self_references:
                Vec::new(),

            values:
                ValueSet::constitutional(),

            principles:
                PrincipleSet::constitutional(),

            continuity:
                IdentityContinuity::new(id),

            autobiography:
                Autobiography::new(),

            evolution:
                IdentityEvolution::new(),

            version:
                1,

            created_at:
                Utc::now(),

            updated_at:
                Utc::now(),

            integrity_digest:
                String::new(),
        };

        identity.register_canonical_references()?;
        identity.refresh_integrity()?;
        identity.validate()?;

        Ok(identity)
    }

    fn register_canonical_references(
        &mut self,
    ) -> Result<(), String> {
        self.self_references.push(
            SelfReference::new(
                SelfReferenceKind::Internal,
                "MIDAS",
                "Canonical internal system identity.",
                true,
            )?,
        );

        self.self_references.push(
            SelfReference::new(
                SelfReferenceKind::Public,
                "Aeron Asford",
                "Canonical public identity of MIDAS.",
                true,
            )?,
        );

        self.self_references.push(
            SelfReference::new(
                SelfReferenceKind::CreatorRelation,
                "Damon",
                "Creator and constitutional authority relationship.",
                true,
            )?,
        );

        Ok(())
    }

    pub fn activate(
        &mut self,
    ) -> Result<(), String> {
        if self.state
            != IdentityState::Initializing
            && self.state
                != IdentityState::Recovering
        {
            return Err(format!(
                "identity cannot activate from state {:?}",
                self.state
            ));
        }

        self.state = IdentityState::Active;
        self.operational_state =
            IdentityOperationalState::Stable;

        self.version += 1;
        self.updated_at = Utc::now();

        self.record_history(
            "identity_activated",
            "MIDAS identity became active.",
        )?;

        self.refresh_integrity()?;

        Ok(())
    }

    pub fn suspend(
        &mut self,
        reason: impl Into<String>,
    ) -> Result<(), String> {
        if self.state != IdentityState::Active
            && self.state
                != IdentityState::Degraded
        {
            return Err(format!(
                "identity cannot suspend from state {:?}",
                self.state
            ));
        }

        self.state = IdentityState::Suspended;
        self.operational_state =
            IdentityOperationalState::Restricted;

        self.version += 1;
        self.updated_at = Utc::now();

        self.record_history(
            "identity_suspended",
            reason.into(),
        )?;

        self.refresh_integrity()?;

        Ok(())
    }

    pub fn begin_recovery(
        &mut self,
    ) -> Result<(), String> {
        if self.state
            != IdentityState::Suspended
            && self.state
                != IdentityState::Degraded
        {
            return Err(format!(
                "identity cannot recover from state {:?}",
                self.state
            ));
        }

        self.state = IdentityState::Recovering;
        self.operational_state =
            IdentityOperationalState::Recovering;

        self.version += 1;
        self.updated_at = Utc::now();

        self.record_history(
            "identity_recovery_started",
            "Identity recovery started.",
        )?;

        self.refresh_integrity()?;

        Ok(())
    }

    pub fn change_operational_state(
        &mut self,
        state: IdentityOperationalState,
        reason: impl Into<String>,
        authorized: bool,
    ) -> Result<(), String> {
        if !authorized {
            return Err(
                "unauthorized identity state change"
                    .into(),
            );
        }

        self.operational_state = state;
        self.version += 1;
        self.updated_at = Utc::now();

        self.record_history(
            "operational_state_changed",
            reason.into(),
        )?;

        self.refresh_integrity()?;

        Ok(())
    }

    pub fn record_history(
        &mut self,
        event_type: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<(), String> {
        let event = super::autobiography::AutobiographicalEvent::new(
            event_type,
            description,
        )?;

        self.autobiography.record(event);

        Ok(())
    }

    pub fn create_continuity_checkpoint(
        &mut self,
    ) -> Result<
        super::continuity::ContinuityCheckpoint,
        String,
    > {
        self.refresh_integrity()?;

        self.continuity
            .checkpoint(self.integrity_digest.clone())
    }

    pub fn refresh_integrity(
        &mut self,
    ) -> Result<(), String> {
        let canonical = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.id,
            self.system_name,
            self.public_name,
            self.creator,
            self.unified,
            self.version,
            self.state_string(),
        );

        let mut hasher = Sha256::new();

        hasher.update(canonical.as_bytes());

        self.integrity_digest =
            format!("{:x}", hasher.finalize());

        Ok(())
    }

    pub fn verify_integrity(
        &self,
    ) -> Result<(), String> {
        let canonical = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.id,
            self.system_name,
            self.public_name,
            self.creator,
            self.unified,
            self.version,
            self.state_string(),
        );

        let mut hasher = Sha256::new();

        hasher.update(canonical.as_bytes());

        let expected =
            format!("{:x}", hasher.finalize());

        if expected != self.integrity_digest {
            return Err(
                "identity integrity verification failed"
                    .into(),
            );
        }

        Ok(())
    }

    fn state_string(
        &self,
    ) -> String {
        format!(
            "{:?}|{:?}",
            self.state,
            self.operational_state
        )
    }

    pub fn validate(
        &self,
    ) -> Result<(), String> {
        if self.system_name != "MIDAS" {
            return Err(
                "canonical system name is MIDAS".into()
            );
        }

        if self.public_name != "Aeron Asford" {
            return Err(
                "canonical public identity is Aeron Asford"
                    .into(),
            );
        }

        if self.creator != "Damon" {
            return Err(
                "canonical creator relation is Damon"
                    .into(),
            );
        }

        if !self.unified {
            return Err(
                "MIDAS identity must remain unified"
                    .into(),
            );
        }

        if !self
            .values
            .contains(
                super::values::CoreValue::Continuity,
            )
        {
            return Err(
                "continuity value is mandatory"
                    .into(),
            );
        }

        super::identity_evolution::IdentityEvolution::validate_constitutional_integrity(
            self.state,
            self.operational_state,
            self.principles.all(),
        )?;

        self.verify_integrity()?;

        Ok(())
    }
}

impl Default for MidasIdentity {
    fn default() -> Self {
        Self::new()
            .expect("canonical MIDAS identity must initialize")
    }
}
