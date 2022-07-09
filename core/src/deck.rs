use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::card::Card;
use crate::player::Player;

/// The `Deck` represents a deck of zero or more cards.
#[derive(Debug, Clone)]
pub struct Deck {
    /// A deck contains zero or more cards.
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a new deck of cards, shuffled.
    pub fn new_shuffled() -> Self {
        let mut deck = Self::new();
        deck.shuffle();
        deck
    }

    /// Returns the number of cards left in this deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Shuffles the deck.
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Draws a card from the deck, removing it from the deck.
    ///
    /// If the deck is empty, this will return `Err`.
    /// Otherwise, it will return the card at the top of the deck.
    pub fn draw(&mut self) -> Result<Card, &'static str> {
        if let Some(card) = self.cards.pop() {
            Ok(card)
        } else {
            Err("Deck is empty")
        }
    }

    /// Returns `true` if this deck is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Deals one or more card straight to the `Player`.
    pub fn deal_to_hand(&mut self, player: &mut Player, n: usize) -> Vec<Card> {
        let mut dealt_cards = Vec::with_capacity(n);
        for _ in 0..n {
            if let Ok(card) = self.draw() {
                player.hand += card;
                dealt_cards.push(card);
            } else {
                break;
            }
        }

        dealt_cards
    }

    /// Deal `n` cards to each player's hand.
    pub fn deal_each(&mut self, players: &mut Vec<Player>, n: usize) {
        if self.len() < players.len() * n {
            panic!("Deck has too few cards!");
        }

        for player in players.iter_mut() {
            self.deal_to_hand(player, n);
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self { cards: (2..99).map(|v| Card(v)).collect() }
    }
}
