// Card enum and other related functions.

#[derive(Debug)]
pub enum Card {
	Assassin, 	// can kill for 5 gold instead of 7
	Thief,		// can steal 2 gold from another player
	Doctor,		// is immune to the assassin
	Soothsayer,	// can view a random card of another player
	Banker,		// can get 3 gold instead of 1
}

