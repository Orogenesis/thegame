use std::fmt::Error;
use crate::board::Board;
use crate::card::Card;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::player::Player;

/// A game status representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Created,
    Playing,
    GameOver(Outcome),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Loss,
}

/// The `Game` contains the entirety of the current state
/// of the game.
#[derive(Debug, Clone)]
pub struct Game {
    /// The amount of turns taken by the players over the course of the game.
    pub turn: usize,
    /// The player who currently has the turn.
    pub current_player_index: usize,
    /// The number of actions taken during the current turn.
    pub actions_taken: u8,
    /// The state of the game.
    pub status: GameStatus,
    /// The board that belongs to this game.
    board: Board,
    /// A vector that contains all players that participate in the game.
    /// Moving through the vector starting from index 0 is equivalent to moving clockwise
    /// around the table of players.
    pub players: Vec<Player>,
}

impl Game {
    /// Returns new `Game` struct initialized with default values.
    pub fn new() -> Game {
        Self {
            turn: 0,
            current_player_index: 0,
            actions_taken: 0,
            status: GameStatus::Created,
            board: Board::new(),
            players: Vec::with_capacity(5),
        }
    }

    /// Starts a new game of by shuffling the deck of cards and
    /// deals the players opening hand.
    pub fn start(&mut self) -> Result<(), &'static str> {
        if !self.is_created() {
            return Err("Can't start an already running game");
        }

        self.status = GameStatus::Playing;
        self.deal_hands();
        Ok(())
    }

    /// Adds a card to the given discard pile.
    pub fn discard_card(&mut self, index: usize, card: &Card) -> Result<(), &'static str> {
        if !self.is_playing() {
            return Err("Game is not running");
        }

        self.board.discard_card(index, card)?;
        self.actions_taken += 1;
        Ok(())
    }

    /// Sets the turn of this game to the next player.
    pub fn next_turn(&mut self) {
        if let Some(next_index) = self.next_player_index() {
            self.current_player_index = next_index;
            self.turn += 1
        }
    }

    /// Returns the index of the next player to play.
    pub fn next_player_index(&mut self) -> Option<usize> {
        let mut players = self.players.clone();
        players.rotate_right(self.current_player_index);
        players.iter().position(|x| !x.hand.is_empty())
    }

    /// Returns `true` if this `Game` is in play.
    pub fn is_playing(&self) -> bool {
        matches!(self.status, GameStatus::Playing)
    }

    /// Returns `true` if this `Game` has not started yet.
    pub fn is_created(&self) -> bool {
        matches!(self.status, GameStatus::Created)
    }

    /// Returns a reason for why this `Game` is over, if it is over. Otherwise it returns `None`.
    pub fn outcome(&self) -> Option<Outcome> {
        if let GameStatus::GameOver(outcome) = self.status {
            Some(outcome)
        } else {
            None
        }
    }

    /// Returns true if current player has a card can play,
    /// false if not (and the game is thus over).
    pub fn can_player_play_card(&self) -> bool {
        self.get_current_player().can_play_a_card(&self.board)
    }

    /// Rests the current player's turn
    /// and attempts to find the next player in line.
    pub fn end_turn(&mut self) -> Result<Vec<Card>, &'static str> {
        if self.actions_taken < self.get_remaining_actions() {
            return Err("An action is required before completing the turn");
        }

        let dealt_cards = self.deal_cards();
        self.actions_taken = 0;
        self.next_turn();
        Ok(dealt_cards)
    }

    /// Returns the current active player.
    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player_index]
    }

    /// Returns `2` if there are still cards left in the draw pile,
    /// or `1`, if the draw pile is empty.
    pub fn get_remaining_actions(&self) -> u8 {
        if self.board.deck.is_empty() { 1 } else { 2 }
    }

    /// Returns the players of this game in the order they are playing.
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    /// Add a player to the game.
    pub fn add_player(&mut self) {
        self.players.push(Player::new());
    }

    /// Deals a hand to all players.
    fn deal_hands(&mut self) {
        let size = self.max_hand_size();
        self.board.deck.deal_each(&mut self.players, size)
    }

    /// Returns `true` if all hands are empty.
    fn all_hands_empty(&self) -> bool {
        self.players.iter().all(|player| player.hand.is_empty())
    }

    /// Determines whether the game is won.
    fn is_won(&self) -> bool {
        self.board.deck.is_empty() && self.all_hands_empty()
    }

    /// Returns max hand size based on the number of in-game players.
    fn max_hand_size(&self) -> usize {
        match self.players.len() {
            1 => 8,
            2 => 7,
            _ => 6
        }
    }

    /// Draws two cards from the top of the deck and
    /// place them into the current player's hand.
    fn deal_cards(&mut self) -> Vec<Card> {
        self.board.deck.deal_to_hand(&mut self.players[self.current_player_index], 2)
    }
}
