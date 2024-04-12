mod commands; // Include the commands module
pub use commands::{Cli, Commands, DatabaseCommands, TestCommands};

use crate::database::DatabaseClient;
use clap::Parser;

use crate::database::queries::{PlayerEntry, PlayerField, Table};

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

                DatabaseCommands::UpdateRecord {
                    table,
                    record_id,
                    field,
                    value,
                } => {} // match table {
                        // Table::Player => match field {
                        //     PlayerField::Email => {
                        //         match db.update_player_field(&player_entry, &PlayerField::Email, value)
                        //         {
                        //             Ok(_) => log::info!(
                        //                 "Successfully updated email for record ID {}.",
                        //                 record_id
                        //             ),
                        //             Err(e) => log::error!("Failed to update email: {e}."),
                        //         }
                        //     }
                        //     PlayerField::Username => {}
                        //     PlayerField::Password => {}
                        //     PlayerField::GamesPlayed => {}
                        // },
                        // },
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
