use engine::assets::Assets;
use engine::components::{
    Component, ComponentBase, ImageComponent, TransformComponent, component_priority,
};
use engine::engine_derive::ComponentBase;
use engine::entity::{Entity, EntityRef};
use engine::math::Vec2;

pub fn create_player(assets: &mut Assets) -> EntityRef {
    let entity = Entity::new_rc();
    let transform_comp = TransformComponent::new();
    let player_comp = PlayerComponent::new();

    let image_path = assets.asset_path(&["images", "entities", "player", "idle", "00.png"]);
    let image_id = assets.load_texture(image_path).unwrap();
    let image_comp = ImageComponent::new_scaled(image_id, Vec2::one() * 2.0);

    {
        let mut entity_ref = entity.borrow_mut();
        entity_ref.add_component(transform_comp);
        entity_ref.add_component(player_comp);
        entity_ref.add_component(image_comp);
    }

    entity
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
