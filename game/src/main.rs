use engine::core::app::App;

mod player;

const TARGET_FPS: u32 = 30;
const VSYNC_ENABLED: bool = true;
const WINDOW_TITLE: &str = "Rusty Platformer";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn main() {
    let mut app = App::new(
        TARGET_FPS,
        VSYNC_ENABLED,
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    let player_entity = player::create_player(&mut app);
    app.get_entity_spawner().spawn_entity(player_entity);

    app.run();
}
