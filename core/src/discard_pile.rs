use crate::card::Card;
use crate::abs_diff_eq::AbsDiffEq;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

/// The `DiscardPile` represents a discard pile of zero or more cards.
#[derive(Debug, Clone)]
pub struct DiscardPile {
    cards: Vec<Card>,
    direction: Direction,
}

impl DiscardPile {
    /// Creates a `DiscardPile`, setting the top card to `start_with`.
    pub fn new(start_with: Card, direction: Direction) -> Self {
        Self { cards: vec![start_with], direction }
    }

    /// Returns new `DiscardPile` struct initialized a pile in ascending order.
    pub fn new_up() -> Self {
        Self::new(Card(1), Direction::Up)
    }

    /// Returns new `DiscardPile` struct initialized a pile in descending order.
    pub fn new_down() -> Self {
        Self::new(Card(100), Direction::Down)
    }

    /// Attempts to place the given card to the discard pile.
    pub fn discard_card(&mut self, card: &Card) -> Result<(), &'static str> {
        if !self.can_play_card(card) {
            return Err("Can't play that card");
        }

        self.cards.push(card.clone());
        Ok(())
    }

    /// Returns `true` if the given card can be played.
    pub fn can_play_card(&self, card: &Card) -> bool {
        self.is_card_corresponds_direction(card) || self.is_card_corresponds_rule10(card)
    }

    /// Peeks at the card on top of the pile, leaving it in place.
    pub fn peek_top_card(&self) -> &Card {
        self.cards.last().unwrap()
    }

    /// Returns the number of cards left in this discard pile.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns `Direction::Up` if the given card is ranked higher than
    /// the top card, `Direction::Down` otherwise.
    fn determine_card_direction(&self, card: &Card) -> Direction {
        if card.0 > self.peek_top_card().0 { Direction::Up } else { Direction::Down }
    }

    /// Returns `true` if the given card corresponds discard pile's direction.
    fn is_card_corresponds_direction(&self, card: &Card) -> bool {
        self.direction == self.determine_card_direction(card)
    }

    /// Returns `true` if the given card differs by 10 from the top card,
    /// according to the discard pile's direction.
    fn is_card_corresponds_rule10(&self, card: &Card) -> bool {
        card.0.abs_diff_eq(&self.peek_top_card().0, 10) && self.determine_card_direction(card) != self.direction
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::discard_pile::DiscardPile;

    #[test]
    fn test_place_card() {
        let expected_card = &Card(20);
        let mut discard_pile_up = DiscardPile::new_up();
        discard_pile_up.place_card(expected_card);
        assert_eq!(discard_pile_up.peek_top_card(), expected_card);
    }

    #[test]
    fn test_rule10() {
        let mut discard_pile_up = DiscardPile::new_up();
        discard_pile_up.place_card(&Card(20));
        assert_eq!(discard_pile_up.is_card_corresponds_rule10(&Card(10)), true);
        assert_eq!(discard_pile_up.is_card_corresponds_rule10(&Card(30)), false);
    }
}
