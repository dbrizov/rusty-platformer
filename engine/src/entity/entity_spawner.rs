use std::collections::HashSet;
use std::mem;

use crate::entity::{Entity, EntityId};

pub struct EntitySpawner {
    m_next_entity_id: EntityId,
    m_entities: Vec<Box<Entity>>,
    m_entity_spawn_requests: Vec<Box<Entity>>,
    m_entity_destroy_requests: HashSet<EntityId>,
}

impl EntitySpawner {
    pub fn new() -> Self {
        Self {
            m_next_entity_id: 0,
            m_entities: Vec::new(),
            m_entity_spawn_requests: Vec::new(),
            m_entity_destroy_requests: HashSet::new(),
        }
    }

    pub fn entity_iter(&self) -> impl Iterator<Item = &Entity> {
        self.m_entities.iter().map(Box::as_ref)
    }

    pub fn entity_iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.m_entities.iter_mut().map(Box::as_mut)
    }

    pub fn spawn_entity(&mut self, mut entity: Box<Entity>) -> EntityId {
        let entity_id = self.m_next_entity_id;
        self.m_next_entity_id += 1;
        entity.set_id(entity_id);

        self.m_entity_spawn_requests.push(entity);

        entity_id
    }

    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        // Remove from spawns requests if present
        self.m_entity_spawn_requests
            .retain(|entity| entity.get_id() != entity_id);

        // Mark for destroy
        self.m_entity_destroy_requests.insert(entity_id);
    }

    pub fn resolve_requests(&mut self) {
        self.resolve_spawn_requests();
        self.resolve_destroy_requests();
    }

    fn resolve_spawn_requests(&mut self) {
        // Take memory, because an entity might make a new spawn request in enter_play()
        let mut spawn_requests = mem::take(&mut self.m_entity_spawn_requests);

        for entity in spawn_requests.drain(..) {
            self.m_entities.push(entity);
            self.m_entities.last_mut().unwrap().enter_play();
        }
    }

    fn resolve_destroy_requests(&mut self) {
        // Take memory, because an entity might make a new destroy request in exit_play()
        let destroy_requests = mem::take(&mut self.m_entity_destroy_requests);

        self.m_entities.retain_mut(|entity| {
            let should_destroy = destroy_requests.contains(&entity.get_id());
            if should_destroy {
                entity.exit_play();
            }

            !should_destroy
        });
    }
}
