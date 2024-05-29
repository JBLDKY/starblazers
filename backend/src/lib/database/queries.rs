use clap::ValueEnum;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
pub enum TableName {
    Players,
}

impl TableName {
    pub fn create(&self) -> &str {
        match self {
            TableName::Players => CREATE_PLAYERS_TABLE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            TableName::Players => DROP_PLAYERS_TABLE,
        }
    }
}

pub const CREATE_PLAYERS_TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS players (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) UNIQUE NOT NULL,
            username VARCHAR(255) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            creation_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            games_played INTEGER DEFAULT 0
        );
    "#;

pub const DROP_PLAYERS_TABLE: &str = "DROP TABLE IF EXISTS players;";

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableName::Players => write!(f, "Players"),
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

    pub fn drop(&self) -> &str {
        self.name.drop()
    }
}

pub const AUTHENTICATE_QUERY_EMAIL: &str = r#"
SELECT * FROM players WHERE email = $1 AND password = $2;
"#;

pub const AUTHENTICATE_QUERY_USERNAME: &str = r#"
SELECT * FROM players WHERE username = $1 AND password = $2;
"#;
