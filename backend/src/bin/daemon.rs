#![cfg(feature = "daemon")]
use daemonize::Daemonize;
use dotenv::dotenv;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use service::daemon::basic::get_local_address;
use service::daemon::basic::get_local_websockt;
use service::daemon::basic::handle_hello_world;
use service::daemon::basic::{create_player, create_player_and_get_jwt};
use service::pid_file::{PID_FILE, SOCKET_PATH, STDOUT};
use service::types::PublicUserRecord;
use std::collections::HashMap;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::UnixListener;
use tokio::sync::{mpsc, watch, Mutex as TokioMutex};
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
struct Task {
    id: Uuid,
    player_id: Uuid,
    jwt: String,
    handle: WebSocketHandle,
    cancel_sender: watch::Sender<bool>,
    message_sender: mpsc::Sender<String>,
}

#[derive(Debug)]
struct GlobalState {
    websocket_handles: HashMap<Uuid, Task>,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            websocket_handles: HashMap::new(),
        }
    }

    /// Looks for an active websocket id starting with kill_id.
    /// Sends a message to the websocket's receiver channel which is
    /// the signal to close itself.
    /// The websocket is then also removed from `websocket_handles`.
    fn kill_websocket(&mut self, kill_id: String) -> Option<Uuid> {
        let to_kill = {
            *self
                .websocket_handles
                .keys()
                .find(|active_id| active_id.to_string().starts_with(&kill_id))?
        };

        let task = &self
            .websocket_handles
            .get(&to_kill)
            .expect("Task not found");

        task.cancel_sender.send(true).ok();

        self.websocket_handles
            .remove(&to_kill.clone())
            .map(|task| task.id);

        Some(to_kill)
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum UnixSocketMessage {
    HelloWorld,
    CreatePlayer,
    CreatePlayerAndGetJwt,
    ListWebsockets,
    NewWebsocket { id: Option<Uuid> },
    KillWebsocket { substring: String },
    Unknown(String),
}

impl From<&str> for UnixSocketMessage {
    fn from(cmd: &str) -> Self {
        match cmd.trim() {
            "helloworld" => UnixSocketMessage::HelloWorld,
            "np" => UnixSocketMessage::CreatePlayer,
            "jwt" => UnixSocketMessage::CreatePlayerAndGetJwt,
            "list_websocket" => UnixSocketMessage::ListWebsockets,

            // TODO: This is dogshit and not scalable
            // waiting until there is other stuff to kill before refactoring.
            s if s.starts_with("kill") => {
                // Expected format:
                // kill <target> <id>
                // e.g.:
                // kill websocket d8a
                let mut command_parts = s.split(" ").skip(1);
                // value for --target or -t
                let target = command_parts.next();
                // value for --id or -i
                let id = command_parts.next();

                if target.is_none() | id.is_none() {
                    // gracefully handle invalid user input
                    return UnixSocketMessage::Unknown(cmd.to_string());
                }

                match target.unwrap() {
                    // The only valid result for now
                    "websocket" => UnixSocketMessage::KillWebsocket {
                        substring: id.unwrap().to_string(),
                    },
                    _ => todo!(),
                }
            }

            // Create a websocket with a specified id
            s if s.starts_with("ws") => {
                let id = s
                    .split_whitespace()
                    .nth(1)
                    .and_then(|id| Uuid::parse_str(id).ok());
                UnixSocketMessage::NewWebsocket { id }
            }
            _ => UnixSocketMessage::Unknown(cmd.to_string()),
        }
    }
}

impl From<String> for UnixSocketMessage {
    fn from(cmd: String) -> Self {
        UnixSocketMessage::from(cmd.as_str())
    }
}

impl From<Uuid> for UnixSocketMessage {
    fn from(id: Uuid) -> Self {
        UnixSocketMessage::NewWebsocket { id: Some(id) }
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
        // The next line blocks the thread until a message is sent from the CLI
        let (mut stream, _) = listener.accept().await?;

        // Need to clone this because we're in a loop
        let state = Arc::clone(&state);

        // Spawn an async task to handle the message we received
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let n = stream
                .read(&mut buffer)
                .await
                .expect("UnixSocket Connection failed");

            // Parse the command into a any of the possible enums
            let command: UnixSocketMessage = match serde_json::from_slice(&buffer[..n]) {
                Ok(msg) => msg,
                Err(_) => {
                    // If we can't parse it as JSON, treat it as a plain string command
                    UnixSocketMessage::from(String::from_utf8_lossy(&buffer[..n]).to_string())
                }
            };

            log::info!("Received command: {:?}", command);

            let response = match command {
                UnixSocketMessage::HelloWorld => handle_hello_world().await,
                UnixSocketMessage::CreatePlayer => {
                    let (user, pass) = create_player().await.expect("Couldn't create player");
                    format!("user: {}/pass: {}", user, pass)
                }
                UnixSocketMessage::CreatePlayerAndGetJwt => {
                    create_player_and_get_jwt().await.expect("No jwt for u")
                }
                UnixSocketMessage::ListWebsockets => format!("{:#?}", state.lock().unwrap()),
                UnixSocketMessage::NewWebsocket { id } => {
                    let id = id.unwrap_or_else(Uuid::new_v4);
                    let task = new_websocket(Some(id)).await;
                    state.lock().unwrap().websocket_handles.insert(id, task);
                    format!("WebSocket connection spawned, connection_id: {}", id)
                }
                UnixSocketMessage::KillWebsocket { ref substring } => {
                    gracefully_shutdown_websocket(&state, substring.to_string()).await
                }
                UnixSocketMessage::Unknown(ref cmd) => format!("Unknown command: {}", cmd),
            };

            log::info!(
                "Successfully executed: {:?}. Response: {}",
                command,
                response
            );

            // Return a message to the CLI
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
    connection_id: Option<Uuid>,
    jwt: Option<String>,
) -> tokio_tungstenite::tungstenite::http::Request<()> {
    let url = get_local_websockt();

    let connection_id: String = match connection_id {
        Some(v) => v.into(),
        None => Uuid::new_v4().into(),
    };

    let jwt = match jwt {
        Some(token) => token,
        None => create_player_and_get_jwt()
            .await
            .expect("Failed to create new player and get jwt"),
    };

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
#[allow(clippy::let_underscore_future)]
async fn new_websocket(connection_id: Option<Uuid>) -> Task {
    let (cancel_sender, mut cancel_receiver) = watch::channel(false);

    let connection_id = connection_id.unwrap_or(Uuid::new_v4());

    // TODO: Handle error
    let jwt = create_player_and_get_jwt()
        .await
        .expect("Failed to create player");

    let request = new_websocket_request(Some(connection_id), Some(jwt.clone())).await;

    let (message_sender, message_receiver) = mpsc::channel(100);

    let handle = tokio::spawn(async move {
        // Establish the WebSocket connection
        let (read, write) = establish_connection(request).await?;

        // Needs to wrapped in Mutex because it is used in the while loop
        let write = Arc::new(TokioMutex::new(write));

        // Create a shared timestamp for the last ping received
        let last_ping = Arc::new(Mutex::new(Instant::now()));

        // Spawn a task to handle incoming WebSocket messages
        let read_task = spawn_read_task(read, Arc::clone(&last_ping), write.clone());

        // Spawn a task to monitor for connection timeouts
        let timeout_task = spawn_timeout_task(Arc::clone(&last_ping));

        // Spawn a task for sending messages to the server
        let write_task = spawn_write_task(message_receiver, write.clone());

        // Wait for either the read task or the timeout task to finish
        tokio::select! {
        _ = cancel_receiver.changed() => {
            let mut write_lock = write.lock().await;
            // Send `Close` msg over websocket connection
            write_lock.close().await?;
        }

            result = read_task => {
                log::info!("Read task finished: {:?}", result);
            }

            _ = timeout_task => {
                log::info!("Timed out: {:?}", &connection_id);
            }

            result = write_task => {
                log::info!("Write task finished: {:?}", result);
            }
        }

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    });

    let player_info = Client::new()
        .get(format!("{}players/player", get_local_address()))
        .header("authorization", format!("Bearer {}", &jwt))
        .send()
        .await
        .expect("Failed to send player info request");

    if player_info.status() != StatusCode::OK {
        log::error!("Did not receive player info in new_websocket function.");
    }

    let record: PublicUserRecord = player_info.json().await.unwrap();
    let player_id: Uuid = Uuid::parse_str(&record.uuid).expect("Player has an invalid ID");

    Task {
        id: connection_id,
        jwt,
        player_id,
        handle,
        cancel_sender,
        message_sender,
    }
}

/// Establishes new websocket connection that is offloaded to an asynchronous task.
/// A new player is created and authenticated.
async fn establish_connection(
    request: Request<()>,
) -> Result<(SStream, SSink), Box<dyn std::error::Error + Send + Sync>> {
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
    mut write: Arc<TokioMutex<SSink>>,
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
async fn handle_message(
    msg: Message,
    last_ping: &Arc<Mutex<Instant>>,
    write: &mut Arc<TokioMutex<SSink>>,
) {
    let mut write_lock = write.lock().await;
    match msg {
        Message::Ping(ping) => {
            log::info!("Received ping");
            *last_ping.lock().unwrap() = Instant::now();
            if let Err(e) = write_lock.send(Message::Pong(ping)).await {
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

/// Sends messages to the server.
async fn spawn_write_task(
    mut message_receiver: mpsc::Receiver<String>,
    write: Arc<TokioMutex<SSink>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(message) = message_receiver.recv().await {
        let mut write_lock = write.lock().await;
        write_lock.send(Message::Text(message)).await?;
    }
    Ok(())
}

/// Look for a websocket in the state. If found, tell it to shut itself down.
/// This finishes the Task which then exits to prevent leaks.
async fn gracefully_shutdown_websocket(
    state: &Arc<Mutex<GlobalState>>,
    connection_id: String,
) -> String {
    let result = state
        .lock()
        .expect("Failed to lock mutex")
        .kill_websocket(connection_id);

    match result {
        Some(id) => format!("Succesfully shut down websocket: {:?}", id),
        None => "Websocket starting with id: `{:?}` does not exist".to_string(),
    }
}

// async fn create_lobby(
//     state: &GlobalState,
//     connection_id: Uuid,
//     lobby_name: String,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let connection = state
//         .websocket_handles
//         .get(&connection_id)
//         .ok_or("Connection not found")?;
//
//     let connection = connection.lock().await;
//
//     // Verify that the player is authenticated
//     let player_id = connection
//         .player_id
//         .as_ref()
//         .ok_or("Player not authenticated")?;
//
//     // Check if the lobby already exists
//     let mut lobbies = state.lobbies.lock().await;
//     if lobbies.contains_key(&lobby_name) {
//         return Err("Lobby already exists".into());
//     }
//
//     // Create the lobby
//     let lobby = Lobby {
//         name: lobby_name.clone(),
//         host_id: player_id.clone(),
//         players: vec![player_id.clone()],
//     };
//     lobbies.insert(lobby_name.clone(), lobby);
//
//     // Send confirmation message to the client
//     let message = json!({
//         "type": "LobbyCreated",
//         "lobby_name": lobby_name,
//         "player_id": player_id,
//     });
//     connection.sender.send(serde_json::to_string(&message)?)?;
//
//     println!("Lobby '{}' created by player '{}'", lobby_name, player_id);
//
//     Ok(())
// }
