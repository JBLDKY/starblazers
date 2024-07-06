#![cfg(feature = "daemon")]
use clap::Parser;
use service::daemon::basic::{get_daemon_status, start_daemon, stop_daemon};

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
}

#[derive(Parser, Debug)]
struct ServerCommand {
    #[command(subcommand)]
    command: ServerSubCommand,
}

#[derive(Parser, Debug)]
pub enum ServerSubCommand {
    Start,
    Stop,
}

#[derive(Parser, Debug)]
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
struct GameCommand {
    #[command(subcommand)]
    command: GameSubCommand,
}

#[derive(Parser, Debug)]
pub enum GameSubCommand {
    Start,
    Stop,
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let cli = Args::parse();

    match cli.command {
        StarblazersCommand::Daemon(command) => handle_daemon_command(command),
        StarblazersCommand::Game(_command) => unimplemented!(),
        StarblazersCommand::Server(_command) => unimplemented!(),
        StarblazersCommand::Lobby(_command) => unimplemented!(),
    }
}

fn handle_daemon_command(daemon_command: DaemonCommand) {
    match daemon_command.command {
        DaemonSubCommand::Run => start_daemon(),
        DaemonSubCommand::Kill => stop_daemon(),
        DaemonSubCommand::Status => get_daemon_status(),
    }
}
