#![cfg(feature = "daemon")]
use clap::Parser;
use dotenv::dotenv;
use service::{
    daemon::basic::{get_daemon_status, get_local_websockt, start_daemon, stop_daemon},
    pid_file::{SOCKET_PATH, STDOUT},
};
use std::io::BufRead;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::{fs::File, io::BufReader};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

#[derive(Parser, Debug)]
#[command(author = "Sb Co.", version = "0.1.0", about, long_about = None, infer_long_args = true, infer_subcommands = true)]
struct Args {
    #[command(subcommand)]
    command: StarblazersCommand,
}

#[derive(Parser, Debug)]
enum StarblazersCommand {
    /// The Starblazers Daemon for simulating real player connections to the server.
    /// Must be running most of this CLI to be able to do anything.
    #[command()]
    Daemon(DaemonCommand),
    /// The Starblazers Server related commands.
    #[command()]
    Server(ServerCommand),
    /// The Starblazers Lobby related commands.
    #[command()]
    Lobby(LobbyCommand),
    /// The Starblazers Game related commands.
    #[command()]
    Game(GameCommand),
    /// The daemon's logs.
    #[command()]
    Log {
        #[arg(short, long, default_value = "10")]
        n: usize,
    },
    #[command()]
    List(ListOptions),
    #[command()]
    Recompile,
}

#[derive(Parser, Debug)]
#[command(long_about = None, infer_long_args = true, infer_subcommands = true)]
struct DaemonCommand {
    #[command(subcommand)]
    command: DaemonSubCommand,
}

#[derive(Parser, Debug)]
struct ListOptions {
    /// List WebSocket connections
    #[arg(short, long)]
    websocket: bool,

    /// List active players
    #[arg(short, long)]
    players: bool,

    /// List active games
    #[arg(short, long)]
    games: bool,
}

#[derive(Parser, Debug)]
pub enum DaemonSubCommand {
    /// Launch the daemon if it is not already running.
    Run,
    /// Terminate the daemon if it is not already stopped.
    Kill,
    /// Get the status of the daemon (Running or Stopped).
    Status,
    /// Kill then run the daemon.
    Restart,
}

#[derive(Parser, Debug)]
#[command(long_about = None, infer_long_args = true, infer_subcommands = true)]
struct ServerCommand {
    #[command(subcommand)]
    command: ServerSubCommand,
}

#[derive(Parser, Debug)]
pub enum ServerSubCommand {
    Helloworld,
    Ws,
    Newplayer,
    Jwt,
}

#[derive(Parser, Debug)]
#[command(long_about = None, infer_long_args = true, infer_subcommands = true)]
struct LobbyCommand {
    #[command(subcommand)]
    command: LobbySubCommand,
}

#[derive(Parser, Debug)]
pub enum LobbySubCommand {
    Start,
    Stop,
}

#[derive(Parser, Debug)]
#[command(long_about = None, infer_long_args = true, infer_subcommands = true)]
struct GameCommand {
    #[command(subcommand)]
    command: GameSubCommand,
}

#[derive(Parser, Debug)]
pub enum GameSubCommand {
    Start,
    Stop,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    dotenv().ok();

    let cli = Args::parse();

    match cli.command {
        StarblazersCommand::Daemon(command) => handle_daemon_command(command),
        StarblazersCommand::Game(_command) => unimplemented!(),
        StarblazersCommand::Server(command) => handle_server_command(command).await,
        StarblazersCommand::Lobby(_command) => unimplemented!(),
        StarblazersCommand::Log { n } => {
            read_last_n_lines(n).expect("Could not read last `n` lines.")
        }
        StarblazersCommand::List(list_options) => handle_list_command(list_options).await,
        StarblazersCommand::Recompile => handle_recompile_command().expect("Failed to recompile"),
    }

    Ok(())
}
async fn handle_list_command(list_options: ListOptions) {
    if list_options.websocket {
        send_message_to_daemon("list_websocket".to_string())
            .await
            .expect("failed")
    }
}
async fn handle_server_command(server_command: ServerCommand) {
    match server_command.command {
        ServerSubCommand::Helloworld => send_message_to_daemon("helloworld".to_string())
            .await
            .expect("failed"),
        ServerSubCommand::Newplayer => send_message_to_daemon("np".to_string())
            .await
            .expect("Failed"),
        ServerSubCommand::Jwt => send_message_to_daemon("jwt".to_string())
            .await
            .expect("Failed"),
        ServerSubCommand::Ws => send_message_to_daemon("ws".to_string())
            .await
            .expect("Failed"),
    }
}

fn handle_daemon_command(daemon_command: DaemonCommand) {
    match daemon_command.command {
        DaemonSubCommand::Run => start_daemon(),
        DaemonSubCommand::Kill => stop_daemon(),
        DaemonSubCommand::Restart => {
            stop_daemon();
            start_daemon()
        }
        DaemonSubCommand::Status => get_daemon_status(),
    }
}

async fn send_message_to_daemon(msg: String) -> Result<(), anyhow::Error> {
    get_local_websockt();
    let mut stream = UnixStream::connect(SOCKET_PATH).await?;
    stream.write_all(msg.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    log::info!("Response from Daemon: {}", &response);

    Ok(())
}

fn read_last_n_lines(n: usize) -> Result<(), anyhow::Error> {
    let file = File::open(STDOUT)?;

    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<String>>();

    let mut count = 0;
    for line in lines.iter().rev() {
        println!("{}", line);

        count += 1;
        if count >= n {
            break;
        }
    }

    Ok(())
}

fn handle_recompile_command() -> Result<(), Box<dyn std::error::Error>> {
    // Kill the current daemon
    stop_daemon();

    // Wait for 1 second
    thread::sleep(Duration::from_secs(1));

    // Start the new daemon
    let output = Command::new("cargo")
        .args(["run", "--bin", "sb-daemon"])
        .output()?;

    read_last_n_lines(2).ok();

    if !output.status.success() {
        log::error!(
            "Failed to start daemon: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Wait for 1 second
    thread::sleep(Duration::from_secs(1));

    // Check daemon status
    get_daemon_status();

    Ok(())
}
