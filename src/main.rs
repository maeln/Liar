// Liar game.

mod player;
use player::Player;

mod card;
use card::Card;

fn main() {
	let p1 = Player::new(Card::Thief, Card::Soothsayer);
    println!("{}", p1);
}
