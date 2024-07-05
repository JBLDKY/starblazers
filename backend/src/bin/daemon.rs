#![cfg(feature = "daemon")]
use daemonize::Daemonize;
use service::pid_file::{ERROUT, PID_FILE, STDOUT};
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let stdout = File::create(STDOUT).unwrap();
    let stderr = File::create(ERROUT).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(PID_FILE) // Every method except `new` and `start`
        .chown_pid_file(false) // Don't change ownership of the pid file
        .working_directory("/tmp") // for default behaviour.
        .umask(0o022) // File is readable
        .stdout(stdout.try_clone().unwrap()) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr.try_clone().unwrap()); // Redirect stderr to `/tmp/daemon.err`.

    match daemonize.start() {
        Ok(_) => loop {
            let mut stdout = stdout.try_clone().unwrap();
            let _stderr = stderr.try_clone().unwrap();

            if let Ok(metadata) = std::fs::metadata(PID_FILE) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o644); // User read/write, group/others read
                let _ = std::fs::set_permissions(PID_FILE, perms);
            }

            std::thread::sleep(std::time::Duration::from_secs(5));
            writeln!(stdout, "Daemon is still alive").unwrap();
            stdout.flush().unwrap();
        },
        Err(e) => eprintln!("Error starting daemon: {}", e),
    }
}
