// The game code.

use rand;
use rand::Rng;

use player::Player;
use card::Card;

pub enum GameMessage {
	PlayerDid(who: Player, what: GameAction),	
	
	PlayerSaw(who: Player, with: Card),
	PlayerStole(who: Player, by: Player),
	PlayerKilled(who: Player, by: Player),
	PlayerAlreadyDead,
}

pub enum GameAction {
	CardAction(card: Card, target: Player),
	TakeGold,
	Kill(Player),
	Pass,
}

pub trait GameIO {
	/** Send a message to a player */
	fn say(p: &Player, msg: GameMessage);
	
	/** Broadcast a message to players */
	fn broadcast(ps: &Vec<Player>, msg: GameMessage);	
		
	/** Ask a player the action he will take for this round. */
    fn ask_player_action(p: &Player) -> GameAction;
    
    /** Take a response action to another player action. 
    For the moment, can only be called when one player is the target of a assassination and can say he is a doctor. */
    fn ask_response_action(p: &Player) -> Option<GameAction>;
    
    /** Ask all the player in the game if they want to call bluff in response to another player action. */
    fn ask_all_bluff(ps: &Vec<Player>) -> bool;
}

#[derive(Debug)]
pub struct Game {
	players: Vec<Player>,
	deck: Vec<Card>,
	discard: Vec<Card>,
	current_player: usize,
	round_number: usize,
}

impl Game {
	pub fn new(nb_player: usize) -> Game {
		let mut deck: Vec<Card> = Game::new_deck(nb_player);
		let mut players: Vec<Player> = Vec::with_capacity(nb_player);
		for _ in 0..nb_player {
			players.push(Player::new(deck.pop().unwrap(), deck.pop().unwrap()));
		}
		Game {players: players, deck: deck, discard: Vec::new(), current_player: 0, round_number: 0}
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
	
	pub fn do_round<T: GameIO>(&mut self, io: T) {
		// first we ask the player the action he want to take.
		let action = io.ask_player_action(self.players[self.current_player]);
		
		match action {
			CardAction(card: Card, target: Player) => {self.handle_card_action(card, target, io);},
			TakeGold => {self.players[self.current_player].money += 1;},
			Kill(target: Player) => {self.handle_kill_action(target);},
			Pass => (),
		}
		
		// Finally, we increment the current player to make sure the next player will play next time we call do_round.
		self.current_player += 1;
	}
	
	fn handle_card_action<T: GameIO>(&self, card: Card, target: Player, io: T) {
		
	}
	
	fn handle_kill_action(&self, target: Player) {
		
	}
}

