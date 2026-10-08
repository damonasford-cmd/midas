
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalRelation {
    Causes,
    ContributesTo,
    Prevents,
    Enables,
    CorrelatesWith,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub id: Uuid,
    pub cause_id: Uuid,
    pub effect_id: Uuid,
    pub relation: CausalRelation,
    pub confidence: f32,
    pub evidence_fact_ids: Vec<Uuid>,
    pub explanation: String,
    pub created_at: DateTime<Utc>,
}

impl CausalLink {
    pub fn new(
        cause_id: Uuid,
        effect_id: Uuid,
        relation: CausalRelation,
        confidence: f32,
        explanation: impl Into<String>,
    ) -> Result<Self, String> {
        if cause_id == effect_id {
            return Err("causal link cannot connect an item to itself".into());
        }

        if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
            return Err("causal confidence must be between 0 and 1".into());
        }

        let explanation = explanation.into();

        if explanation.trim().is_empty() {
            return Err("causal explanation cannot be empty".into());
        }

        Ok(Self {
            id: Uuid::new_v4(),
            cause_id,
            effect_id,
            relation,
            confidence,
            evidence_fact_ids: Vec::new(),
            explanation,
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CausalityGraph {
    pub links: Vec<CausalLink>,
}

impl CausalityGraph {
    pub fn add_link(
        &mut self,
        link: CausalLink,
    ) -> Result<(), String> {
        if self.links.iter().any(|existing| {
            existing.cause_id == link.cause_id
                && existing.effect_id == link.effect_id
                && existing.relation == link.relation
        }) {
            return Err("duplicate causal link".into());
        }

        self.links.push(link);
        Ok(())
    }

    pub fn effects_of(&self, cause_id: Uuid) -> Vec<&CausalLink> {
        self.links
            .iter()
            .filter(|link| link.cause_id == cause_id)
            .collect()
    }

    pub fn causes_of(&self, effect_id: Uuid) -> Vec<&CausalLink> {
        self.links
            .iter()
            .filter(|link| link.effect_id == effect_id)
            .collect()
    }

    pub fn remove_link(&mut self, link_id: Uuid) -> bool {
        let original_len = self.links.len();
        self.links.retain(|link| link.id != link_id);
        original_len != self.links.len()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.links
            .iter()
            .map(|link| link.created_at)
            .max()
            .unwrap_or_else(Utc::now)
    }
}
