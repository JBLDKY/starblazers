use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

use crate::multiplayer::WsLobbySession;

use super::{protocol::ProtocolHandler, user_state::UserEvent};

/// This file defines common data structures used in the Actix actor-based
/// communication system of the application. These structures are designed
/// to be shared across different modules, facilitating synchronization
/// and communication between actors. Additionally, these data structures
/// are serialized and sent across the WebSocket in literal form for
/// real-time updates.

/// Structure representing the game state, used for synchronization
#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GameState {
    /// X position of the player in the game
    position_x: usize,
    /// Y position of the player in the game
    position_y: usize,
    /// Unique identifier for the player
    player_id: String,
    /// Timestamp of the game state update
    timestamp: String,
}

impl GameState {
    /// Returns a reference to the player ID
    pub fn player_id(&self) -> &str {
        &self.player_id
    }

    /// Returns a cloned version of the player ID as a String
    pub fn into_player_id(&self) -> String {
        self.player_id.clone()
    }
}

impl ProtocolHandler for GameState {
    fn handle(self, session: &mut WsLobbySession, _: &mut ws::WebsocketContext<WsLobbySession>) {
        session.addr.do_send(self);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateLobbyRequest {
    pub lobby_name: String,
    pub player_id: String,
}

impl ProtocolHandler for CreateLobbyRequest {
    fn handle(self, session: &mut WsLobbySession, _: &mut ws::WebsocketContext<WsLobbySession>) {
        // Send the CreateLobbyRequest to the LobbyManager to create the lobby and add the user to it
        session.addr.do_send(self);

        // If the user state correctly contains a uuid, transition to the InLobby state with the
        // JoinLobby event.
        if let Some(uuid) = session.user_state.user_id() {
            session.user_state.transition(UserEvent::JoinLobby(uuid));
        } else {
            // This shouldn't happen...
            log::error!(
                "Could not create a lobby because user has no uuid.\nUser state: {:?}",
                session.user_state
            );
        };
    }
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug)]
pub struct JoinLobbyRequest {
    pub lobby_name: String,
    pub player_id: String,
}

impl ProtocolHandler for JoinLobbyRequest {
    fn handle(self, session: &mut WsLobbySession, _: &mut ws::WebsocketContext<WsLobbySession>) {
        session.addr.do_send(self);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveLobbyRequest {
    pub lobby_name: String,
    pub player_id: String,
}

impl ProtocolHandler for LeaveLobbyRequest {
    fn handle(self, session: &mut WsLobbySession, _: &mut ws::WebsocketContext<WsLobbySession>) {
        session.addr.do_send(self);
    }
}
