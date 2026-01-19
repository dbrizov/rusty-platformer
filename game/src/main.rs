mod player;

use engine::app::App;

fn main() {
    let mut app = App::init();

    app.spawn_entity(player::create_player());

    app.run();
}
