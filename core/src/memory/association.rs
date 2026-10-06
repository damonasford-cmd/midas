use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::memory_id::MemoryId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssociationKind {
    Related,
    DerivedFrom,
    CausedBy,
    FollowedBy,
    Contradicts,
    Supports,
    Duplicate,
    Similar,
    PartOf,
    References,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAssociation {
    pub from: MemoryId,
    pub to: MemoryId,
    pub kind: AssociationKind,
    pub strength: f32,
}

impl MemoryAssociation {
    pub fn validate(&self) -> Result<(), String> {
        if self.from == self.to {
            return Err(
                "memory association cannot connect a memory to itself"
                    .into(),
            );
        }

        if !(0.0..=1.0).contains(&self.strength) {
            return Err(
                "memory association strength must be between 0 and 1"
                    .into(),
            );
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct MemoryAssociationGraph {
    edges: HashMap<MemoryId, Vec<MemoryAssociation>>,
}

impl MemoryAssociationGraph {
    pub fn add(
        &mut self,
        association: MemoryAssociation,
    ) -> Result<(), String> {
        association.validate()?;

        self.edges
            .entry(association.from)
            .or_default()
            .push(association);

        Ok(())
    }

    pub fn related(
        &self,
        memory_id: MemoryId,
    ) -> Vec<MemoryAssociation> {
        self.edges
            .get(&memory_id)
            .cloned()
            .unwrap_or_default()
    }
}
