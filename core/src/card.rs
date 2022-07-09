/// Represents a single card.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Debug)]
pub struct Card(pub u8);

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
