mod player;

use engine::game_loop::Game;

fn main() {
    let mut game = Game::init();

    let player = player::create_player();
    game.spawn_entity(player);

    game.run();
}
