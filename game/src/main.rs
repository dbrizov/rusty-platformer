use engine::{core::app::App, math::Vec2};

mod player;

const TARGET_FPS: u32 = 60;
const VSYNC_ENABLED: bool = true;
const WINDOW_TITLE: &str = "Rusty Platformer";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const PHYSICS_TICKS_PER_SECOND: u32 = 60;
const PHYSICS_INTERPOLATION: bool = true;

fn main() {
    let mut app = App::new(
        TARGET_FPS,
        VSYNC_ENABLED,
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        PHYSICS_TICKS_PER_SECOND,
        PHYSICS_INTERPOLATION,
    );

    let p0 = player::create_player(&mut app, Vec2::from_xy(50.0, 50.0));
    app.get_entity_spawner().spawn_entity(p0);

    app.run();
}
