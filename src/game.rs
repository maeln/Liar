// The game code.

use rand;
use rand::Rng;

use player::Player;
use card::Card;

#[derive(Debug)]
pub struct Game {
	players: Vec<Player>,
	deck: Vec<Card>,
	discard: Vec<Card>,
}

impl Game {
	pub fn new(nb_player: usize) -> Game {
		let mut deck: Vec<Card> = Game::new_deck(nb_player);
		let mut players: Vec<Player> = Vec::with_capacity(nb_player);
		for _ in 0..nb_player {
			players.push(Player::new(deck.pop().unwrap(), deck.pop().unwrap()));
		}
		Game {players: players, deck: deck, discard: Vec::new()}
	}
	
	fn new_deck(nb_player: usize) -> Vec<Card> {
		let mut deck: Vec<Card> = Vec::with_capacity(nb_player*2*5); // The deck as 2 card of each type per player.
		for i in 0..(nb_player*2*5) {
			match i%5 {
				0 => {deck.push(Card::Assassin);},
				1 => {deck.push(Card::Thief);},
				2 => {deck.push(Card::Doctor);},
				3 => {deck.push(Card::Soothsayer);},
				_ => {deck.push(Card::Banker);},
			}
		}
		rand::thread_rng().shuffle(&mut deck);
		deck
	}
}

