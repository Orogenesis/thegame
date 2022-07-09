use linked_hash_set::LinkedHashSet;
use uuid::Uuid;
use the_game_core::game::Game;

#[derive(Debug, Clone)]
pub struct Room {
    pub players: LinkedHashSet<Uuid>,
    pub game: Game,
}

impl Room {
    /// Returns new `Room` struct initialized with default values.
    pub fn new() -> Self {
        Self { players: LinkedHashSet::new(), game: Game::new() }
    }

    /// Makes the given player join the room.
    pub fn join(&mut self, player: Uuid) {
        self.players.insert(player);
    }

    /// Removes the given player from the room.
    pub fn leave(&mut self, player: &Uuid) {
        self.players.remove(player);
    }

    /// Returns the number of players in this room.
    pub fn len(&self) -> usize {
        self.players.len()
    }

    /// Returns `true` if this room is empty.
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    /// Starts a new game of by shuffling the deck of cards and
    /// deals the players opening hand.
    pub fn start_game(&mut self) -> Result<(), &'static str> {
        for _ in 0..self.players.len() {
            self.game.add_player();
        }

        self.game.start()
    }

    /// Returns the owner of the room.
    pub fn get_owner(&self) -> Option<&Uuid> {
        self.players.front()
    }
}
