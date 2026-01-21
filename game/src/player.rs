use std::cell::RefCell;
use std::rc::Rc;

use engine::components::{Component, ComponentBase, component_priority};
use engine::engine_derive::ComponentBase;
use engine::entity::{Entity, EntityRef};

pub fn create_player() -> EntityRef {
    let player = Rc::new(RefCell::new(Entity::new()));
    let player_comp = Box::new(PlayerComponent::new());
    player.borrow_mut().add_component(player_comp);
    player
}

#[derive(ComponentBase)]
pub struct PlayerComponent {
    m_entity: *mut Entity,
}

impl PlayerComponent {
    pub fn new() -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
        }
    }
}

impl Component for PlayerComponent {
    fn priority(&self) -> i32 {
        component_priority::DEFAULT
    }

    fn enter_play(&mut self) {
        println!("enter_play");
        println!("player_id: {}", self.get_entity().id());
    }

    fn exit_play(&mut self) {
        println!("exit_play");
        println!("player_id: {}", self.get_entity().id());
    }
}
