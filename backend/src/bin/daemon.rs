#![cfg(feature = "daemon")]
use daemonize::Daemonize;
use futures_util::StreamExt;
use reqwest::Client;
use service::pid_file::{ERROUT, PID_FILE, SOCKET_PATH, STDOUT};
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::UnixListener;
use tokio_tungstenite::tungstenite::handshake::client::generate_key;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::{connect_async, WebSocketStream};

// struct SimulatedPlayer {
//     id: String,
// }
//
// impl Actor for SimulatedPlayer {
//     type Context = ws::WebsocketContext<Self>;
//
//     fn started(&mut self, ctx: &mut Self::Context) {
//         ctx.run_interval(Duration::from_secs(1), |act, ctx| {
//             let msg = format!("{{\"type\": \"move\", \"player\": \"{}\"}}", act.id);
//             ctx.text(msg);
//         });
//     }
// }
//
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SimulatedPlayer {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         if let Ok(ws::Message::Text(text)) = msg {
//             println!("Received: {}", text);
//         }
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Keep the daemon running and waiting for commands
    let listener = init_daemon().expect("Failed to init daemon");

    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await.unwrap();
            let command = std::str::from_utf8(&buffer[..n]).unwrap();

            let response = match command.trim() {
                "helloworld" => handle_hello_world().await,
                "ws" => new_websocket()
                    .await
                    .unwrap_or("Error occurred.".to_string()),
                _ => "Unknown command".to_string(),
            };

            stream.write_all(response.as_bytes()).await.unwrap();
        });
    }
}

async fn handle_hello_world() -> String {
    let client = Client::new();
    match client
        .get("http://localhost:3030/helloworld".to_string())
        .send()
        .await
    {
        Ok(resp) => resp
            .text()
            .await
            .unwrap_or_else(|_| "Failed to get response text".to_string()),
        Err(e) => format!("Request failed: {}", e),
    }
}

fn init_daemon() -> Result<UnixListener, anyhow::Error> {
    let stdout = File::create(STDOUT)?;
    let stderr = File::create(ERROUT)?; // Remove the socket file if it already exists
                                        //
    if std::path::Path::new(SOCKET_PATH).exists() {
        std::fs::remove_file(SOCKET_PATH)?;
    }

    let daemonize = Daemonize::new()
        .pid_file(PID_FILE)
        .chown_pid_file(false)
        .working_directory("/tmp")
        .umask(0o022)
        .stdout(stdout.try_clone()?)
        .stderr(stderr.try_clone()?);

    if let Err(e) = daemonize.start() {
        eprintln!("Failed to initialize Daemon: {}", e)
    }

    if let Ok(metadata) = std::fs::metadata(PID_FILE) {
        let mut perms = metadata.permissions();
        perms.set_mode(0o644);
        std::fs::set_permissions(PID_FILE, perms)?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;

    Ok(listener)
}

async fn new_websocket() -> Result<String, Box<dyn std::error::Error>> {
    let url = "ws://localhost:3030/lobby";
    let jwt = "";
    let ws_key = generate_key();

    println!("building request");
    let request = Request::builder()
        .method("GET")
        .uri(url)
        .header("Cookie", format!("Authorization={}", jwt))
        .header("Host", url)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", ws_key)
        .body(())
        .unwrap();

    println!("building request succeeded");

    // Connect with the custom request
    let (ws_stream, _) = match connect_async(request).await {
        Ok((w, r)) => (w, r),
        Err(e) => return Ok(e.to_string()),
    };

    // At this point, you have a connected WebSocket stream
    // You might want to spawn a task to handle incoming messages, for example:
    tokio::spawn(async move {
        let (write, read) = ws_stream.split();
    });

    Ok("WebSocket connection established successfully".to_string())
}
