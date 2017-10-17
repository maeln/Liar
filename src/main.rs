// Liar game.

extern crate rand;

mod player;
use player::Player;

mod card;
use card::Card;

mod game;
use game::Game;

fn main() {
	let g = Game::new(2);
    println!("{:?}", g);
}
