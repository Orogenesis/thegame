use actix::prelude::*;
use uuid::Uuid;
use crate::server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect(pub Uuid);

impl Handler<Disconnect> for server::GameServer {
    type Result = ();

    /// Handles `Disconnect` message.
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.0);
    }
}
