use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::components::{Component, ComponentBase, component_priority};
use crate::entity::Entity;
use crate::input::{Input, InputEvent, InputEventType, SubscriberId};

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
    fn get_priority(&self) -> i32 {
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

    pub fn unbind_action(&mut self, action_name: &str, handler_id: u32) {
        if let Some(handlers) = self.m_handlers_by_action_pressed.get_mut(action_name) {
            handlers.retain(|(id, _)| *id != handler_id);

            if handlers.is_empty() {
                self.m_handlers_by_action_pressed.remove(action_name);
            }
        }

        if let Some(handlers) = self.m_handlers_by_action_released.get_mut(action_name) {
            handlers.retain(|(id, _)| *id != handler_id);

            if handlers.is_empty() {
                self.m_handlers_by_action_released.remove(action_name);
            }
        }
    }

    pub fn clear_all_bindings(&mut self) {
        self.m_handlers_by_axis.clear();
        self.m_handlers_by_action_pressed.clear();
        self.m_handlers_by_action_released.clear();
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
