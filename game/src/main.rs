mod player;

use engine::app::{App, Sdl2Instance};
use std::path::PathBuf;

fn main() {
    let mut sdl2 = Sdl2Instance::new();

    let mut app = App::new();
    app.set_assets_root(get_assets_root());

    app.spawn_entity(player::create_player());

    app.run(&mut sdl2);
}

fn get_assets_root() -> PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}
