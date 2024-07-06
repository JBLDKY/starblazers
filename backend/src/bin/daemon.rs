#![cfg(feature = "daemon")]
use daemonize::Daemonize;
use futures_util::SinkExt;
use futures_util::StreamExt;
use reqwest::Client;
use service::daemon::basic::{create_player, create_player_and_get_jwt};
use service::pid_file::{ERROUT, PID_FILE, SOCKET_PATH, STDOUT};
use std::collections::HashMap;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::handshake::client::generate_key;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use uuid::Uuid;

type WebSocketHandle = JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>;

#[derive(Debug)]
struct GlobalState {
    websocket_handles: HashMap<Uuid, WebSocketHandle>,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            websocket_handles: HashMap::new(),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Keep the daemon running and waiting for commands
    let listener = init_daemon().expect("Failed to init daemon");
    let state = Arc::new(Mutex::new(GlobalState::new()));

    loop {
        let (mut stream, _) = listener.accept().await?;
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await.unwrap();
            let command = std::str::from_utf8(&buffer[..n]).unwrap();

            let response = match command.trim() {
                "helloworld" => handle_hello_world().await,
                "np" => {
                    let (user, pass) = create_player().await.expect("Couldnt create player");
                    format!("user: {}/pass: {}", user, pass)
                }

                "jwt" => create_player_and_get_jwt().await.expect("No jwt for u"),
                "list" => format!("{:#?}", state),
                "ws" => {
                    let handle = new_websocket();
                    let id = Uuid::new_v4();
                    state.lock().unwrap().websocket_handles.insert(id, handle);
                    "WebSocket connection spawned".to_string()
                }
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

fn new_websocket() -> JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
    tokio::spawn(async move {
        let url = "ws://localhost:3030/lobby";
        let jwt = create_player_and_get_jwt()
            .await
            .expect("Failed to create new player and get jwt");
        let request = Request::builder()
            .method("GET")
            .uri(url)
            .header("Cookie", format!("Authorization={}", jwt))
            .header("Host", url)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .body(())
            .unwrap();

        let (ws_stream, _) = connect_async(request).await?;
        let (mut write, mut read) = ws_stream.split();
        let last_ping = Arc::new(Mutex::new(Instant::now()));
        let timeout_check = Arc::clone(&last_ping);

        println!("WebSocket connection established successfully");

        let read_task = tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => match msg {
                        Message::Ping(ping) => {
                            println!("ping");
                            *last_ping.lock().unwrap() = Instant::now();
                            if let Err(e) = write.send(Message::Pong(ping)).await {
                                eprintln!("Error sending pong: {:?}", e);
                                break;
                            }
                        }
                        Message::Text(text) => {
                            println!("Message: {}", text);
                        }
                        _ => println!("OTher message: {:?}", msg),
                    },
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        break;
                    }
                }
            }
            println!("WebSocket read task died");
        });

        let timeout_task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let last_ping = timeout_check.lock().unwrap();
                if Instant::now().duration_since(*last_ping) > Duration::from_secs(10) {
                    eprintln!("Connection timed out");
                    break;
                }
            }
        });

        tokio::select! {
            _ = read_task => println!("Read task finished"),
            _ = timeout_task => println!("Timeout task finished"),
        }

        Ok(())
    })
}
