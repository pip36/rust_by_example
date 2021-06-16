mod game;

fn main() {
    let mut game = game::Game::new();

    match game.play(0) {
        _ => (),
    }
}
