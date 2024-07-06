#![cfg(feature = "daemon")]
use actix::prelude::*;
use actix_web_actors::ws;
use daemonize::Daemonize;
use reqwest::Client;
use service::pid_file::{ERROUT, PID_FILE, SOCKET_PATH, STDOUT};
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;

struct SimulatedPlayer {
    id: String,
}

impl Actor for SimulatedPlayer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(1), |act, ctx| {
            let msg = format!("{{\"type\": \"move\", \"player\": \"{}\"}}", act.id);
            ctx.text(msg);
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SimulatedPlayer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            println!("Received: {}", text);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Keep the daemon running and waiting for commands
    let listener = init_daemon().expect("Failed to init daemon");

    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await.unwrap();
            let command = std::str::from_utf8(&buffer[..n]).unwrap();

            let response = match command.trim() {
                "helloworld" => handle_hello_world().await,
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
