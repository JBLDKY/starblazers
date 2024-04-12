mod commands; // Include the commands module
use crate::cli::commands::Executable;

pub use commands::{Cli, Commands, DatabaseCommands, TestCommands};

use crate::database::DatabaseClient;
use clap::Parser;

pub fn handle_cli_input() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Test => {
            test_something_new();
        }
        Commands::Db { action } => {
            let mut db = DatabaseClient::new().expect("Failed to initialize database client.");

            match action {
                DatabaseCommands::Drop { table } => match db.reset_table(table) {
                    Ok(v) => log::info!("Succesfully dropped `{table:?}`. Result: {v:?}."),
                    Err(e) => log::error!("Failed to drop `{table:?}` because of {e}."),
                },
                DatabaseCommands::Wipe => match db.wipe() {
                    Ok(_) => log::info!("Succesfully wiped the database."),
                    Err(e) => {
                        log::error!("Failed to wipe database because of the following error: {e}.")
                    }
                },
                DatabaseCommands::CreateTable { name } => match db.create_table(name) {
                    Ok(_) => log::info!("Succesfully created table: `{name:?}`."),
                    Err(e) => log::error!(
                        "Failed to create table: `{name:?}` because of the following error: {e}."
                    ),
                },
                DatabaseCommands::CreatePlayer(player) => {
                    player.execute(&mut db);
                }
            }
        }
        Commands::Server { message } => {
            log::info!("Sending broadcast message: {}", message);
            // broadcast a message somehow
        }
    }
}

fn test_something_new() {
    log::info!("Testing...");
}
