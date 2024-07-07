#![cfg(feature = "daemon")]
use std::{io::Read, process::Command};

use reqwest::Client;
use uuid::Uuid;

use crate::{
    pid_file::PID_FILE,
    types::{LoginDetails, LoginError, User},
};

pub fn start_daemon() {
    if is_daemon_running() {
        log::info!("Daemon is already running.");
        return;
    }
    log::info!("Starting daemon...");

    let output = Command::new("cargo")
        .args(["run", "--bin", "sb-daemon"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        log::info!("Starblazers Daemon started");
    } else {
        log::info!(
            "Failed to start daemon: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub fn stop_daemon() {
    if !is_daemon_running() {
        log::info!("Daemon is not running.");
        return;
    }
    log::info!("Stopping daemon...");

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

pub fn is_daemon_running() -> bool {
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

pub fn get_daemon_status() {
    if is_daemon_running() {
        log::info!("Daemon status: running.");
    } else {
        log::info!("Daemon status: stopped.");
    }
}

pub async fn create_player() -> Result<(String, String), anyhow::Error> {
    let password = Uuid::new_v4().to_string();
    let email = format!("player_{}@mail.com", Uuid::new_v4());
    let new_user = User {
        email: email.clone(),
        username: format!(
            "Player_{}",
            Uuid::new_v4().to_string().split('-').next().unwrap()
        ),
        password: password.clone(), // Generate a random password
        ..Default::default()
    };

    let response = Client::new()
        .post(format!("{}auth/signup", get_local_address()))
        .json(&new_user)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(LoginError::InvalidInputSentByUser("".to_string()).into());
    }

    Ok((email, password))
}

pub async fn create_player_and_get_jwt() -> Result<String, anyhow::Error> {
    let (email, password) = create_player().await.expect("Failed to create player");

    let login_details = LoginDetails {
        email: Some(email),
        username: None,
        password,
    };

    let response = Client::new()
        .post(format!("{}auth/login", get_local_address()))
        .json(&login_details)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(LoginError::InvalidInputSentByUser("".to_string()).into());
    }

    let jwt: String = response
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    log::info!("{:?}", jwt);

    Ok(jwt)
}

pub fn get_local_address() -> String {
    std::env::var("LOCALHOST").expect("Add localhost to .env")
}

pub fn get_local_websockt() -> String {
    std::env::var("LOCALWS").expect("Add localws to .env")
}

pub async fn handle_hello_world() -> String {
    let client = Client::new();

    match client
        .get(format!("{}helloworld", get_local_address()))
        .send()
        .await
    {
        Ok(res) => format!(
            "{} - {}",
            res.status(),
            res.text().await.unwrap_or_default()
        ),
        Err(e) => e.to_string(),
    }
}
