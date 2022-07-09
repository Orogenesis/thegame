use crate::card::Card;
use crate::deck::Deck;
use crate::discard_pile::DiscardPile;

#[derive(Debug, Clone)]
pub struct Board {
    /// The deck starts at 98 cards.
    pub deck: Deck,
    /// The four discard piles.
    /// Each discard pile starts at 1 card for a total of 100.
    discard_piles: [DiscardPile; 4],
}

impl Board {
    /// Returns new `Board` struct initialized with default values.
    pub fn new() -> Board {
        let discard_piles: [DiscardPile; 4] = [
            DiscardPile::new_up(),
            DiscardPile::new_up(),
            DiscardPile::new_down(),
            DiscardPile::new_down(),
        ];

        Self { deck: Deck::new_shuffled(), discard_piles }
    }

    /// Returns true if the given card have any valid moves to make.
    pub fn can_play_card(&self, card: &Card) -> bool {
        self.discard_piles.iter().any(|discard_pile| discard_pile.can_play_card(card))
    }

    /// Adds a card to the given discard pile.
    pub fn discard_card(&mut self, index: usize, card: &Card) -> Result<(), &'static str> {
        let discard_pile = self.discard_piles.get_mut(index).unwrap();
        discard_pile.discard_card(card)
    }
}
