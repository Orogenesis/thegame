use actix::prelude::*;
use uuid::Uuid;
use crate::server;

#[derive(Message)]
#[rtype(result = "Result<Uuid, &'static str>")]
pub struct CreateRoom(pub Uuid);

impl Handler<CreateRoom> for server::GameServer {
    type Result = Result<Uuid, &'static str>;

    /// Handles `CreateRoom` message.
    fn handle(&mut self, msg: CreateRoom, _: &mut Context<Self>) -> Self::Result {
        let room_id = Uuid::new_v4();
        self.create_room(room_id, msg.0);
        Ok(room_id)
    }
}
