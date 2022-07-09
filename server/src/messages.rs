use actix::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A message from the client, transported over the websocket
/// connection.
#[derive(Message, Debug, Deserialize)]
#[serde(tag = "type", content = "params", rename_all = "snake_case")]
#[rtype(result = "()")]
pub enum MessageFromClient {
    /// A client joins the room with the given id.
    JoinRoom(Uuid),
    /// A client wants to leave its current game room.
    LeaveRoom(Uuid),
    /// A client creates a new room.
    CreateRoom,
    /// A client starts the game.
    StartGame(Uuid),
    /// A client discards a card.
    DiscardCard(DiscardCardMessage),
    /// A client ends the turn.
    EndTurn(Uuid),
}

#[derive(Debug, Deserialize)]
pub struct DiscardCardMessage {
    pub card: u8,
    pub position: usize,
    pub room_id: Uuid,
}

/// A message sent from the game to the client handler threads
/// which more directly interact with the players.
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum MessageToClient {
    /// When cards are dealt.
    DealtHand(Vec<u8>),
    /// When someone has discarded their card.
    DiscardOne(u8, usize),
    /// When someone have ended their turn
    /// and the next player has been instructed to take theirs.
    EndTurn(usize),
}
