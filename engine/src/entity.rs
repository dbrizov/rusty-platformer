use crate::components::{Component, TransformComponent};
use std::collections::HashSet;
use std::mem;

pub type EntityId = u32;

pub struct Entity {
    m_id: EntityId,
    m_components: Vec<Box<dyn Component>>,
    m_is_in_play: bool,
    m_is_ticking: bool,
}

impl Entity {
    pub fn new() -> Self {
        let mut entity: Entity = Self {
            m_id: 0,
            m_components: Vec::new(),
            m_is_in_play: false,
            m_is_ticking: false,
        };
        // Always has TransformComponent
        entity.add_component(Box::new(TransformComponent::new()));
        entity
    }

    pub fn enter_play(&mut self) {
        self.m_is_in_play = true;
        for comp in &mut self.m_components {
            comp.enter_play();
        }
    }

    pub fn exit_play(&mut self) {
        self.m_is_in_play = false;
        for comp in &mut self.m_components {
            comp.exit_play();
        }
    }

    pub fn tick(&mut self, delta_time: f32) {
        for comp in &mut self.m_components {
            comp.tick(delta_time);
        }
    }

    pub fn physics_tick(&mut self, fixed_delta_time: f32) {
        for comp in &mut self.m_components {
            comp.physics_tick(fixed_delta_time);
        }
    }

    pub fn render_tick(&mut self, delta_time: f32) {
        for comp in &mut self.m_components {
            comp.render_tick(delta_time);
        }
    }

    pub fn id(&self) -> EntityId {
        self.m_id
    }

    pub fn set_id(&mut self, id: EntityId) {
        self.m_id = id;
    }

    pub fn is_in_play(&self) -> bool {
        self.m_is_in_play
    }

    pub fn is_ticking(&self) -> bool {
        self.m_is_ticking
    }

    pub fn set_is_ticking(&mut self, is_ticking: bool) {
        self.m_is_ticking = is_ticking;
    }

    pub fn get_transform(&mut self) -> &mut TransformComponent {
        let transform: &mut TransformComponent =
            self.get_component_mut::<TransformComponent>().unwrap();
        transform
    }

    pub fn add_component<T>(&mut self, mut comp: Box<T>)
    where
        T: Component + 'static,
    {
        if self.is_in_play() {
            comp.enter_play();
        }

        comp.base_mut().set_entity_ptr(self as *mut Entity);

        self.m_components.push(comp);
        self.m_components
            .sort_by_key(|c: &Box<dyn Component + 'static>| c.priority());
    }

    pub fn get_component<T>(&self) -> Option<&T>
    where
        T: Component + 'static,
    {
        self.m_components
            .iter()
            .find_map(|c: &Box<dyn Component>| c.as_any().downcast_ref::<T>())
    }

    pub fn get_component_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        self.m_components
            .iter_mut()
            .find_map(|c: &mut Box<dyn Component + 'static>| c.as_any_mut().downcast_mut::<T>())
    }
}

// Entity Spawner
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

    pub fn entity_iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.m_entities.iter_mut().map(|e| e.as_mut())
    }

    pub fn spawn(&mut self, entity: Box<Entity>) {
        self.m_entity_spawn_requests.push(entity);
    }

    pub fn destroy(&mut self, entity_id: EntityId) {
        self.m_entity_destroy_requests.insert(entity_id);
    }

    pub fn resolve(&mut self) {
        self.resolve_entity_spawn_requests();
        self.resolve_entity_destroy_requests();
    }

    fn resolve_entity_spawn_requests(&mut self) {
        for entity in &mut self.m_entity_spawn_requests {
            entity.set_id(self.m_next_entity_id);
            self.m_next_entity_id += 1;

            entity.enter_play();
        }

        self.m_entities.append(&mut self.m_entity_spawn_requests);
    }

    fn resolve_entity_destroy_requests(&mut self) {
        let destroy_requests = mem::take(&mut self.m_entity_destroy_requests);
        for entity in self.m_entities.iter_mut() {
            if destroy_requests.contains(&entity.id()) {
                entity.exit_play();
            }
        }

        self.m_entities
            .retain(|entity| !destroy_requests.contains(&entity.id()));
    }
}
