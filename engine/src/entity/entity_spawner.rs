use std::collections::{HashMap, HashSet};
use std::mem;

use crate::entity::{Entity, EntityId};

pub struct EntitySpawner {
    m_next_entity_id: EntityId,
    m_entities: Vec<Box<Entity>>,
    m_entity_index_by_id: HashMap<EntityId, usize>,
    m_entity_spawn_requests: Vec<Box<Entity>>,
    m_entity_destroy_requests: HashSet<EntityId>,
}

impl EntitySpawner {
    pub fn new() -> Self {
        Self {
            m_next_entity_id: 0,
            m_entities: Vec::new(),
            m_entity_index_by_id: HashMap::new(),
            m_entity_spawn_requests: Vec::new(),
            m_entity_destroy_requests: HashSet::new(),
        }
    }

    pub fn get_entity(&mut self, entity_id: EntityId) -> Option<&Entity> {
        let index = *self.m_entity_index_by_id.get(&entity_id)?;
        self.m_entities.get(index).map(Box::as_ref)
    }

    pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<&mut Entity> {
        let index = *self.m_entity_index_by_id.get(&entity_id)?;
        self.m_entities.get_mut(index).map(Box::as_mut)
    }

    pub fn entities_iter(&self) -> impl Iterator<Item = &Entity> {
        self.m_entities.iter().map(Box::as_ref)
    }

    pub fn entities_iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.m_entities.iter_mut().map(Box::as_mut)
    }

    pub fn ticking_entities_iter(&self) -> impl Iterator<Item = &Entity> {
        // TODO Optimize to not make O(n) filtering
        self.entities_iter().filter(|e| e.is_ticking())
    }

    pub fn ticking_entities_iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        // TODO Optimize to not make O(n) filtering
        self.entities_iter_mut().filter(|e| e.is_ticking())
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
        let spawn_requests = mem::take(&mut self.m_entity_spawn_requests);

        let start_index = self.m_entities.len();

        // Add them to the list of entities
        for entity in spawn_requests {
            let entity_id = entity.get_id();
            let index = self.m_entities.len();
            self.m_entity_index_by_id.insert(entity_id, index);
            self.m_entities.push(entity);
        }

        // Call enter_play() for the newly spawned entities
        for entity in &mut self.m_entities[start_index..] {
            entity.enter_play();
        }
    }

    fn resolve_destroy_requests(&mut self) {
        // Take memory, because an entity might make a new destroy request in exit_play()
        let destroy_requests = mem::take(&mut self.m_entity_destroy_requests);

        // Call exit_play() while entities are still present in the map
        for &entity_id in &destroy_requests {
            if let Some(index) = self.m_entity_index_by_id.get(&entity_id) {
                self.m_entities[*index].exit_play();
            }
        }

        // Erase the entities
        for entity_id in destroy_requests {
            let Some(index) = self.m_entity_index_by_id.remove(&entity_id) else {
                continue;
            };

            let last_index = self.m_entities.len() - 1;

            if index != last_index {
                // Move last entity into the hole.
                self.m_entities.swap(index, last_index);

                // Fix moved entity's index.
                let moved_entity_id = self.m_entities[index].get_id();
                self.m_entity_index_by_id.insert(moved_entity_id, index);
            }

            self.m_entities.pop();
        }
    }
}
