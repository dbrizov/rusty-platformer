use engine::components::{Component, ComponentBase, component_priority};
use engine::engine_derive::ComponentBase;
use engine::entity::Entity;

pub fn create_player() -> Box<Entity> {
    let mut player = Box::new(Entity::new());
    let player_comp = Box::new(PlayerComponent::new());
    player.add_component(player_comp);
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
