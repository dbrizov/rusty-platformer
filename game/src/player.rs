use std::cell::RefCell;
use std::rc::Rc;

use engine::assets::Assets;
use engine::components::{
    Component, ComponentBase, ImageComponent, InputComponent, TransformComponent,
    component_priority,
};
use engine::engine_derive::ComponentBase;
use engine::entity::{Entity, EntityRef};
use engine::input::Input;
use engine::math::Vec2;

pub fn create_player(assets: &mut Assets, input: Rc<RefCell<Input>>) -> EntityRef {
    let entity = Entity::new();

    let transform_comp = TransformComponent::new();
    let player_comp = PlayerComponent::new();
    let input_comp = InputComponent::new(input);

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

#[derive(ComponentBase)]
pub struct PlayerComponent {
    m_entity: *mut Entity,
    m_horizontal_id: u32,
    m_vertical_id: u32,
}

impl PlayerComponent {
    pub fn new() -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
            m_horizontal_id: 0,
            m_vertical_id: 0,
        }
    }

    fn horizontal(&mut self, axis: f32) {
        println!("horizontal: {axis}");
    }

    fn vertical(&mut self, axis: f32) {
        println!("vertical: {axis}");
    }
}

impl Component for PlayerComponent {
    fn priority(&self) -> i32 {
        component_priority::DEFAULT
    }

    fn enter_play(&mut self) {
        let this: *mut PlayerComponent = self;
        unsafe {
            let input_comp = (*this)
                .get_entity_mut()
                .get_component_mut::<InputComponent>()
                .unwrap();

            (*this).m_horizontal_id = input_comp.bind_axis("horizontal", move |axis| {
                (*this).horizontal(axis);
            });

            (*this).m_vertical_id = input_comp.bind_axis("vertical", move |axis| {
                (*this).vertical(axis);
            });
        }
    }

    fn exit_play(&mut self) {
        let input_comp = self
            .get_entity_mut()
            .get_component_mut::<InputComponent>()
            .unwrap();

        input_comp.unbind_axis("horizontal", self.m_horizontal_id);
        input_comp.unbind_axis("vertical", self.m_vertical_id);
    }
}
