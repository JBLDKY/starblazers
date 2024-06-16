use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use thiserror::Error;

use actix::prelude::*;
use actix_web_actors::ws;

use crate::claims::{Claims, TokenError};
use crate::ringbuffer::RingBuffer;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct GameState {
    r#type: String,
    position_x: usize,
    position_y: usize,
    player_id: String,
    timestamp: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New chat session is created
#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub claims: Claims,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}
/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub lobby: String,
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client ID
    pub id: usize,

    /// Room name
    pub name: String,
}

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

#[derive(Debug)]
pub struct WsLobbySession {
    /// unique session id
    pub id: usize,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// joined room
    pub lobby: String,

    /// peer name
    pub name: Option<String>,

    /// Chat server
    pub addr: Addr<LobbyServer>,
}

impl WsLobbySession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(Disconnect {
                    id: act.id.to_string(),
                });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
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
            id: self.id.to_string(),
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
                // we check for /sss type of messages
                if m.starts_with("{\"type\":\"auth\"") {
                    let auth = serde_json::from_str::<WebsocketAuthJwt>(m)
                        .expect("Invalid websocket auth jwt format");
                    let claims = auth.claims().expect("Failed to parse claims");

                    let addr = ctx.address().into();
                    self.addr.do_send(Connect { addr, claims });
                } else if m.starts_with("{\"type\":\"gamestate\"") {
                    let gs = serde_json::from_str::<GameState>(m).expect("couldnt parse gamestate");

                    self.addr.do_send(gs);
                } else {
                    log::error!("unknown dataformat: {}", m);

                    let msg = if let Some(ref name) = self.name {
                        format!("{name}: {m}")
                    } else {
                        m.to_owned()
                    };
                    // send message to chat server
                    self.addr.do_send(ClientMessage {
                        id: self.id,
                        msg,
                        lobby: self.lobby.clone(),
                    })
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

#[derive(Serialize, Deserialize, Debug)]
struct WebsocketAuthJwt {
    r#type: String,
    jwt: String,
}
impl WebsocketAuthJwt {
    fn claims(&self) -> Result<Claims, TokenError> {
        Claims::decode(self.jwt.split(' ').last().ok_or(TokenError::ParseError)?)
    }
}

impl Handler<GameState> for LobbyServer {
    type Result = ();

    fn handle(&mut self, state: GameState, _: &mut Self::Context) {
        let player_id = state.player_id.clone();

        let player_ring = self
            .ring
            .get_mut(&player_id)
            .expect("Could not access player ring");

        player_ring.push(state.clone());

        self.send_message(
            "main",
            &serde_json::to_string(&state.clone()).expect("couldnt parse gamestate to string"),
            player_id,
        );
    }
}

#[derive(Error, Debug)]
pub enum InvalidDataError {
    #[error("Player id is not a valid uuid: {0}")]
    PlayerIdIsNotUuid(String),
}
