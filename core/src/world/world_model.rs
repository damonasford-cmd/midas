use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use super::{
    causality::CausalRelation,
    entity::{Entity, EntityId},
    environment::{Environment, EnvironmentKind},
    event::WorldEvent,
    fact::{Fact, FactStatus},
    location::Location,
    resource::Resource,
};

#[derive(Debug, Clone, Default)]
pub struct WorldModel {
    pub entities: HashMap<EntityId, Entity>,
    pub locations: HashMap<Uuid, Location>,
    pub environments: HashMap<Uuid, Environment>,
    pub resources: HashMap<Uuid, Resource>,
    pub facts: HashMap<Uuid, Fact>,
    pub events: Vec<WorldEvent>,
    pub causal_relations: HashMap<Uuid, CausalRelation>,
    pub unknowns: Vec<String>,
}

pub type SharedWorldModel = Arc<RwLock<WorldModel>>;

impl WorldModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity(
        &mut self,
        entity: Entity,
    ) -> EntityId {
        let id = entity.id;

        self.entities.insert(id, entity);

        id
    }

    pub fn get_entity(
        &self,
        id: &EntityId,
    ) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub fn get_entity_mut(
        &mut self,
        id: &EntityId,
    ) -> Option<&mut Entity> {
        self.entities.get_mut(id)
    }

    pub fn remove_entity(
        &mut self,
        id: &EntityId,
    ) -> Option<Entity> {
        self.entities.remove(id)
    }

    pub fn register_location(
        &mut self,
        location: Location,
    ) -> Uuid {
        let id = location.id;

        self.locations.insert(id, location);

        id
    }

    pub fn register_environment(
        &mut self,
        environment: Environment,
    ) -> Uuid {
        let id = environment.id;

        self.environments.insert(id, environment);

        id
    }

    pub fn find_environments_by_kind(
        &self,
        kind: &EnvironmentKind,
    ) -> Vec<&Environment> {
        self.environments
            .values()
            .filter(|environment| &environment.kind == kind)
            .collect()
    }

    pub fn register_resource(
        &mut self,
        resource: Resource,
    ) -> Uuid {
        let id = resource.id;

        self.resources.insert(id, resource);

        id
    }

    pub fn register_fact(
        &mut self,
        fact: Fact,
    ) -> Uuid {
        let id = fact.id;

        self.facts.insert(id, fact);

        id
    }

    pub fn verify_fact(
        &mut self,
        id: &Uuid,
    ) -> Result<(), String> {
        let fact = self
            .facts
            .get_mut(id)
            .ok_or_else(|| "Fait introuvable".to_string())?;

        fact.set_status(FactStatus::Verified);

        Ok(())
    }

    pub fn register_event(
        &mut self,
        event: WorldEvent,
    ) {
        self.events.push(event);
    }

    pub fn register_causal_relation(
        &mut self,
        relation: CausalRelation,
    ) -> Uuid {
        let id = relation.id;

        self.causal_relations.insert(id, relation);

        id
    }

    pub fn register_unknown(
        &mut self,
        unknown: impl Into<String>,
    ) {
        let value = unknown.into();

        if !self.unknowns.contains(&value) {
            self.unknowns.push(value);
        }
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn location_count(&self) -> usize {
        self.locations.len()
    }

    pub fn environment_count(&self) -> usize {
        self.environments.len()
    }

    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn causal_relation_count(&self) -> usize {
        self.causal_relations.len()
    }

    pub fn unknown_count(&self) -> usize {
        self.unknowns.len()
    }
}
