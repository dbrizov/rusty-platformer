use crate::entity::Entity;
use crate::math::Vec2;
use std::any::Any;

mod component_priority {
    pub const INPUT: i32 = -150;
    pub const TRANSFORM: i32 = -100;
    pub const DEFAULT: i32 = 0;
    pub const RENDER: i32 = 100;
}

// Component base
pub struct ComponentBase {
    m_entity_ptr: *mut Entity,
}

impl ComponentBase {
    pub fn new() -> Self {
        Self {
            m_entity_ptr: std::ptr::null_mut(),
        }
    }

    pub fn set_entity_ptr(&mut self, entity: *mut Entity) {
        self.m_entity_ptr = entity;
    }

    pub fn get_entity(&self) -> &Entity {
        assert!(!self.m_entity_ptr.is_null());
        unsafe { &*self.m_entity_ptr }
    }

    pub fn get_entity_mut(&self) -> &mut Entity {
        assert!(!self.m_entity_ptr.is_null());
        unsafe { &mut *self.m_entity_ptr }
    }
}

// Component
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for T
where
    T: Any,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Component: AsAny {
    fn priority(&self) -> i32 {
        component_priority::DEFAULT
    }

    fn base(&self) -> &ComponentBase;
    fn base_mut(&mut self) -> &mut ComponentBase;

    fn enter_play(&mut self) {}
    fn exit_play(&mut self) {}
    fn tick(&mut self, _delta_time: f32) {}
    fn physics_tick(&mut self, _fixed_delta_time: f32) {}
    fn render_tick(&mut self, _delta_time: f32) {}
}

// TransformComponent
pub struct TransformComponent {
    m_base: ComponentBase,
    m_position: Vec2,
    m_prev_position: Vec2,
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            m_base: ComponentBase::new(),
            m_position: Vec2::zero(),
            m_prev_position: Vec2::zero(),
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.m_position
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.m_prev_position = self.m_position;
        self.m_position = pos;
    }

    pub fn get_prev_position(&self) -> Vec2 {
        self.m_prev_position
    }
}

impl Component for TransformComponent {
    fn priority(&self) -> i32 {
        component_priority::TRANSFORM
    }
    
    fn base(&self) -> &ComponentBase {
        &self.m_base
    }
    
    fn base_mut(&mut self) -> &mut ComponentBase {
        &mut self.m_base
    }
}
