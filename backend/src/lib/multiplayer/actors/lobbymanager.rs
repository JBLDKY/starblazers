// This module defines the `LobbyManager` actor, which is responsible for managing
// player sessions and lobbies. It handles the registration of players, their
// connections and disconnections, and the organization of players into different
// lobbies. The `LobbyManager` also facilitates the broadcasting of messages to
// all players in a specific lobby and maintains the game state using ring buffers.

use crate::multiplayer::communication::common::{
    CreateLobbyRequest, GameState, JoinLobbyRequest, LeaveLobbyRequest,
};
use crate::multiplayer::communication::message::{
    ClientMessage, Connect, Disconnect, Join, Message,
};
use crate::multiplayer::ringbuffer::RingBuffer;
use actix::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct LobbyManager {
    sessions: HashMap<String, Recipient<Message>>,
    lobbies: HashMap<String, HashSet<String>>,
    ring: HashMap<String, RingBuffer<GameState, 5>>,
}

impl LobbyManager {
    pub fn new() -> LobbyManager {
        // default room

        LobbyManager {
            sessions: HashMap::new(),
            lobbies: HashMap::new(),
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

impl Default for LobbyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Make actor from `LobbyServer`
impl Actor for LobbyManager {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for LobbyManager {
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

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for LobbyManager {
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
impl Handler<ClientMessage> for LobbyManager {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.lobby, msg.msg.as_str(), msg.id.to_string());
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for LobbyManager {
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

impl Handler<GameState> for LobbyManager {
    type Result = ();

    fn handle(&mut self, state: GameState, _: &mut Self::Context) {
        let player_ring = self
            .ring
            .get_mut(state.player_id())
            .expect("Could not access player ring");

        player_ring.push(state.clone());

        // TODO: this must now send the gamestate to the players in the appopriate lobby
        //
        // self.send_message(
        //     "main",
        //     &serde_json::to_string(&state.clone()).expect("couldnt parse gamestate to string"),
        //     state.into_player_id(),
        // );
    }
}

impl Handler<CreateLobbyRequest> for LobbyManager {
    type Result = ();
    fn handle(&mut self, req: CreateLobbyRequest, _: &mut Self::Context) {
        let players_in_lobby = HashSet::from([req.player_id.clone()]);
        self.lobbies.insert(req.lobby_name, players_in_lobby);

        log::info!("A player created a new lobby, current lobbies:\n");
        log::info!("{:#?}", &self.lobbies);
    }
}

impl Handler<JoinLobbyRequest> for LobbyManager {
    type Result = ();
    fn handle(&mut self, req: JoinLobbyRequest, _: &mut Self::Context) {
        // if let Some(lobby) = self.lobbies.get_mut(&req.lobby_name) {
        //     lobby.insert(req.player_id.clone());
        // };

        self.lobbies
            .entry(req.lobby_name.clone())
            .or_default()
            .insert(req.player_id.clone());

        log::info!(
            "Player `{}` joined lobby `{}`, current lobbies:\n",
            req.player_id,
            req.lobby_name
        );
        log::info!("{:#?}", &self.lobbies);
    }
}

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct ListLobbies;

impl Handler<ListLobbies> for LobbyManager {
    type Result = MessageResult<ListLobbies>;

    fn handle(&mut self, _: ListLobbies, _: &mut Self::Context) -> Self::Result {
        let lobbies = self.lobbies.keys().cloned().collect();
        MessageResult(lobbies)
    }
}

impl Handler<LeaveLobbyRequest> for LobbyManager {
    type Result = ();
    fn handle(&mut self, req: LeaveLobbyRequest, _: &mut Self::Context) {
        // Remove the player from the lobby
        self.lobbies
            .entry(req.lobby_name.clone())
            .or_default()
            .remove(&req.player_id.clone());

        // If that was the last player in the lobby, unlist the lobby
        if self
            .lobbies
            .entry(req.lobby_name.clone())
            .or_default()
            .is_empty()
        {
            self.lobbies.remove(&req.lobby_name.clone());
        }

        log::info!("A player has left the lobby, current lobbies:\n");
        log::info!("{:#?}", &self.lobbies);
    }
}
