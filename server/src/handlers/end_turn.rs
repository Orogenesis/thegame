use actix::prelude::*;
use uuid::Uuid;
use crate::messages::MessageToClient::DealtHand;
use crate::server;

#[derive(Message)]
#[rtype(result = "Result<(), &'static str>")]
pub struct EndTurn {
    pub room_id: Uuid,
    pub session: Uuid,
}

impl Handler<EndTurn> for server::GameServer {
    type Result = Result<(), &'static str>;

    /// Handles `EndTurn` message.
    fn handle(&mut self, msg: EndTurn, _: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            let dealt_cards = room.game.end_turn()?;
            let cards = dealt_cards.iter().map(|x| x.0).collect::<Vec<_>>();
            self.send_message_to(&msg.session, &DealtHand(cards));
        }

        Ok(())
    }
}
