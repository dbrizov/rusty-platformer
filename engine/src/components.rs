use crate::assets::TextureId;
use crate::entity::Entity;
use crate::math::Vec2;
use crate::render::RenderQueue;
use crate::render::RenderStruct;
use engine_derive::ComponentBase;
use std::any::Any;

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
pub trait Component: ComponentBase + AsAny + 'static {
    fn into_box(self) -> Box<dyn Component>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn priority(&self) -> i32 {
        component_priority::DEFAULT
    }

    fn enter_play(&mut self) {}
    fn exit_play(&mut self) {}
    fn tick(&mut self, _delta_time: f32) {}
    fn physics_tick(&mut self, _fixed_delta_time: f32) {}
    fn render_tick(&mut self, _delta_time: f32, render_queue: &mut RenderQueue) {}
}

// Transform Component
#[derive(ComponentBase)]
pub struct TransformComponent {
    m_entity: *mut Entity,
    m_position: Vec2,
    m_prev_position: Vec2,
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
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
}

// Image Component
#[derive(ComponentBase)]
pub struct ImageComponent {
    m_entity: *mut Entity,
    m_texture_id: TextureId,
    m_scale: Vec2,
}

impl ImageComponent {
    pub fn new(texture_id: TextureId) -> Self {
        ImageComponent::new_scaled(texture_id, Vec2::one())
    }

    pub fn new_scaled(texture_id: TextureId, scale: Vec2) -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
            m_texture_id: texture_id,
            m_scale: scale,
        }
    }

    pub fn get_texture_id(&self) -> TextureId {
        self.m_texture_id
    }

    pub fn set_texture_id(&mut self, id: TextureId) {
        self.m_texture_id = id;
    }
}

impl Component for ImageComponent {
    fn priority(&self) -> i32 {
        component_priority::RENDER
    }

    fn render_tick(&mut self, _delta_time: f32, render_queue: &mut RenderQueue) {
        let transform = self
            .get_entity()
            .get_component::<TransformComponent>()
            .unwrap();

        render_queue.enqueue(RenderStruct::new(
            self.m_texture_id,
            transform.get_position(),
            transform.get_prev_position(),
            self.m_scale,
        ));
    }
}
