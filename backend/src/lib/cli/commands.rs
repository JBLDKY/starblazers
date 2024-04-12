use crate::database::queries::{PlayerField, Table};
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
    /// Updates a record
    UpdateRecord {
        /// Table of the recod
        #[clap(value_enum)]
        table: Table,
        /// ID
        record_id: i32,
        /// Field to update
        #[clap(value_enum)]
        field: PlayerField,
        /// New value for the field
        value: String,
    },
}
