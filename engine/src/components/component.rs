use std::any::Any;

use crate::core::render::RenderQueue;
use crate::entity::Entity;

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
    unsafe fn set_entity(&mut self, entity: *mut Entity);
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

    fn get_priority(&self) -> i32 {
        component_priority::DEFAULT
    }

    fn enter_play(&mut self) {}
    fn exit_play(&mut self) {}
    fn tick(&mut self, _delta_time: f32) {}
    fn physics_tick(&mut self, _fixed_delta_time: f32) {}
    fn render_tick(&mut self, _delta_time: f32, render_queue: &mut RenderQueue) {}
}
