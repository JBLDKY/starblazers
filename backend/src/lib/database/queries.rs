use clap::ValueEnum;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
pub enum TableName {
    Players,
    Users,
}

impl TableName {
    pub fn create(&self) -> &str {
        match self {
            TableName::Players => CREATE_PLAYERS_TABLE,
            TableName::Users => CREATE_USERS_TABLE,
        }
    }

    pub fn reset(&self) -> &str {
        match self {
            TableName::Players => RESET_PLAYERS_TABLE,
            TableName::Users => RESET_USERS_TABLE,
        }
    }
}

pub const CREATE_PLAYERS_TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS players (
            id SERIAL PRIMARY KEY,
            uuid VARCHAR(255) NOT NULL,
            games_played INTEGER DEFAULT 0
            );
    "#;

pub const CREATE_USERS_TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) UNIQUE NOT NULL,
            username VARCHAR(255) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            creation_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            uuid VARCHAR(255) NOT NULL,
            authority VARCHAR(255) NOT NULL DEFAULT 'user'
        );
    "#;

pub const RESET_PLAYERS_TABLE: &str = "TRUNCATE players RESTART IDENTITY;";

pub const RESET_USERS_TABLE: &str = "TRUNCATE users RESTART IDENTITY;";

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableName::Players => write!(f, "Players"),
            TableName::Users => write!(f, "Users"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    name: TableName,
}

impl Table {
    pub fn create(&self) -> &str {
        self.name.create()
    }

    pub fn reset(&self) -> &str {
        self.name.reset()
    }
}

pub const AUTHENTICATE_QUERY_EMAIL: &str = r#"
SELECT * FROM players WHERE email = $1 AND password = $2;
"#;

pub const AUTHENTICATE_QUERY_USERNAME: &str = r#"
SELECT * FROM players WHERE username = $1 AND password = $2;
"#;
