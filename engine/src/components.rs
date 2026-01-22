use std::any::Any;

use crate::entity::Entity;
use crate::math::Vec2;
use engine_derive::ComponentBase;

pub mod component_priority {
    pub const INPUT: i32 = -150;
    pub const TRANSFORM: i32 = -100;
    pub const DEFAULT: i32 = 0;
    pub const RENDER: i32 = 100;
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

pub trait ComponentBase {
    unsafe fn set_entity_ptr(&mut self, entity: *mut Entity);
    fn get_entity(&self) -> &Entity;
    fn get_entity_mut(&self) -> &mut Entity;
}

#[allow(unused_variables)]
pub trait Component: ComponentBase + AsAny {
    fn priority(&self) -> i32 {
        component_priority::DEFAULT
    }

    fn enter_play(&mut self) {}
    fn exit_play(&mut self) {}
    fn tick(&mut self, delta_time: f32) {}
    fn physics_tick(&mut self, fixed_delta_time: f32) {}
    fn render_tick(&mut self, delta_time: f32) {}
}

// TransformComponent
#[derive(ComponentBase)]
pub struct TransformComponent {
    m_entity: *mut Entity,
    m_position: Vec2,
    m_prev_position: Vec2,
}

impl TransformComponent {
    pub fn new_box() -> Box<Self> {
        Box::new(Self {
            m_entity: std::ptr::null_mut(),
            m_position: Vec2::zero(),
            m_prev_position: Vec2::zero(),
        })
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
}
