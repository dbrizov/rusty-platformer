use crate::components::{Component, ComponentBase, component_priority};
use crate::entity::Entity;
use crate::math::Vec2;

#[derive(ComponentBase)]
pub struct TransformComponent {
    m_entity: *mut Entity,
    m_position: Vec2,
    m_prev_position: Vec2,
    m_scale: Vec2,
}

impl Component for TransformComponent {
    fn get_priority(&self) -> i32 {
        component_priority::TRANSFORM
    }
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
            m_position: Vec2::zero(),
            m_prev_position: Vec2::zero(),
            m_scale: Vec2::one(),
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

    pub fn get_scale(&self) -> Vec2 {
        self.m_scale
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.m_scale = scale;
    }
}
