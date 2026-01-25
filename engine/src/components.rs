use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::assets::TextureId;
use crate::entity::Entity;
use crate::input::Input;
use crate::input::InputEvent;
use crate::input::InputEventType;
use crate::input::SubscriberId;
use crate::math::Vec2;
use crate::render::RenderQueue;
use crate::render::RenderStruct;
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
    m_scale: Vec2,
}

impl Component for TransformComponent {
    fn priority(&self) -> i32 {
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

// Image Component
#[derive(ComponentBase)]
pub struct ImageComponent {
    m_entity: *mut Entity,
    m_texture_id: TextureId,
    m_scale: Vec2,
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

// Input Component
#[derive(ComponentBase)]
pub struct InputComponent {
    m_entity: *mut Entity,
    m_input: Rc<RefCell<Input>>,
    m_input_subscriber_id: SubscriberId,
    m_next_handler_id: u32,
    m_handlers_by_axis: HashMap<String, Vec<(u32, Box<dyn Fn(f32)>)>>,
    m_handlers_by_action_pressed: HashMap<String, Vec<(u32, Box<dyn Fn()>)>>,
    m_handlers_by_action_released: HashMap<String, Vec<(u32, Box<dyn Fn()>)>>,
}

impl Component for InputComponent {
    fn priority(&self) -> i32 {
        component_priority::INPUT
    }

    fn enter_play(&mut self) {
        let this: *const InputComponent = self;
        self.m_input_subscriber_id =
            self.m_input
                .borrow_mut()
                .subscribe_to_input_event(move |event| unsafe {
                    (*this).on_input_event(event);
                });
    }

    fn exit_play(&mut self) {
        self.m_input
            .borrow_mut()
            .unsubscribe_from_input_event(self.m_input_subscriber_id);
    }
}

impl InputComponent {
    pub fn new(input: Rc<RefCell<Input>>) -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
            m_input: input,
            m_input_subscriber_id: 0,
            m_next_handler_id: 0,
            m_handlers_by_axis: HashMap::new(),
            m_handlers_by_action_pressed: HashMap::new(),
            m_handlers_by_action_released: HashMap::new(),
        }
    }

    pub fn bind_axis<T>(&mut self, axis_name: &str, handler: T) -> u32
    where
        T: Fn(f32) + 'static,
    {
        let axis_name_copy = axis_name.to_string();
        let handler_id = self.m_next_handler_id;
        self.m_next_handler_id += 1;

        self.m_handlers_by_axis
            .entry(axis_name_copy)
            .or_insert_with(Vec::new)
            .push((handler_id, Box::new(handler)));

        handler_id
    }

    pub fn unbind_axis(&mut self, axis_name: &str, handler_id: u32) {
        if let Some(handlers) = self.m_handlers_by_axis.get_mut(axis_name) {
            handlers.retain(|(id, _)| *id != handler_id);

            if handlers.is_empty() {
                self.m_handlers_by_axis.remove(axis_name);
            }
        }
    }

    pub fn unbind_all_axis(&mut self, axis_name: &str) {
        self.m_handlers_by_axis.remove(axis_name);
    }

    pub fn bind_action<T>(
        &mut self,
        action_name: &str,
        event_type: InputEventType,
        handler: T,
    ) -> u32
    where
        T: Fn() + 'static,
    {
        let action_name_copy = action_name.to_string();
        let handler_id = self.m_next_handler_id;
        self.m_next_handler_id += 1;

        match event_type {
            InputEventType::Pressed => {
                self.m_handlers_by_action_pressed
                    .entry(action_name_copy)
                    .or_insert_with(Vec::new)
                    .push((handler_id, Box::new(handler)));
            }
            InputEventType::Released => {
                self.m_handlers_by_action_released
                    .entry(action_name_copy)
                    .or_insert_with(Vec::new)
                    .push((handler_id, Box::new(handler)));
            }
            _ => {}
        }

        handler_id
    }

    pub fn unbind_action(
        &mut self,
        action_name: &str,
        event_type: InputEventType,
        handler_id: u32,
    ) {
        match event_type {
            InputEventType::Pressed => {
                if let Some(handlers) = self.m_handlers_by_action_pressed.get_mut(action_name) {
                    handlers.retain(|(id, _)| *id != handler_id);
                }
            }
            InputEventType::Released => {
                if let Some(handlers) = self.m_handlers_by_action_released.get_mut(action_name) {
                    handlers.retain(|(id, _)| *id != handler_id);
                }
            }
            _ => {}
        }
    }

    pub fn unbind_all_actions(&mut self, action_name: &str, event_type: InputEventType) {
        match event_type {
            InputEventType::Pressed => {
                self.m_handlers_by_action_pressed.remove(action_name);
            }
            InputEventType::Released => {
                self.m_handlers_by_action_released.remove(action_name);
            }
            _ => {}
        }
    }

    fn on_input_event(&self, event: &InputEvent) {
        match event.ev_type {
            InputEventType::Axis => {
                if let Some(handlers) = self.m_handlers_by_axis.get(event.ev_name) {
                    for (_, handler) in handlers {
                        handler(event.axis_value);
                    }
                }
            }
            InputEventType::Pressed => {
                if let Some(handlers) = self.m_handlers_by_action_pressed.get(event.ev_name) {
                    for (_, handler) in handlers {
                        handler();
                    }
                }
            }
            InputEventType::Released => {
                if let Some(handlers) = self.m_handlers_by_action_released.get(event.ev_name) {
                    for (_, handler) in handlers {
                        handler();
                    }
                }
            }
        };
    }
}
