use std::{io::Read, process::Command};

use clap::Parser;
use service::pid_file::PID_FILE;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Start,
    Stop,
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start) => start_daemon(),
        Some(Commands::Stop) => stop_daemon(),
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
