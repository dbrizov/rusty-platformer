use std::cell::RefCell;
use std::rc::Rc;

use engine::components::{
    Component, ComponentBase, ImageComponent, InputComponent, TransformComponent,
    component_priority,
};
use engine::core::assets::Assets;
use engine::core::input::Input;
use engine::entity::Entity;
use engine::math::Vec2;

pub fn create_player(assets: &mut Assets, input: Rc<RefCell<Input>>) -> Box<Entity> {
    let transform_comp = TransformComponent::new();
    let player_comp = PlayerComponent::new();
    let input_comp = InputComponent::new(input);

    let texture_path = assets
        .get_asset_path(&["images", "entities", "player", "idle", "00.png"])
        .unwrap();
    let texture_id = assets.load_texture(texture_path).unwrap();
    let mut image_comp = ImageComponent::new(texture_id);
    image_comp.set_scale(Vec2::one() * 2.0);

    let mut entity = Entity::new();
    entity.add_component(transform_comp);
    entity.add_component(player_comp);
    entity.add_component(input_comp);
    entity.add_component(image_comp);

    entity
}

#[derive(ComponentBase)]
pub struct PlayerComponent {
    m_entity: *mut Entity,
    m_speed: f32,
    m_movement_input: Vec2,
    m_horizontal_id: u32,
    m_vertical_id: u32,
}

impl PlayerComponent {
    pub fn new() -> Self {
        Self {
            m_entity: std::ptr::null_mut(),
            m_speed: 300.0,
            m_movement_input: Vec2::zero(),
            m_horizontal_id: 0,
            m_vertical_id: 0,
        }
    }

    fn set_movement_input_x(&mut self, axis: f32) {
        self.m_movement_input.x = axis;
    }

    fn set_movement_input_y(&mut self, axis: f32) {
        self.m_movement_input.y = axis;
    }
}

impl Component for PlayerComponent {
    fn get_priority(&self) -> i32 {
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
                (*this).set_movement_input_x(axis);
            });

            (*this).m_vertical_id = input_comp.bind_axis("vertical", move |axis| {
                (*this).set_movement_input_y(axis);
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
        self.m_movement_input = Vec2::zero();
    }

    fn tick(&mut self, _delta_time: f32) {
        let transform_comp = self
            .get_entity_mut()
            .get_component_mut::<TransformComponent>()
            .unwrap();

        let movement_input;
        if self.m_movement_input.len_sqr() > 1.0 {
            movement_input = self.m_movement_input.normalized();
        } else {
            movement_input = self.m_movement_input;
        }

        let pos_delta_x = Vec2::right() * movement_input.x;
        let pos_delta_y = Vec2::up() * movement_input.y;
        let pos_delta = (pos_delta_x + pos_delta_y) * self.m_speed * _delta_time;
        let new_pos = transform_comp.get_position() + pos_delta;
        transform_comp.set_position(new_pos);
    }
}
