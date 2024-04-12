use clap::ValueEnum;
use std::fmt;

#[derive(Debug, ValueEnum, Clone)]
pub enum Table {
    Player,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Table::Player => write!(f, "player"),
        }
    }
}

impl Table {
    pub fn create(&self) -> &str {
        match self {
            Table::Player => CREATE_PLAYER_TABLE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            Table::Player => DROP_PLAYER_TABLE,
        }
    }
}

pub const CREATE_PLAYER_TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS players (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) UNIQUE NOT NULL,
            username VARCHAR(255) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            creation_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            games_played INTEGER DEFAULT 0
        );
    "#;

pub const DROP_PLAYER_TABLE: &str = "DROP TABLE IF EXISTS players;";
