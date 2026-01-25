use std::cell::RefCell;
use std::rc::Rc;

use engine::assets::Assets;
use engine::components::{
    Component, ComponentBase, ImageComponent, InputComponent, TransformComponent,
    component_priority,
};
use engine::engine_derive::ComponentBase;
use engine::entity::{Entity, EntityRef};
use engine::input::{Input, InputEventType};
use engine::math::Vec2;

pub fn create_player(assets: &mut Assets, input: Rc<RefCell<Input>>) -> EntityRef {
    let entity = Entity::new();

    let transform_comp = TransformComponent::new();
    let player_comp = PlayerComponent::new();
    let mut input_comp = InputComponent::new(input);
    input_comp.bind_axis("horizontal", horizontal);
    input_comp.bind_axis("vertical", vertical);
    input_comp.bind_action("left", InputEventType::Pressed, left_pressed);
    input_comp.bind_action("left", InputEventType::Released, left_released);
    input_comp.bind_action("right", InputEventType::Pressed, right_pressed);
    input_comp.bind_action("right", InputEventType::Released, right_released);
    input_comp.bind_action("up", InputEventType::Pressed, up_pressed);
    input_comp.bind_action("up", InputEventType::Released, up_released);
    input_comp.bind_action("down", InputEventType::Pressed, down_pressed);
    input_comp.bind_action("down", InputEventType::Released, down_released);

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
    println!("horizontal: {axis}");
}

fn vertical(axis: f32) {
    println!("vertical: {axis}");
}

fn left_pressed() {
    println!("left_pressed");
}

fn left_released() {
    println!("left_released");
}

fn right_pressed() {
    println!("right_pressed");
}

fn right_released() {
    println!("right_released");
}

fn up_pressed() {
    println!("up_pressed");
}

fn up_released() {
    println!("up_released");
}

fn down_pressed() {
    println!("down_pressed");
}

fn down_released() {
    println!("down_released");
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
