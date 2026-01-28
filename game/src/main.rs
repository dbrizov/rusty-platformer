use std::env;
use std::path::PathBuf;

use engine::{
    app::{App, Sdl2Context},
    assets::Assets,
};

mod player;

fn main() {
    let mut sdl2 = Sdl2Context::new();
    let mut assets = Assets::new(get_assets_root(), sdl2.get_texture_creator());
    let mut app = App::new();
    app.spawn_entity(player::create_player(&mut assets, sdl2.get_input()));
    app.run(&mut sdl2, &mut assets);
}

fn get_assets_root() -> PathBuf {
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

    let assets_root = root_path.join("assets");
    println!("assets_root_path: '{}'", assets_root.display());

    assets_root
}
