use actix::prelude::*;
use uuid::Uuid;
use crate::server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: Uuid,
    pub addr: Recipient<server::Message>,
}

impl Handler<Connect> for server::GameServer {
    type Result = ();

    /// Handles `Connect` message.
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
    }
}
