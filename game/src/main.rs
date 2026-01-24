mod player;

use engine::{
    app::{App, Sdl2Context},
    assets::Assets,
};
use std::path::PathBuf;

fn main() {
    let mut sdl2 = Sdl2Context::new();
    let mut assets = Assets::new(get_assets_root(), sdl2.texture_creator());
    let mut app = App::new();
    app.spawn_entity(player::create_player(&mut assets));
    app.run(&mut sdl2, &mut assets);
}

fn get_assets_root() -> PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}
