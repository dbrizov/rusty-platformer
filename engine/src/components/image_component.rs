use crate::assets::TextureId;
use crate::components::{Component, ComponentBase, TransformComponent, component_priority};
use crate::entity::Entity;
use crate::math::Vec2;
use crate::render::{RenderQueue, RenderStruct};

#[derive(ComponentBase)]
pub struct ImageComponent {
    m_entity: *mut Entity,
    m_texture_id: TextureId,
    m_scale: Vec2,
}

impl Component for ImageComponent {
    fn get_priority(&self) -> i32 {
        component_priority::RENDER
    }

    fn render_tick(&mut self, _delta_time: f32, render_queue: &mut RenderQueue) {
        let transform = self
            .get_entity()
            .get_component::<TransformComponent>()
            .unwrap();

        let t_scale = transform.get_scale();
        render_queue.enqueue(RenderStruct::new(
            self.m_texture_id,
            transform.get_position(),
            transform.get_prev_position(),
            Vec2::from_xy(self.m_scale.x * t_scale.x, self.m_scale.y * t_scale.y),
        ));
    }
}

impl ImageComponent {
    pub fn new(texture_id: TextureId) -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
            m_texture_id: texture_id,
            m_scale: Vec2::one(),
        }
    }

    pub fn get_texture_id(&self) -> TextureId {
        self.m_texture_id
    }

    pub fn set_texture_id(&mut self, id: TextureId) {
        self.m_texture_id = id;
    }

    pub fn get_scale(&self) -> Vec2 {
        self.m_scale
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.m_scale = scale;
    }
}
