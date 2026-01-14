use crate::components::{Component, TransformComponent};

pub struct Entity {
    m_components: Vec<Box<dyn Component>>,
    m_is_ticking: bool,
    m_is_in_play: bool,
}

impl Entity {
    pub fn new() -> Self {
        let mut entity: Entity = Self {
            m_components: Vec::new(),
            m_is_ticking: true,
            m_is_in_play: false,
        };
        // Always has TransformComponent
        entity.add_component(Box::new(TransformComponent::new()));
        entity
    }

    pub fn add_component<T>(&mut self, mut comp: Box<T>)
    where
        T: Component + 'static,
    {
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
