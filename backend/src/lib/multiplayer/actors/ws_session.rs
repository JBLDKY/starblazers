// This module defines the `WsLobbySession` actor, which handles WebSocket
// communication for each player session. It manages the lifecycle of a
// WebSocket connection, including sending and receiving messages, maintaining
// heartbeats to keep the connection alive, and forwarding messages to the
// `LobbyManager` actor for further processing.

use super::{LobbyManager, UserStateManager};
use crate::multiplayer::communication::message::{Disconnect, Message, RegisterWebSocket};
use crate::multiplayer::communication::protocol::{
    ProtocolHandler, SynchronizeState, WebSocketMessage,
};
use crate::multiplayer::UserState;
use actix::prelude::*;
use actix::{Actor, Addr, Handler, Running, StreamHandler};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsSession {
    /// Player's user state to identify the state and connection
    pub user_state: UserState,
    pub user_id: Uuid,

    ///Unique id for the websocket session for a user
    pub connection_id: Uuid,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// Lobby manager server
    pub lobby_manager_addr: Addr<LobbyManager>,

    /// User state manager
    pub user_state_manager_addr: Addr<UserStateManager>,
}

impl WsSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| act.check_heartbeat(ctx));
    }

    /// Runs every HEARTBEAT_INTERVAL, stops if hb times out
    fn check_heartbeat(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        // Log the current state at each interval
        let state = SynchronizeState::from(self.user_state.clone());
        let s = serde_json::to_string(&state).expect("Could not parse state to string");

        if Instant::now().duration_since(self.hb) < CLIENT_TIMEOUT {
            log::debug!("{:?}", &self.connection_id);
            ctx.ping(b"");
            ctx.text(s);
        } else {
            // Heartbeat timed out
            ctx.stop();
        }
    }

    pub fn set_player_state(&mut self, player_state: UserState) {
        self.user_state = player_state;
    }

    pub fn player_state_mut(&mut self) -> &mut UserState {
        &mut self.user_state
    }

    pub fn user_id(&self) -> Option<Uuid> {
        self.user_state.user_id()
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in UserStateManager. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsLobbySession, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.user_state_manager_addr
            .send(RegisterWebSocket {
                addr: addr.recipient(),
                connection_id: self.connection_id,
                user_id: self.user_id,
            })
            .into_actor(self)
            .then(|_res, _act, _ctx| {
                // match res {
                //     Ok(res) => act.connection_id = res,
                //     // something is wrong with chat server
                //     _ => ctx.stop(),
                // }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        if let Some(id) = self.user_state.user_id() {
            // notify chat server
            self.lobby_manager_addr
                .do_send(Disconnect { id: id.to_string() });
        }

        Running::Stop
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();

                if !m.starts_with('{') {
                    return;
                }

                match serde_json::from_str::<WebSocketMessage>(m) {
                    Ok(message) => message.handle(self, ctx),
                    Err(e) => {
                        log::error!("Failed to parse message: {}", e);
                        log::error!("Message contents: {}", m);
                    }
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Message> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
