use std::ops::AddAssign;
use crate::card::Card;

/// A `Hand` is zero or more cards that represents
/// the cards a person is holding.
#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    /// Create an empty hand.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one `Card` to the `Hand`.
    pub fn push_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Returns the number of cards.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns `true` if the hand is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Play the card at `card_idx`. This removes the card from the hand and
    /// returns it.
    pub fn play(&mut self, card_idx: usize) -> Card {
        assert!(card_idx < self.len());
        self.cards.remove(card_idx)
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self { cards: Vec::with_capacity(8) }
    }
}

impl AddAssign<Card> for Hand {
    fn add_assign(&mut self, rhs: Card) {
        self.push_card(rhs);
    }
}
