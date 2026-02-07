use engine::core::app::{App, Sdl2Context};
use engine::core::assets::Assets;
use engine::core::path_utils::*;

mod player;

const TARGET_FPS: u32 = 30;
const VSYNC_ENABLED: bool = true;
const WINDOW_TITLE: &str = "Rusty Platformer";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn main() {
    // TODO Remove input config out of Sdl2Context
    let mut sdl2 = Sdl2Context::new(
        TARGET_FPS,
        VSYNC_ENABLED,
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        get_input_config_path(),
    );

    let mut assets = Assets::new(get_assets_root_path(), sdl2.get_texture_creator());
    let mut app = App::new();
    app.spawn_entity(player::create_player(&mut assets, sdl2.get_input()));
    app.run(&mut sdl2, &mut assets);
}
