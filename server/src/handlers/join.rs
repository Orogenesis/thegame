use actix::prelude::*;
use uuid::Uuid;
use crate::server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub room_id: Uuid,
    pub session: Uuid,
}

impl Handler<JoinRoom> for server::GameServer {
    type Result = ();

    /// Handles `JoinRoom` message.
    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) {
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            room.join(msg.session)
        }
    }
}
