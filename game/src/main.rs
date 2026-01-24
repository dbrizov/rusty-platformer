mod player;

use engine::{
    app::{App, Sdl2Instance},
    assets::Assets,
};
use std::path::PathBuf;

fn main() {
    let mut sdl2 = Sdl2Instance::new();
    let mut assets = Assets::new(get_assets_root());

    let mut app = App::new();

    app.spawn_entity(player::create_player());

    app.run(&mut sdl2, &mut assets);
}

fn get_assets_root() -> PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}
