// Player structure and methods.

use std::fmt;

use card::Card;

#[derive(Debug)]
pub struct Player {
	first_card: Option<Card>,
	second_card: Option<Card>,
	money: usize,
}

impl Player {
	pub fn new(card1: Card, card2: Card) -> Player {
		Player {first_card: Some(card1), second_card: Some(card2), money: 0}
	}
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut s = String::from("Player(1c:");
    	match self.first_card {
    		Some(ref n) => {s.push_str(&format!("{:?}, 2c:", n));},
    		None => {s.push_str("∅, 2c:");},
    	}
    	match self.second_card {
    		Some(ref n) => {s.push_str(&format!("{:?}, ", n));},
    		None => {s.push_str("∅, ");},
    	}
    	s.push_str(&format!("m: {})", self.money));
        write!(f, "{}", s)
    }
}

