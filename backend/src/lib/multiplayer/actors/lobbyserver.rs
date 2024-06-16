use std::collections::{HashMap, HashSet};

use actix::prelude::*;

use crate::multiplayer::communication::common::GameState;
use crate::multiplayer::communication::message::{
    ClientMessage, Connect, Disconnect, Join, Message,
};
use crate::multiplayer::ringbuffer::RingBuffer;

#[derive(Debug)]
pub struct LobbyServer {
    sessions: HashMap<String, Recipient<Message>>,
    lobbies: HashMap<String, HashSet<String>>,
    ring: HashMap<String, RingBuffer<GameState, 5>>,
}

impl LobbyServer {
    pub fn new() -> LobbyServer {
        // default room
        let mut lobbies = HashMap::new();
        lobbies.insert("main".to_owned(), HashSet::new());

        LobbyServer {
            sessions: HashMap::new(),
            lobbies,
            ring: HashMap::new(),
        }
    }

    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: String) {
        if let Some(sessions) = self.lobbies.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

impl Default for LobbyServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Make actor from `LobbyServer`
impl Actor for LobbyServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for LobbyServer {
    type Result = String;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room

        // register session with random id
        let id = msg.claims.uuid.clone();
        log::info!("Connected: {}", id);
        self.sessions.insert(id.clone(), msg.addr);

        let ring_buffer: RingBuffer<GameState, 5> = RingBuffer::new();
        self.ring.insert(id.clone(), ring_buffer);

        // auto join session to main room
        self.lobbies
            .entry("main".to_owned())
            .or_default()
            .insert(msg.claims.uuid.clone());

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for LobbyServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut lobbies: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all lobbies
            for (name, sessions) in &mut self.lobbies {
                if sessions.remove(&msg.id) {
                    lobbies.push(name.to_owned());
                }
            }
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for LobbyServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.lobby, msg.msg.as_str(), msg.id.to_string());
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for LobbyServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut lobbies = Vec::new();

        // remove session from all lobbies
        for (n, sessions) in &mut self.lobbies {
            if sessions.remove(&id.to_string()) {
                lobbies.push(n.to_owned());
            }
        }
        // send message to other users
        for room in lobbies {
            self.send_message(&room, "Someone disconnected", id.to_string());
        }

        self.lobbies
            .entry(name.clone())
            .or_default()
            .insert(id.to_string());

        self.send_message(&name, "Someone connected", id.to_string());
    }
}

impl Handler<GameState> for LobbyServer {
    type Result = ();

    fn handle(&mut self, state: GameState, _: &mut Self::Context) {
        let player_ring = self
            .ring
            .get_mut(state.player_id())
            .expect("Could not access player ring");

        player_ring.push(state.clone());

        self.send_message(
            "main",
            &serde_json::to_string(&state.clone()).expect("couldnt parse gamestate to string"),
            state.into_player_id(),
        );
    }
}
