use crate::board::Board;
use crate::card::Card;
use crate::hand::Hand;

/// A player throughout the game.
#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Hand,
}

impl Player {
    pub fn new() -> Player {
        Self { hand: Hand::new() }
    }

    /// Returns true if any of this player's cards have any valid moves to make.
    pub fn can_play_a_card(&self, board: &Board) -> bool {
        self.hand.cards.iter().any(|card| board.can_play_card(card))
    }

    /// Gets the hand of this player, suitable for examination.
    pub fn hand(&self) -> &[Card] {
        &self.hand.cards
    }
}
