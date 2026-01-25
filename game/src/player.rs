use engine::assets::Assets;
use engine::components::{
    Component, ComponentBase, ImageComponent, InputComponent, TransformComponent,
    component_priority,
};
use engine::engine_derive::ComponentBase;
use engine::entity::{Entity, EntityRef};
use engine::input::{Input, InputEventType};
use engine::math::Vec2;

pub fn create_player(assets: &mut Assets, input: &'static mut Input) -> EntityRef {
    let entity = Entity::new();

    let transform_comp = TransformComponent::new();
    let player_comp = PlayerComponent::new();
    let mut input_comp = InputComponent::new(input);
    input_comp.bind_axis("horizontal", horizontal);
    input_comp.bind_axis("vertical", vertical);
    input_comp.bind_action("left", InputEventType::Pressed, left);
    input_comp.bind_action("left", InputEventType::Released, left);
    input_comp.bind_action("right", InputEventType::Pressed, right);
    input_comp.bind_action("right", InputEventType::Released, right);
    input_comp.bind_action("up", InputEventType::Pressed, up);
    input_comp.bind_action("up", InputEventType::Released, up);
    input_comp.bind_action("down", InputEventType::Pressed, down);
    input_comp.bind_action("down", InputEventType::Released, down);

    let image_path = assets.asset_path(&["images", "entities", "player", "idle", "00.png"]);
    let image_id = assets.load_texture(image_path).unwrap();
    let mut image_comp = ImageComponent::new(image_id);
    image_comp.set_scale(Vec2::one() * 2.0);

    {
        let mut entity_ref = entity.borrow_mut();
        entity_ref.add_component(transform_comp);
        entity_ref.add_component(player_comp);
        entity_ref.add_component(input_comp);
        entity_ref.add_component(image_comp);
    }

    entity
}

fn horizontal(axis: f32) {
    println!("{axis}");
}

fn vertical(axis: f32) {
    println!("{axis}");
}

fn left() {
    println!("left");
}

fn right() {
    println!("right");
}

fn up() {
    println!("up");
}

fn down() {
    println!("down");
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
}
