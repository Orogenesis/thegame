use actix::prelude::*;
use uuid::Uuid;
use crate::messages::MessageToClient::DealtHand;
use crate::server;

#[derive(Message)]
#[rtype(result = "Result<(), &'static str>")]
pub struct StartGame {
    pub room_id: Uuid,
    pub session: Uuid,
}

/// Starts a new game of by shuffling the deck of cards and
/// deals the players opening hand.
/// Only the room owner has the privilege to start the game.
impl Handler<StartGame> for server::GameServer {
    type Result = Result<(), &'static str>;

    /// Handles `StartGame` message.
    fn handle(&mut self, msg: StartGame, _: &mut Context<Self>) -> Self::Result {
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            if Some(&msg.session) == room.get_owner() {
                if let Err(err) = room.start_game() {
                    return Err(err);
                }

                // Sends each player an event containing a set of cards in their hand.
                for (id, player) in room.players.clone().iter().zip(room.game.clone().players()) {
                    let cards = player.hand().iter().map(|x| x.0).collect::<Vec<_>>();
                    self.send_message_to(id, &DealtHand(cards))
                }
            }
        }

        Ok(())
    }
}
