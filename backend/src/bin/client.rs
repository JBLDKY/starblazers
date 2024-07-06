#![cfg(feature = "daemon")]
use clap::Parser;
use service::daemon::basic::{get_daemon_status, start_daemon, stop_daemon};

#[derive(Parser, Debug)]
#[command(author = "Sb Co.", version = "0.1.0", about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: StarblazersCommand,
}

#[derive(Parser, Debug)]
enum StarblazersCommand {
    #[command()]
    Daemon(DaemonCommand),
    #[command()]
    Server(ServerCommand),
    #[command()]
    Lobby(LobbyCommand),
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
    Start,
    Stop,
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
        DaemonSubCommand::Start => start_daemon(),
        DaemonSubCommand::Stop => stop_daemon(),
        DaemonSubCommand::Status => get_daemon_status(),
    }
}
