#![cfg(feature = "daemon")]
use clap::Parser;
use service::{
    daemon::basic::{get_daemon_status, start_daemon, stop_daemon},
    pid_file::SOCKET_PATH,
};
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
}

#[derive(Parser, Debug)]
#[command(long_about = None, infer_long_args = true, infer_subcommands = true)]
struct DaemonCommand {
    #[command(subcommand)]
    command: DaemonSubCommand,
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
    List,
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

    let cli = Args::parse();

    match cli.command {
        StarblazersCommand::Daemon(command) => handle_daemon_command(command),
        StarblazersCommand::Game(_command) => unimplemented!(),
        StarblazersCommand::Server(command) => handle_server_command(command).await,
        StarblazersCommand::Lobby(_command) => unimplemented!(),
    }

    Ok(())
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
        ServerSubCommand::List => send_message_to_daemon("list".to_string())
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
    let mut stream = UnixStream::connect(SOCKET_PATH).await?;
    stream.write_all(msg.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    log::info!("Response from Daemon: {}", &response);

    Ok(())
}
