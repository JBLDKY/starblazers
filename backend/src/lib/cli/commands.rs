use crate::database::db::DatabaseClient;
use crate::database::queries::Table;
use clap::{Parser, Subcommand};

/// CLI for the Starblazers client
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs test utilities
    Test,
    /// Database operations
    Db {
        #[clap(subcommand)]
        action: DatabaseCommands,
    },
    /// Server operations
    Server {
        /// Send a broadcast message
        message: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum TestCommands {
    /// Run a specific test utility function
    Run,
}

#[derive(Subcommand, Debug)]
pub enum DatabaseCommands {
    /// Drops a table
    Drop {
        /// The name of the table to drop
        #[clap(value_enum)]
        table: Table,
    },
    /// Wipe the database
    Wipe,
    /// Creates a new table
    CreateTable {
        /// The name of the new table
        #[clap(value_enum)]
        name: Table,
    },
    CreatePlayer(CreatePlayer),
}

pub trait Executable {
    fn execute(&self, db: &mut DatabaseClient);
    fn query(&self) -> &str;
}

#[derive(Parser, Debug)]
pub struct CreatePlayer {
    #[clap(long)]
    email: String,
    #[clap(long)]
    username: String,
    #[clap(long)]
    password: String,
    #[clap(long, default_value_t = 0)]
    games_played: i32,
}

impl Executable for CreatePlayer {
    fn execute(&self, db: &mut DatabaseClient) {
        match db.execute_query(
            self.query(),
            &[
                &self.email,
                &self.username,
                &self.password,
                &self.games_played,
            ],
        ) {
            Ok(v) => log::info!("Succesfully created a new record: {:?}", v),
            Err(e) => log::error!(
                "Could not create record because of the following error: {}",
                e
            ),
        };
    }
    fn query(&self) -> &str {
        r#"
    INSERT INTO players (email, username, password, games_played)
    VALUES ($1, $2, $3, $4)
    RETURNING id;
"#
    }
}
