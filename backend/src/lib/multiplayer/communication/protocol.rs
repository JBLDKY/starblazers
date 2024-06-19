use super::{
    common::{CreateLobbyRequest, GameState, JoinLobbyRequest, LeaveLobbyRequest},
    message::Connect,
    user_state::UserEvent,
};
use crate::{
    claims::{Claims, TokenError},
    multiplayer::WsLobbySession,
};
use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

/// This file defines data structures specifically used for sending and
/// receiving data over WebSocket connections. These structures are
/// serialized and deserialized for communication between the client
/// and server in real-time.
pub trait ProtocolHandler {
    fn handle(self, session: &mut WsLobbySession, ctx: &mut ws::WebsocketContext<WsLobbySession>);
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")] // e.g. this adds: `{"type": "CreateLobby"}` to the serialized json
pub enum WebSocketMessage {
    Auth(WebsocketAuthJwt),
    GameState(GameState),
    CreateLobby(CreateLobbyRequest),
    JoinLobby(JoinLobbyRequest),
    LeaveLobby(LeaveLobbyRequest),
}

impl ProtocolHandler for WebSocketMessage {
    fn handle(self, session: &mut WsLobbySession, ctx: &mut ws::WebsocketContext<WsLobbySession>) {
        match self {
            WebSocketMessage::Auth(auth) => auth.handle(session, ctx),
            WebSocketMessage::GameState(gs) => gs.handle(session, ctx),
            WebSocketMessage::CreateLobby(create_lobby) => create_lobby.handle(session, ctx),
            WebSocketMessage::JoinLobby(join_lobby) => join_lobby.handle(session, ctx),
            WebSocketMessage::LeaveLobby(leave_lobby) => leave_lobby.handle(session, ctx),
        }
    }
}

/// Structure representing a WebSocket authentication message containing a JWT
#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketAuthJwt {
    /// JSON Web Token (JWT) for authentication
    jwt: String,
}

impl WebsocketAuthJwt {
    /// Decodes the JWT to extract claims
    ///
    /// # Returns
    ///
    /// A result containing the decoded `Claims` if successful,
    /// or a `TokenError` if decoding fails.
    pub fn claims(&self) -> Result<Claims, TokenError> {
        let token = Claims::extract_token(&self.jwt)?;
        Claims::decode(token)
    }
}

impl ProtocolHandler for WebsocketAuthJwt {
    fn handle(self, session: &mut WsLobbySession, ctx: &mut ws::WebsocketContext<WsLobbySession>) {
        let claims = self.claims().expect("Failed to parse claims");

        let event = UserEvent::Login(claims.uuid().expect("Invalid Uuid"));

        let addr = ctx.address().into();

        session.user_state.transition(event);

        session.addr.do_send(Connect { addr, claims });
    }
}
