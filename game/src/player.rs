use engine::components::{Component, ComponentBase, component_priority};
use engine::entity::Entity;

pub fn create_player() -> Box<Entity> {
    let mut player = Box::new(Entity::new());
    let player_comp = Box::new(PlayerComponent::new());
    player.add_component(player_comp);
    player
}

pub struct PlayerComponent {
    m_base: ComponentBase,
}

impl PlayerComponent {
    pub fn new() -> Self {
        Self {
            m_base: ComponentBase::new(),
        }
    }
}

impl Component for PlayerComponent {
    fn base(&self) -> &ComponentBase {
        &self.m_base
    }

    fn base_mut(&mut self) -> &mut ComponentBase {
        &mut self.m_base
    }

    fn priority(&self) -> i32 {
        component_priority::DEFAULT
    }

    fn enter_play(&mut self) {
        println!("enter_play");
        println!("player_id: {}", self.m_base.get_entity().id());
    }

    fn exit_play(&mut self) {
        println!("exit_play");
    }

    fn tick(&mut self, _delta_time: f32) {
        // println!("tick");
    }

    fn physics_tick(&mut self, _fixed_delta_time: f32) {
        // println!("physics_tick");
    }

    fn render_tick(&mut self, _delta_time: f32) {
        // println!("render_tick");
    }
}
