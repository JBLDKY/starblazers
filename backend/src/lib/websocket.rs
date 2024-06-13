use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

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
