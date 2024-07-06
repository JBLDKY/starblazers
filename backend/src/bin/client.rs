use std::{io::Read, process::Command};

use clap::{Parser, Subcommand};
use service::pid_file::PID_FILE;

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
        ArgCommand { sub } => handle_daemon_command(sub),
        None => println!("No command specified. Use --help for usage information."),
    }
}

fn start_daemon() {
    if is_daemon_running() {
        println!("Daemon is already running.");
        return;
    }

    let output = Command::new("cargo")
        .args(["run", "--bin", "sb-daemon", "--features", "daemon"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        log::info!("Starblazers Daemon started");
    } else {
        println!(
            "Failed to start daemon: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn stop_daemon() {
    if !is_daemon_running() {
        log::info!("Daemon is not running.");
        return;
    }

    // Read the PID from the file
    let mut file = std::fs::File::open(PID_FILE).expect("Failed to open PID file");
    let mut pid = String::new();
    file.read_to_string(&mut pid)
        .expect("Failed to read PID file");
    let pid: i32 = pid.trim().parse().expect("Failed to parse PID");

    // Send a termination signal to the daemon
    let output = Command::new("kill")
        .arg(pid.to_string())
        .output()
        .expect("Failed to execute kill command");

    if output.status.success() {
        log::info!("Sent termination signal to daemon.");
        std::fs::remove_file(PID_FILE).expect("Failed to remove PID file");
        log::info!("Daemon stopped");
    } else {
        log::info!(
            "Failed to stop daemon: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn is_daemon_running() -> bool {
    if !std::path::Path::new(PID_FILE).exists() {
        return false;
    }

    let mut file = std::fs::File::open(PID_FILE).expect("Failed to open PID file");
    let mut pid = String::new();
    file.read_to_string(&mut pid)
        .expect("Failed to read PID file");
    let pid: i32 = pid.trim().parse().expect("Failed to parse PID");

    // Check if the process with this PID exists
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn handle_daemon_command(command: DaemonCommand) {
    dbg!(command);
    println!("hi");
}
