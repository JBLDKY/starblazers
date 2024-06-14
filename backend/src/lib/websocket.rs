use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::claims::{Claims, TokenError};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Default for MyWebSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                log::error!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                // parse
                // verify
                // send back
                let json = serde_json::from_str::<GameState>(&text);

                if json.is_err() {
                    ctx.text("invalid json");
                    return;
                }

                let mut res = json.unwrap();
                res.move_up();

                ctx.text(format!(
                    "gameStateVerified: {}",
                    serde_json::to_value(res).unwrap()
                ));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

pub static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Starblazers Sanity Check</title>
    </head>

    <br>

    <body>
    Welcome to StarBlazers index.html! <br>

    The server is working I guess...<br>

    Now what?
    </body>
</html>
"#;

#[derive(Serialize, Deserialize, Default, Debug)]
struct GameState {
    r#type: String,
    data: HashMap<String, usize>,
}

#[allow(dead_code)]
impl GameState {
    fn move_up(&mut self) {
        if let Some(y) = self.data.get_mut("y") {
            *y -= 1;
        }
    }
    fn move_down(&mut self) {
        if let Some(y) = self.data.get_mut("y") {
            *y += 1;
        }
    }
    fn move_left(&mut self) {
        if let Some(x) = self.data.get_mut("x") {
            *x -= 1;
        }
    }
    fn move_right(&mut self) {
        if let Some(x) = self.data.get_mut("x") {
            *x += 1;
        }
    }
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

/// List of available lobbies
pub struct ListLobbies;

impl actix::Message for ListLobbies {
    type Result = Vec<String>;
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
    player_count: Arc<AtomicUsize>,
}

impl LobbyServer {
    pub fn new(player_count: Arc<AtomicUsize>) -> LobbyServer {
        // default room
        let mut lobbies = HashMap::new();
        lobbies.insert("main".to_owned(), HashSet::new());

        LobbyServer {
            sessions: HashMap::new(),
            lobbies,
            player_count,
        }
    }
}

impl LobbyServer {
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
        self.send_message("main", "Someone joined", "0".to_string());

        // register session with random id
        let id = msg.claims.uuid.clone();
        self.sessions.insert(id.clone(), msg.addr);

        // auto join session to main room
        self.lobbies
            .entry("main".to_owned())
            .or_default()
            .insert(msg.claims.uuid.clone());

        let count = self
            .player_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        self.send_message("main", &format!("Total visitors {count}"), "0".to_string());
        self.send_message(
            "main",
            &format!("Current players {:?}", self.player_count.as_ref()),
            "0".to_string(),
        );

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
            let count = self
                .player_count
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);

            self.send_message(
                "main",
                &format!("Current players after someone left: {count}"),
                "0".to_string(),
            );

            // remove session from all lobbies
            for (name, sessions) in &mut self.lobbies {
                if sessions.remove(&msg.id) {
                    lobbies.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in lobbies {
            self.send_message(&room, "Someone disconnected", "0".to_string());
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

/// Handler for `Listlobbies` message.
impl Handler<ListLobbies> for LobbyServer {
    type Result = MessageResult<ListLobbies>;

    fn handle(&mut self, _: ListLobbies, _: &mut Context<Self>) -> Self::Result {
        let mut lobbies = Vec::new();

        for key in self.lobbies.keys() {
            lobbies.push(key.to_owned())
        }

        MessageResult(lobbies)
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
            self.send_message(&room, "Someone disconnected", "0".to_string());
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

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsLobbySessionState, state is shared
        // across all routes within application
        // let addr = ctx.address();
        // self.addr
        //     .send(Connect {
        //         addr: addr.recipient(),
        //     })
        //     .into_actor(self)
        //     .then(|res, act, ctx| {
        //         match res {
        //             Ok(res) => act.id = res,
        //             // something is wrong with chat server
        //             _ => ctx.stop(),
        //         }
        //         fut::ready(())
        //     })
        //     .wait(ctx);
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

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");
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
                if m.starts_with("{\"type\":\"auth\"}") {
                    let auth = serde_json::from_str::<WebsocketAuthJwt>(m)
                        .expect("Invalid websocket auth jwt format");
                    let claims = auth.claims().expect("Failed to parse claims");

                    let addr = ctx.address().into();
                    self.addr.do_send(Connect { addr, claims });
                } else if m.starts_with("{\"type\":\"gamestate\"}") {
                    log::warn!("received game info: {}", m);
                } else {
                    let msg = if let Some(ref name) = self.name {
                        format!("{name}: {m}")
                    } else {
                        m.to_owned()
                    };
                    log::warn!("{}", &msg);
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

struct RingBuffer<T, const S: usize> {
    buffer: [Option<T>; S],
    start: usize,
    end: usize,
}

impl<T, const S: usize> RingBuffer<T, S> {
    fn new() -> Self {
        // Initialize the buffer with None values.
        // Using .map() here instead of [None; S] lifts the restriction that T must implement copy
        Self {
            buffer: [(); S].map(|_| None),
            start: 0,
            end: 0,
        }
    }
}
