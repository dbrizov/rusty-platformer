use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::components::{Component, ComponentBase, component_priority};
use crate::core::input::{
    INVALID_INPUT_EVENT_HANDLER_ID, Input, InputEvent, InputEventHandlerId, InputEventType,
};
use crate::entity::Entity;

pub type BindingId = i32;
pub const INVALID_BINDING_ID: BindingId = -1;

#[derive(ComponentBase)]
pub struct InputComponent {
    m_entity: *mut Entity,
    m_input: Rc<RefCell<Input>>,
    m_input_event_handler_id: InputEventHandlerId,
    m_next_binding_id: BindingId,
    m_axis_bindings: HashMap<String, Vec<(BindingId, Box<dyn Fn(f32)>)>>,
    m_action_pressed_bindings: HashMap<String, Vec<(BindingId, Box<dyn Fn()>)>>,
    m_action_released_bindings: HashMap<String, Vec<(BindingId, Box<dyn Fn()>)>>,
}

impl Component for InputComponent {
    fn get_priority(&self) -> i32 {
        component_priority::INPUT
    }

    fn enter_play(&mut self) {
        let this: *const InputComponent = self;
        self.m_input_event_handler_id =
            self.m_input
                .borrow_mut()
                .add_input_event_handler(move |event| unsafe {
                    (*this).on_input_event(event);
                });
    }

    fn exit_play(&mut self) {
        self.m_input
            .borrow_mut()
            .remove_input_event_handler(self.m_input_event_handler_id);
    }
}

impl InputComponent {
    pub fn new(input: Rc<RefCell<Input>>) -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
            m_input: input,
            m_input_event_handler_id: INVALID_INPUT_EVENT_HANDLER_ID,
            m_next_binding_id: 0,
            m_axis_bindings: HashMap::new(),
            m_action_pressed_bindings: HashMap::new(),
            m_action_released_bindings: HashMap::new(),
        }
    }

    pub fn bind_axis<T>(&mut self, axis_name: &str, func: T) -> BindingId
    where
        T: Fn(f32) + 'static,
    {
        let axis_name_copy = axis_name.to_string();
        let binding_id = self.m_next_binding_id;
        self.m_next_binding_id += 1;

        self.m_axis_bindings
            .entry(axis_name_copy)
            .or_insert_with(Vec::new)
            .push((binding_id, Box::new(func)));

        binding_id
    }

    pub fn unbind_axis(&mut self, axis_name: &str, binding_id: BindingId) {
        if let Some(bindings) = self.m_axis_bindings.get_mut(axis_name) {
            bindings.retain(|(id, _)| *id != binding_id);

            if bindings.is_empty() {
                self.m_axis_bindings.remove(axis_name);
            }
        }
    }

    pub fn bind_action<T>(
        &mut self,
        action_name: &str,
        event_type: InputEventType,
        func: T,
    ) -> BindingId
    where
        T: Fn() + 'static,
    {
        let action_name_copy = action_name.to_string();
        let binding_id = self.m_next_binding_id;
        self.m_next_binding_id += 1;

        match event_type {
            InputEventType::Pressed => {
                self.m_action_pressed_bindings
                    .entry(action_name_copy)
                    .or_insert_with(Vec::new)
                    .push((binding_id, Box::new(func)));
            }
            InputEventType::Released => {
                self.m_action_released_bindings
                    .entry(action_name_copy)
                    .or_insert_with(Vec::new)
                    .push((binding_id, Box::new(func)));
            }
            _ => {}
        }

        binding_id
    }

    pub fn unbind_action(&mut self, action_name: &str, binding_id: BindingId) {
        if let Some(bindings) = self.m_action_pressed_bindings.get_mut(action_name) {
            bindings.retain(|(id, _)| *id != binding_id);

            if bindings.is_empty() {
                self.m_action_pressed_bindings.remove(action_name);
            }
        }

        if let Some(bindings) = self.m_action_released_bindings.get_mut(action_name) {
            bindings.retain(|(id, _)| *id != binding_id);

            if bindings.is_empty() {
                self.m_action_released_bindings.remove(action_name);
            }
        }
    }

    pub fn clear_all_bindings(&mut self) {
        self.m_axis_bindings.clear();
        self.m_action_pressed_bindings.clear();
        self.m_action_released_bindings.clear();
    }

    fn on_input_event(&self, event: &InputEvent) {
        match event.ev_type {
            InputEventType::Axis => {
                if let Some(bindings) = self.m_axis_bindings.get(event.ev_name) {
                    for (_, func) in bindings {
                        func(event.axis_value);
                    }
                }
            }
            InputEventType::Pressed => {
                if let Some(bindings) = self.m_action_pressed_bindings.get(event.ev_name) {
                    for (_, func) in bindings {
                        func();
                    }
                }
            }
            InputEventType::Released => {
                if let Some(bindings) = self.m_action_released_bindings.get(event.ev_name) {
                    for (_, func) in bindings {
                        func();
                    }
                }
            }
        };
    }
}
