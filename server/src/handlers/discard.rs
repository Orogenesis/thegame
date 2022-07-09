use actix::prelude::*;
use uuid::Uuid;
use crate::messages::MessageToClient::DiscardOne;
use crate::server;

#[derive(Message)]
#[rtype(result = "Result<(), &'static str>")]
pub struct DiscardCard {
    pub card: u8,
    pub position: usize,
    pub room_id: Uuid,
    pub session: Uuid,
}

/// Discards a card. Returning `Ok` once a card has been discarded, or `Err` otherwise.
impl Handler<DiscardCard> for server::GameServer {
    type Result = Result<(), &'static str>;

    /// Handles `DiscardCard` message.
    fn handle(&mut self, msg: DiscardCard, _: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            if let Err(err) = room.game.discard_card(msg.position, &msg.card.into()) {
                return Err(err);
            }

            self.send_message(&msg.room_id, &DiscardOne(msg.card, msg.position));
        }

        Ok(())
    }
}
