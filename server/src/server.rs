use std::collections::HashMap;
use actix::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use crate::room::Room;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Debug)]
pub struct GameServer {
    /// Map of all sessions and their send handles.
    pub sessions: HashMap<Uuid, Recipient<Message>>,
    /// Map of all available rooms.
    pub rooms: HashMap<Uuid, Room>,
}

impl GameServer {
    /// Removes the given session from all rooms.
    pub fn leave(&mut self, session_id: &Uuid) {
        for (_, room) in &mut self.rooms {
            room.leave(session_id);
        }
    }

    /// Creates a new room for a given player.
    pub fn create_room(&mut self, room_id: Uuid, session_id: Uuid) {
        let mut room = Room::new();
        room.join(session_id);
        self.rooms.insert(room_id, room);
    }

    /// Sends a message to all players in the room.
    pub fn send_message<T>(&self, room: &Uuid, msg: &T) where T: Serialize {
        if let Some(room) = self.rooms.get(room) {
            for session_id in &room.players {
                if let Some(session) = self.sessions.get(session_id) {
                    let message = Message(serde_json::to_string(&msg).unwrap());
                    session.do_send(message);
                }
            }
        }
    }

    /// Sends a message to the given user.
    pub fn send_message_to<T>(&self, receiver_id: &Uuid, msg: &T) where T: Serialize {
        if let Some(session) = self.sessions.get(receiver_id) {
            let message = Message(serde_json::to_string(&msg).unwrap());
            session.do_send(message);
        }
    }
}

impl Default for GameServer {
    fn default() -> Self {
        Self { rooms: HashMap::new(), sessions: HashMap::new() }
    }
}

/// Make actor from `GameServer`.
impl Actor for GameServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}
