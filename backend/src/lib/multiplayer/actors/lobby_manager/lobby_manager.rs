// This module defines the `LobbyManager` actor, which is responsible for managing
// player sessions and lobbies. It handles the registration of players, their
// connections and disconnections, and the organization of players into different
// lobbies. The `LobbyManager` also facilitates the broadcasting of messages to
// all players in a specific lobby and maintains the game state using ring buffers.

use crate::multiplayer::communication::common::{
    CreateLobbyRequest, GameState, JoinLobbyRequest, LeaveLobbyRequest,
};
use crate::multiplayer::communication::message::{
    ClientMessage, Connect, Disconnect, PlayersInLobby,
};
use crate::multiplayer::ringbuffer::RingBuffer;
use crate::multiplayer::ListLobbies;
use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::lobbies::Lobbies;

#[derive(Debug)]
pub struct LobbyManager {
    lobbies: Lobbies,
    ring: HashMap<String, RingBuffer<GameState, 5>>,
}

impl LobbyManager {
    pub fn new() -> LobbyManager {
        // default room

        LobbyManager {
            lobbies: Lobbies::new(),
            ring: HashMap::new(),
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
        // register session with random id
        let id = msg.claims.uuid.clone();
        log::info!("Connected: {}", id);

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

    fn handle(&mut self, _msg: ClientMessage, _: &mut Context<Self>) {
        todo!()
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
    fn handle(&mut self, req: JoinLobbyRequest, _ctx: &mut Self::Context) {
        log::info!("\njoin lobby request: {:#?}", req);
        log::info!(
            "Player `{}` joined lobby `{}`, current lobbies:\n",
            req.player_id,
            req.lobby_name
        );

        let player_id = Uuid::parse_str(&req.player_id).expect("Not a valid uuid");
        let _ = self.lobbies.add_player_to(player_id, req.lobby_name);

        log::info!("{:#?}\n", &self.lobbies);
    }
}

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

        log::info!("{:#?}\n", &self.lobbies.get());
    }
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
