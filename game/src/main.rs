mod player;

use engine::app::App;
use std::path::PathBuf;

fn main() {
    let mut app = App::init();
    app.set_assets_root(get_assets_root());

    app.spawn_entity(player::create_player());

    app.run();
}

fn get_assets_root() -> PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}
