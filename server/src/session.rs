use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Serialize;
use uuid::Uuid;
use crate::handlers::prelude::*;
use crate::protocol;
use crate::messages::MessageFromClient;
use crate::server;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

/// An actor representing a websocket connection.
#[derive(Debug)]
pub struct Session {
    /// Client must send ping at least once per 30 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// Unique session id.
    id: Uuid,
    /// Game server.
    addr: Addr<server::GameServer>,
}

impl Session {
    /// Returns new `Session` struct initialized with default values.
    pub fn new(id: Uuid, addr: Addr<server::GameServer>) -> Self {
        Self { hb: Instant::now(), id, addr }
    }

    /// Sends ping to client every `HEARTBEAT_INTERVAL`.
    /// Also this method checks heartbeats from client and disconnects unresponsive clients.
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(Disconnect(act.id));
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }

    /// Sends the given message to the websockets actor and waits for response.
    fn send_message<M, T>(&mut self, request_id: protocol::Id, msg: M, ctx: &mut ws::WebsocketContext<Self>)
        where M: actix::Message<Result=Result<T, &'static str>> + Send + 'static,
              M::Result: Send,
              server::GameServer: Handler<M>,
              T: Serialize
    {
        self.addr
            .send(msg)
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(Ok(val)) => {
                        let msg = protocol::Response::success(request_id, &val);
                        ctx.text(serde_json::to_string(&msg).unwrap())
                    }
                    Ok(Err(err)) => {
                        let msg = protocol::Response::failure(request_id, err);
                        ctx.text(serde_json::to_string(&msg).unwrap())
                    }
                    _ => {}
                }

                fut::ready(())
            })
            .wait(ctx)
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    /// Called on actor start.
    /// We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        // We'll start heartbeat process on session start.
        self.hb(ctx);
        // Register self in game server.
        let recipient = ctx.address().recipient();
        self.addr.do_send(Connect { id: self.id, addr: recipient })
    }

    /// Called after an actor is in Actor::Stopping state.
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect(self.id));
        Running::Stop
    }
}

impl Handler<server::Message> for Session {
    type Result = ();

    /// Handles messages from game server, we simply send it to peer websocket.
    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// The main handler for all immediate socket and context related operations
/// on the connection.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now()
            }
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<protocol::Request>(&text) {
                    Ok(protocol::Request { id, payload }) => {
                        match payload {
                            MessageFromClient::CreateRoom => {
                                let msg = CreateRoom(self.id);
                                self.send_message(id, msg, ctx)
                            }
                            MessageFromClient::StartGame(room_id) => {
                                let msg = StartGame { room_id, session: self.id };
                                self.send_message(id, msg, ctx)
                            }
                            MessageFromClient::EndTurn(room_id) => {
                                let msg = EndTurn { room_id, session: self.id };
                                self.send_message(id, msg, ctx)
                            }
                            MessageFromClient::DiscardCard(message) => {
                                let msg = DiscardCard {
                                    card: message.card,
                                    position: message.position,
                                    room_id: message.room_id,
                                    session: self.id,
                                };

                                self.send_message(id, msg, ctx)
                            }
                            _ => println!("Unhandled event")
                        }
                    }
                    _ => println!("Failed to parse json")
                }
            }
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => ctx.stop(),
        }
    }
}
