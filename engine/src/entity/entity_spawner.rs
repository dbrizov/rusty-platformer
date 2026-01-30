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

    pub fn spawn(&mut self, entity: Box<Entity>) {
        self.m_entity_spawn_requests.push(entity);
    }

    pub fn destroy(&mut self, entity_id: EntityId) {
        self.m_entity_destroy_requests.insert(entity_id);
    }

    pub fn resolve_requests(&mut self) {
        self.resolve_spawn_requests();
        self.resolve_destroy_requests();
    }

    fn resolve_spawn_requests(&mut self) {
        // Take memory, because an entity might make a new spawn request in enter_play()
        let mut spawn_requests = mem::take(&mut self.m_entity_spawn_requests);

        for entity in &mut spawn_requests {
            entity.set_id(self.m_next_entity_id);
            entity.enter_play();

            self.m_next_entity_id += 1;
        }

        self.m_entities.append(&mut spawn_requests);
    }

    fn resolve_destroy_requests(&mut self) {
        // Take memory, because an entity might make a new destroy request in exit_play()
        let destroy_requests = mem::take(&mut self.m_entity_destroy_requests);

        for entity in self.m_entities.iter_mut() {
            let entity_id = entity.get_id();
            if destroy_requests.contains(&entity_id) {
                entity.exit_play();
            }
        }

        self.m_entities
            .retain(|entity| !destroy_requests.contains(&entity.get_id()));
    }
}
