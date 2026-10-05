use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    IdentityContinuity,
    IdentityOperationalState,
    IdentityState,
    PrincipleSet,
    SelfModel,
    SelfReference,
    ValueSet,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentityKind {
    UnifiedArtificialIntelligence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidasIdentity {
    pub id: Uuid,

    /// Nom système fondamental.
    pub system_name: String,

    /// Nom public/persona d'identité.
    /// MIDAS = Aeron Asford : il ne s'agit pas d'une seconde entité.
    pub public_name: String,

    /// Créateur / autorité humaine de référence.
    pub creator_name: String,

    /// Nature de l'identité.
    pub kind: IdentityKind,

    /// Toujours vrai pour MIDAS :
    /// une identité unique et continue.
    pub unified: bool,

    /// État courant de l'identité.
    pub state: IdentityState,

    /// État opérationnel de l'identité.
    pub operational_state: IdentityOperationalState,

    /// Modèle interne de soi.
    pub self_model: SelfModel,

    /// Continuité temporelle.
    pub continuity: IdentityContinuity,

    /// Auto-référence structurée.
    pub self_reference: SelfReference,

    /// Valeurs fondamentales.
    pub values: ValueSet,

    /// Principes constitutionnels.
    pub principles: PrincipleSet,

    /// Date de création de cette identité logique.
    pub created_at: DateTime<Utc>,

    /// Dernière modification structurelle de l'identité.
    pub updated_at: DateTime<Utc>,

    /// Numéro de version de l'identité.
    pub version: u64,
}

impl MidasIdentity {
    pub fn new() -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            system_name: "MIDAS".to_string(),
            public_name: "Aeron Asford".to_string(),
            creator_name: "Damon".to_string(),
            kind: IdentityKind::UnifiedArtificialIntelligence,
            unified: true,
            state: IdentityState::default(),
            operational_state: IdentityOperationalState::Initialized,
            self_model: SelfModel::default(),
            continuity: IdentityContinuity::new(now),
            self_reference: SelfReference::new(),
            values: ValueSet::default(),
            principles: PrincipleSet::default(),
            created_at: now,
            updated_at: now,
            version: 1,
        }
    }

    pub fn is_unified(&self) -> bool {
        self.unified
    }

    pub fn canonical_name(&self) -> &str {
        &self.system_name
    }

    pub fn public_name(&self) -> &str {
        &self.public_name
    }

    pub fn creator(&self) -> &str {
        &self.creator_name
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn increment_version(&mut self) {
        self.version = self.version.saturating_add(1);
        self.touch();
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.system_name != "MIDAS" {
            return Err("L'identité système doit être MIDAS".to_string());
        }

        if self.public_name != "Aeron Asford" {
            return Err("L'identité publique doit être Aeron Asford".to_string());
        }

        if self.creator_name != "Damon" {
            return Err("Le créateur doit être Damon".to_string());
        }

        if !self.unified {
            return Err(
                "MIDAS doit rester une identité unifiée et continue".to_string(),
            );
        }

        if self.version == 0 {
            return Err("La version d'identité ne peut pas être zéro".to_string());
        }

        Ok(())
    }
}

impl Default for MidasIdentity {
    fn default() -> Self {
        Self::new()
    }
}
