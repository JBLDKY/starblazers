#![cfg(feature = "daemon")]
use daemonize::Daemonize;
use dotenv::dotenv;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::StreamExt;
use reqwest::Client;
use service::daemon::basic::get_local_address;
use service::daemon::basic::get_local_websockt;
use service::daemon::basic::handle_hello_world;
use service::daemon::basic::{create_player, create_player_and_get_jwt};
use service::pid_file::{PID_FILE, SOCKET_PATH, STDOUT};
use std::collections::HashMap;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::UnixListener;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::handshake::client::generate_key;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use uuid::Uuid;

type SSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type SStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WebSocketHandle = JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

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
    dotenv().ok();

    // Keep the daemon running and waiting for commands
    let listener = init_daemon().expect("Failed to init daemon");
    let state = Arc::new(Mutex::new(GlobalState::new()));

    loop {
        let (mut stream, _) = listener.accept().await?;
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await.unwrap();
            let command = std::str::from_utf8(&buffer[..n]).unwrap().trim();

            log::info!("Received command: {}", command);

            let response = match command.trim() {
                "helloworld" => handle_hello_world().await,
                "np" => {
                    let (user, pass) = create_player().await.expect("Couldnt create player");
                    format!("user: {}/pass: {}", user, pass)
                }
                "jwt" => create_player_and_get_jwt().await.expect("No jwt for u"),
                "list_websocket" => format!("{:#?}", state),
                "ws" => {
                    let handle = new_websocket();
                    let id = Uuid::new_v4();
                    state.lock().unwrap().websocket_handles.insert(id, handle);
                    "WebSocket connection spawned".to_string()
                }
                _ => "Unknown command".to_string(),
            };

            log::info!(
                "Successfully executed: {}. Response: {}",
                command.trim(),
                response
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
    }
}

/// Initializes the Daemon configuration, STDOUT, and the Unixsocket.
/// Returns a listener for the socket.
fn init_daemon() -> Result<UnixListener, anyhow::Error> {
    let stdout = File::create(STDOUT)?;
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
        .stderr(stdout.try_clone()?);

    if let Err(e) = daemonize.start() {
        log::info!("Failed to initialize Daemon: {}", e)
    }

    if let Ok(metadata) = std::fs::metadata(PID_FILE) {
        let mut perms = metadata.permissions();
        perms.set_mode(0o644);
        std::fs::set_permissions(PID_FILE, perms)?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;

    log::info!("Initialized");
    Ok(listener)
}

/// Returns a http request with which a websocket connection can be established
/// A JWT can be provided to this function. Must be prefixed by Bearer!
/// If None is provided, a new account will be created and authenticated to get
/// a fresh JWT.
async fn new_websocket_request(
    jwt: Option<String>,
) -> tokio_tungstenite::tungstenite::http::Request<()> {
    let url = get_local_websockt();

    let jwt = match jwt {
        Some(token) => token,
        None => create_player_and_get_jwt()
            .await
            .expect("Failed to create new player and get jwt"),
    };

    let connection_id: String = Uuid::new_v4().into();

    Request::builder()
        .method("GET")
        .uri(&url)
        .header("Cookie", format!("Authorization={}", jwt))
        .header("Host", &url)
        .header("Connection", "Upgrade")
        .header("Connection-ID", &connection_id)
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .body(())
        .expect("Failed to build websocket request")
}

/// Connects a new websocket to the server in an asynchronous task. Runs
/// until it is manually terminated.
fn new_websocket() -> JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
    tokio::spawn(async move {
        // Establish the WebSocket connection
        let (read, write) = establish_connection().await?;

        // Create a shared timestamp for the last ping received
        let last_ping = Arc::new(Mutex::new(Instant::now()));

        // Spawn a task to handle incoming WebSocket messages
        let read_task = spawn_read_task(read, Arc::clone(&last_ping), write);

        // Spawn a task to monitor for connection timeouts
        let timeout_task = spawn_timeout_task(Arc::clone(&last_ping));

        // Wait for either the read task or the timeout task to finish
        tokio::select! {
            _ = read_task => log::info!("Read task finished"),
            _ = timeout_task => log::info!("Timeout task finished"),
        }

        Ok(())
    })
}

/// Establishes new websocket connection that is offloaded to an asynchronous task.
/// A new player is created and authenticated.
async fn establish_connection() -> Result<(SStream, SSink), Box<dyn std::error::Error + Send + Sync>>
{
    let request = new_websocket_request(None).await;
    let (ws_stream, _) = connect_async(request).await?;
    let (write, read) = ws_stream.split();
    log::info!("WebSocket connection established successfully");
    Ok((read, write))
}

/// Monitors WebSocket connection for timeouts.
/// Spawns a task that checks the Stream for incoming messages.
/// Stops the thread if something goes wrong reading a message.
/// Forwards messages to handle_message() if there is no error.
fn spawn_read_task(
    mut read: SStream,
    last_ping: Arc<Mutex<Instant>>,
    mut write: SSink,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => handle_message(msg, &last_ping, &mut write).await,
                Err(e) => {
                    log::error!("Error: {:?}", e);
                    break;
                }
            }
        }
        log::info!("WebSocket read task ended");
    })
}

/// Handles incoming websocket traffic.
/// Responds to Ping by sending a Pong.
/// Logs received Text to STDOUT.
async fn handle_message(msg: Message, last_ping: &Arc<Mutex<Instant>>, write: &mut SSink) {
    match msg {
        Message::Ping(ping) => {
            log::info!("Received ping");
            *last_ping.lock().unwrap() = Instant::now();
            if let Err(e) = write.send(Message::Pong(ping)).await {
                log::error!("Error sending pong: {:?}", e);
            }
        }
        Message::Text(text) => log::info!("Received message: {}", text),
        _ => log::info!("Received other message: {:?}", msg),
    }
}
/// Monitors WebSocket connection for timeouts.
/// Spawns a task that checks the time since last ping every HEARTBEAT_INTERVAL.
/// Logs and terminates if no ping received within CONNECTION_TIMEOUT.
fn spawn_timeout_task(last_ping: Arc<Mutex<Instant>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(HEARTBEAT_INTERVAL).await;
            let last_ping = last_ping.lock().unwrap();
            if Instant::now().duration_since(*last_ping) > CONNECTION_TIMEOUT {
                log::info!("Connection timed out");
                break;
            }
        }
    })
}
