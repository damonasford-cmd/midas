
use std::{
    collections::HashMap,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    foundation::{
        contracts::{Event, EventKind},
        event_bus::EventBus,
    },
    identity::MidasIdentity,
};

use super::{
    causality::{CausalLink, CausalityGraph},
    entity::Entity,
    fact::Fact,
    location::Location,
    provenance::Provenance,
    resource::Resource,
    temporal::TemporalRecord,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldModelSnapshot {
    pub revision: u64,
    pub entities: HashMap<Uuid, Entity>,
    pub locations: HashMap<Uuid, Location>,
    pub resources: HashMap<Uuid, Resource>,
    pub provenances: HashMap<Uuid, Provenance>,
    pub facts: HashMap<Uuid, Fact>,
    pub temporal_records: HashMap<Uuid, TemporalRecord>,
    pub causality: CausalityGraph,
}

impl WorldModelSnapshot {
    pub fn contains_object(&self, id: Uuid) -> bool {
        self.entities.contains_key(&id)
            || self.locations.contains_key(&id)
            || self.resources.contains_key(&id)
            || self.provenances.contains_key(&id)
            || self.facts.contains_key(&id)
            || self.temporal_records.contains_key(&id)
    }
}

#[derive(Clone)]
pub struct WorldModel {
    identity_id: Uuid,
    state: Arc<RwLock<WorldModelSnapshot>>,
    event_bus: EventBus,
}

impl WorldModel {
    pub fn new(
        identity: &MidasIdentity,
        event_bus: EventBus,
    ) -> Result<Self, String> {
        identity.validate()?;

        Ok(Self {
            identity_id: identity.id,
            state: Arc::new(RwLock::new(
                WorldModelSnapshot::default(),
            )),
            event_bus,
        })
    }

    pub fn identity_id(&self) -> Uuid {
        self.identity_id
    }

    pub async fn snapshot(&self) -> WorldModelSnapshot {
        self.state.read().await.clone()
    }

    async fn publish_change(
        &self,
        object_type: &'static str,
        object_id: Uuid,
        revision: u64,
    ) -> Result<(), String> {
        let event = Event::new(
            EventKind::Custom,
            "world.world_model",
            json!({
                "operation": "world_object_registered",
                "object_type": object_type,
                "object_id": object_id,
                "revision": revision,
                "identity_id": self.identity_id,
            }),
        );

        self.event_bus
            .publish(event)
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    pub async fn add_entity(
        &self,
        entity: Entity,
    ) -> Result<Uuid, String> {
        let id = entity.id;

        let revision = {
            let mut state = self.state.write().await;

            if state.entities.contains_key(&id) {
                return Err("entity ID already exists".into());
            }

            state.entities.insert(id, entity);
            state.revision += 1;
            state.revision
        };

        self.publish_change("entity", id, revision).await?;
        Ok(id)
    }

    pub async fn add_location(
        &self,
        location: Location,
    ) -> Result<Uuid, String> {
        let id = location.id;

        let revision = {
            let mut state = self.state.write().await;

            if state.locations.contains_key(&id) {
                return Err("location ID already exists".into());
            }

            if let Some(parent_id) = location.parent_id {
                if !state.locations.contains_key(&parent_id) {
                    return Err("parent location does not exist".into());
                }
            }

            state.locations.insert(id, location);
            state.revision += 1;
            state.revision
        };

        self.publish_change("location", id, revision).await?;
        Ok(id)
    }

    pub async fn add_resource(
        &self,
        resource: Resource,
    ) -> Result<Uuid, String> {
        let id = resource.id;

        let revision = {
            let mut state = self.state.write().await;

            if state.resources.contains_key(&id) {
                return Err("resource ID already exists".into());
            }

            if let Some(owner_id) = resource.owner_entity_id {
                if !state.entities.contains_key(&owner_id) {
                    return Err("resource owner entity does not exist".into());
                }
            }

            if let Some(location_id) = resource.location_id {
                if !state.locations.contains_key(&location_id) {
                    return Err("resource location does not exist".into());
                }
            }

            if let Some(provenance_id) = resource.source_provenance_id {
                if !state.provenances.contains_key(&provenance_id) {
                    return Err("resource provenance does not exist".into());
                }
            }

            state.resources.insert(id, resource);
            state.revision += 1;
            state.revision
        };

        self.publish_change("resource", id, revision).await?;
        Ok(id)
    }

    pub async fn add_provenance(
        &self,
        provenance: Provenance,
    ) -> Result<Uuid, String> {
        let id = provenance.id;

        let revision = {
            let mut state = self.state.write().await;

            if state.provenances.contains_key(&id) {
                return Err("provenance ID already exists".into());
            }

            state.provenances.insert(id, provenance);
            state.revision += 1;
            state.revision
        };

        self.publish_change("provenance", id, revision).await?;
        Ok(id)
    }

    pub async fn add_fact(
        &self,
        fact: Fact,
    ) -> Result<Uuid, String> {
        let id = fact.id;

        let revision = {
            let mut state = self.state.write().await;

            if state.facts.contains_key(&id) {
                return Err("fact ID already exists".into());
            }

            if let Some(subject_id) = fact.subject_entity_id {
                if !state.entities.contains_key(&subject_id) {
                    return Err("fact subject entity does not exist".into());
                }
            }

            for provenance_id in &fact.provenance_ids {
                if !state.provenances.contains_key(provenance_id) {
                    return Err(format!(
                        "fact provenance does not exist: {provenance_id}"
                    ));
                }
            }

            state.facts.insert(id, fact);
            state.revision += 1;
            state.revision
        };

        self.publish_change("fact", id, revision).await?;
        Ok(id)
    }

    pub async fn add_temporal_record(
        &self,
        record: TemporalRecord,
    ) -> Result<Uuid, String> {
        let id = record.id;

        let revision = {
            let mut state = self.state.write().await;

            if state.temporal_records.contains_key(&id) {
                return Err("temporal record ID already exists".into());
            }

            if let Some(subject_id) = record.subject_id {
                if !state.contains_object(subject_id) {
                    return Err("temporal record subject does not exist".into());
                }
            }

            if let Some(provenance_id) = record.source_provenance_id {
                if !state.provenances.contains_key(&provenance_id) {
                    return Err("temporal record provenance does not exist".into());
                }
            }

            state.temporal_records.insert(id, record);
            state.revision += 1;
            state.revision
        };

        self.publish_change("temporal_record", id, revision).await?;
        Ok(id)
    }

    pub async fn add_causal_link(
        &self,
        link: CausalLink,
    ) -> Result<Uuid, String> {
        let id = link.id;

        let revision = {
            let mut state = self.state.write().await;

            if !state.contains_object(link.cause_id) {
                return Err("causal source object does not exist".into());
            }

            if !state.contains_object(link.effect_id) {
                return Err("causal target object does not exist".into());
            }

            for fact_id in &link.evidence_fact_ids {
                if !state.facts.contains_key(fact_id) {
                    return Err(format!(
                        "causal evidence fact does not exist: {fact_id}"
                    ));
                }
            }

            state.causality.add_link(link)?;
            state.revision += 1;
            state.revision
        };

        self.publish_change("causal_link", id, revision).await?;
        Ok(id)
    }

    pub async fn get_entity(
        &self,
        id: Uuid,
    ) -> Option<Entity> {
        self.state.read().await.entities.get(&id).cloned()
    }

    pub async fn find_entity_by_name(
        &self,
        name: &str,
    ) -> Vec<Entity> {
        self.state
            .read()
            .await
            .entities
            .values()
            .filter(|entity| entity.matches_name(name))
            .cloned()
            .collect()
    }

    pub async fn get_fact(
        &self,
        id: Uuid,
    ) -> Option<Fact> {
        self.state.read().await.facts.get(&id).cloned()
    }

    pub async fn get_resource(
        &self,
        id: Uuid,
    ) -> Option<Resource> {
        self.state.read().await.resources.get(&id).cloned()
    }

    pub async fn counts(&self) -> WorldModelCounts {
        let state = self.state.read().await;

        WorldModelCounts {
            revision: state.revision,
            entities: state.entities.len(),
            locations: state.locations.len(),
            resources: state.resources.len(),
            provenances: state.provenances.len(),
            facts: state.facts.len(),
            temporal_records: state.temporal_records.len(),
            causal_links: state.causality.links.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelCounts {
    pub revision: u64,
    pub entities: usize,
    pub locations: usize,
    pub resources: usize,
    pub provenances: usize,
    pub facts: usize,
    pub temporal_records: usize,
    pub causal_links: usize,
}
