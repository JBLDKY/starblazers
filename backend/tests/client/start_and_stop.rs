use assert_cmd::Command;
use predicates::prelude::*;
use std::process::Command as StdCommand;
use std::thread;
use std::time::Duration;

fn is_daemon_running() -> bool {
    // This is a placeholder. You'll need to implement a way to check if the daemon is running.
    // For now, we'll just check if a file exists (you might use a PID file in practice).
    std::path::Path::new("/tmp/test.pid").exists()
}

// #[test]
// fn test_start_daemon() {
//     let mut cmd = Command::cargo_bin("daemon").unwrap();
//     let assert = cmd.arg("start").assert();
//     assert
//         .success()
//         .stdout(predicate::str::contains("Daemon started"));
//
//     thread::sleep(Duration::from_secs(1)); // Give the daemon time to start
//
//     assert!(
//         is_daemon_running(),
//         "Daemon should be running after start command"
//     );
//
//     // Cleanup: stop the daemon
//     let _ = StdCommand::new("cargo")
//         .args(["run", "--bin", "daemon", "stop"])
//         .output();
// }

#[test]
fn test_stop_daemon() {
    // First, start the daemon
    let _ = StdCommand::new("cargo")
        .args(["run", "--bin", "daemon"])
        .output();

    thread::sleep(Duration::from_secs(1)); // Give the daemon time to start

    // Now test stopping it
    let mut cmd = Command::cargo_bin("client").unwrap();
    let assert = cmd.arg("stop").assert();
    assert
        .success()
        .stdout(predicate::str::contains("Daemon stopped"));

    thread::sleep(Duration::from_secs(1)); // Give the daemon time to stop

    assert!(
        !is_daemon_running(),
        "Daemon should not be running after stop command"
    );
}
