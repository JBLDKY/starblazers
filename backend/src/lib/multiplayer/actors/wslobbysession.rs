// This module defines the `WsLobbySession` actor, which handles WebSocket
// communication for each player session. It manages the lifecycle of a
// WebSocket connection, including sending and receiving messages, maintaining
// heartbeats to keep the connection alive, and forwarding messages to the
// `LobbyManager` actor for further processing.

use super::LobbyManager;
use crate::multiplayer::communication::message::{Disconnect, Message};
use crate::multiplayer::communication::player_context::InMenu;
use crate::multiplayer::communication::protocol::{ProtocolHandler, WebSocketMessage};
use crate::multiplayer::{HasPlayerId, PlayerContext};
use actix::prelude::*;
use actix::{Actor, Addr, Handler, Running, StreamHandler};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsLobbySession {
    /// Player's Uuid to identify the connection
    pub player_ctx: PlayerContext<InMenu>,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// Chat server
    pub addr: Addr<LobbyManager>,
}

impl WsLobbySession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) < CLIENT_TIMEOUT {
                ctx.ping(b"");
                return;
            }

            // heartbeat timed out
            log::info!("Heartbeat stopped: {}", act.player_ctx.id());

            // notify chat server
            act.addr.do_send(Disconnect {
                id: act.player_ctx.id().to_string(),
            });

            // stop actor
            ctx.stop();
        });
    }
}

impl Actor for WsLobbySession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(Disconnect {
            id: self.player_ctx.id().to_string(),
        });

        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Message> for WsLobbySession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsLobbySession {
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
