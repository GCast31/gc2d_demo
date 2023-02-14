
mod game;

use game::Game;
use gc2d::gc2d::*;

fn main() {
    Gc2d::new().run(Game::new()).unwrap();
}