use std::env;
use std::path::PathBuf;

use engine::core::app::{App, Sdl2Context};
use engine::core::assets::Assets;

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

fn get_root_path() -> PathBuf {
    let is_debug_build = cfg!(debug_assertions);
    let root_path;
    if is_debug_build {
        // Return the root directory of the Cargo.toml file
        root_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    } else {
        // Return the current directory of the executable
        root_path = env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Exe has no parent")
            .to_path_buf();
    }

    root_path
}

fn get_input_config_path() -> PathBuf {
    let is_debug_build = cfg!(debug_assertions);
    let root_path = get_root_path();
    let config_file_path;

    if is_debug_build {
        config_file_path = root_path.join("../engine/config").join("input_config.json");
    } else {
        config_file_path = root_path.join("engine/config").join("input_config.json");
    }

    println!("input_config_path: '{}'", config_file_path.display());

    config_file_path
}

fn get_assets_root_path() -> PathBuf {
    let is_debug_build = cfg!(debug_assertions);
    let root_path = get_root_path();
    let assets_root;

    if is_debug_build {
        assets_root = root_path.join("assets");
    } else {
        assets_root = root_path.join("game").join("assets");
    }

    println!("assets_root_path: '{}'", assets_root.display());

    assets_root
}
