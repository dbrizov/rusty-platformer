use engine::assets::Assets;
use engine::components::{
    Component, ComponentBase, ImageComponent, TransformComponent, component_priority,
};
use engine::engine_derive::ComponentBase;
use engine::entity::{Entity, EntityRef};
use engine::math::Vec2;

pub fn create_player(assets: &mut Assets) -> EntityRef {
    let entity = Entity::new_rc();
    let transform_comp = TransformComponent::new_box();
    let player_comp = PlayerComponent::new_box();

    let image_path = assets.asset_path(&["images", "entities", "player", "idle", "00.png"]);
    let image_id = assets.load_texture(image_path).unwrap();
    let image_comp = ImageComponent::new_box(image_id);

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
    pub fn new_box() -> Box<Self> {
        Box::new(Self {
            m_entity: std::ptr::null_mut(),
        })
    }
}

impl Component for PlayerComponent {
    fn priority(&self) -> i32 {
        component_priority::DEFAULT
    }

    fn enter_play(&mut self) {
        if let Some(trans) = self
            .get_entity_mut()
            .get_component_mut::<TransformComponent>()
        {
            let id = self.get_entity().id() as f32;
            trans.set_position(Vec2::from_xy(id, id));
            println!("{:?}", trans.get_position())
        }
    }
}
