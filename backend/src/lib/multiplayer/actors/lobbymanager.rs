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
use crate::multiplayer::InvalidDataError;
use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub struct Lobbies {
    lobbies: HashMap<String, HashSet<Uuid>>,
    player_to_lobby: HashMap<Uuid, String>,
}

impl Lobbies {
    pub fn get(&self) -> &HashMap<String, HashSet<Uuid>> {
        &self.lobbies
    }

    pub fn new() -> Self {
        Self {
            lobbies: HashMap::new(),
            player_to_lobby: HashMap::new(),
        }
    }

    pub fn new_lobby(&mut self, lobby_name: String) -> Result<(), InvalidDataError> {
        if self.lobbies.contains_key(&lobby_name) {
            return Err(InvalidDataError::LobbyAlreadyExists);
        }

        self.lobbies.insert(lobby_name, HashSet::new());

        Ok(())
    }

    pub fn empty_lobbies(&self) -> Vec<String> {
        self.lobbies
            .iter()
            .filter_map(|(key, value)| {
                if value.is_empty() {
                    Some(key.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove_empty(&mut self) {
        for key in self.empty_lobbies() {
            log::info!("Removing empty lobby: {}", &key);
            self.lobbies.remove(&key);
        }
    }

    pub fn add_player_to(
        &mut self,
        player: Uuid,
        lobby_name: String,
    ) -> Result<(), InvalidDataError> {
        if let Some(players) = self.lobbies.get_mut(&lobby_name) {
            players.insert(player);
            Ok(())
        } else {
            Err(InvalidDataError::LobbyDoesNotExist(lobby_name))
        }
    }

    pub fn remove_lobby(&mut self, lobby_name: &str) -> Option<HashSet<Uuid>> {
        self.lobbies.remove(lobby_name)
    }

    pub fn remove_player_from_lobby(
        &mut self,
        player: Uuid,
        lobby_name: String,
    ) -> Result<(), InvalidDataError> {
        self.lobbies
            .get_mut(&lobby_name)
            .ok_or_else(|| InvalidDataError::LobbyDoesNotExist(lobby_name.clone()))
            .and_then(|players| {
                if players.remove(&player) {
                    Ok(())
                } else {
                    Err(InvalidDataError::PlayerIsNotInLobby(lobby_name))
                }
            })
    }
}

impl fmt::Display for Lobbies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lobbies: {:?}\nPlayer-To-Lobby: {:?}",
            self.lobbies, self.player_to_lobby
        )
    }
}

#[derive(Debug)]
pub struct LobbyManager {
    sessions: HashMap<String, Recipient<Message>>,
    lobbies: Lobbies,
    ring: HashMap<String, RingBuffer<GameState, 5>>,
}

impl LobbyManager {
    pub fn new() -> LobbyManager {
        // default room

        LobbyManager {
            sessions: HashMap::new(),
            lobbies: Lobbies::new(),
            ring: HashMap::new(),
        }
    }

    // /// Send message to all users in the room
    // fn send_message(&self, room: &str, message: &str, skip_id: Uuid) {
    //     if let Some(sessions) = self.lobbies.lobbies.get(room) {
    //         for id in sessions {
    //             if *id != skip_id {
    //                 if let Some(addr) = self.sessions.get(id) {
    //                     addr.do_send(Message(message.to_owned()));
    //                 }
    //             }
    //         }
    //     }
    // }
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
        log::info!("Disconnect msg received: {:?}", msg);
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for LobbyManager {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        // self.send_message(&msg.lobby, msg.msg.as_str(), msg.id.to_string());
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for LobbyManager {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;

        // remove session from all lobbies
        // for (n, sessions) in &mut self.lobbies.0 {
        //     if sessions.remove(&id.to_string()) {
        //         lobbies.push(n.to_owned());
        //     }
        // }
        // send message to other users
        // for room in lobbies {
        //     self.send_message(&room, "Someone disconnected", id.to_string());
        // }

        // self.lobbies
        //     .get_lobbies()
        //     .entry(name.clone())
        //     .or_default()
        //     .insert(id.to_string());

        // self.send_message(&name, "Someone connected", id.to_string());
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
        let player_id = Uuid::parse_str(&req.player_id).expect("Not a valid uuid");

        let _ = self.lobbies.new_lobby(req.lobby_name.clone());
        let _ = self.lobbies.add_player_to(player_id, req.lobby_name);

        log::info!("A player created a new lobby, current lobbies:\n");
        log::info!("{:#?}", &self.lobbies.get());
    }
}

impl Handler<JoinLobbyRequest> for LobbyManager {
    type Result = ();
    fn handle(&mut self, req: JoinLobbyRequest, ctx: &mut Self::Context) {
        log::info!("join lobby request: {:#?}", req);
        log::info!(
            "Player `{}` joined lobby `{}`, current lobbies:\n",
            req.player_id,
            req.lobby_name
        );

        let player_id = Uuid::parse_str(&req.player_id).expect("Not a valid uuid");
        let _ = self.lobbies.add_player_to(player_id, req.lobby_name);

        log::info!("{:#?}", &self.lobbies);
    }
}

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct ListLobbies;

impl Handler<ListLobbies> for LobbyManager {
    type Result = MessageResult<ListLobbies>;

    fn handle(&mut self, _: ListLobbies, _: &mut Self::Context) -> Self::Result {
        self.lobbies.remove_empty();
        MessageResult(self.lobbies.get().keys().cloned().collect())
    }
}

impl Handler<LeaveLobbyRequest> for LobbyManager {
    type Result = ();
    fn handle(&mut self, req: LeaveLobbyRequest, _: &mut Self::Context) {
        log::info!("Leave lobby request: {:#?}", req);

        // Remove the player from the lobby
        let player_id = Uuid::parse_str(&req.player_id).expect("Not a valid Uuid");

        match self
            .lobbies
            .remove_player_from_lobby(player_id, req.lobby_name)
        {
            Ok(_) => log::info!("A player has left the lobby, current lobbies:"),
            Err(e) => log::error!("Could not remove player from lobby: {}", e),
        };

        // Cleanup empty lobbies
        self.lobbies.remove_empty();

        log::info!("{:#?}", &self.lobbies.get());
    }
}

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct PlayersInLobby {
    pub lobby_name: String,
}

impl Handler<PlayersInLobby> for LobbyManager {
    type Result = MessageResult<PlayersInLobby>;

    fn handle(&mut self, req: PlayersInLobby, _: &mut Self::Context) -> Self::Result {
        let players: Vec<String> = self
            .lobbies
            .get()
            .get(&req.lobby_name)
            .unwrap_or(&HashSet::new())
            .iter()
            .map(|s| s.to_string().to_owned())
            .collect();

        MessageResult(players)
    }
}
