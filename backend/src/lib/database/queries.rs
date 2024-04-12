use clap::ValueEnum;
use std::fmt;
use tokio_postgres::types::ToSql;

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

#[derive(Debug, Clone, ValueEnum)]
pub enum PlayerField {
    Email,
    Username,
    Password,
    GamesPlayed,
}

impl fmt::Display for PlayerField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerField::Email => write!(f, "email"),
            PlayerField::Username => write!(f, "username"),
            PlayerField::Password => write!(f, "password"),
            PlayerField::GamesPlayed => write!(f, "gamesplayed"),
        }
    }
}

pub struct PlayerEntry {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub games_played: i32,
}

impl PlayerEntry {
    pub fn update_field(
        &self,
        field: &PlayerField,
        value: &str,
    ) -> Result<(String, Vec<Box<dyn ToSql + Sync>>), String> {
        match field {
            PlayerField::Email => self.update_email_query(value),
            PlayerField::Username => self.update_username_query(value),
            PlayerField::Password => self.update_password_query(value),
            PlayerField::GamesPlayed => value
                .parse::<i32>()
                .map_err(|_| "Invalid integer {value} for games played".to_string())
                .map(|val| self.update_games_played_query(val)),
        }
    }
    // Private methods for each specific field update
    fn update_username_query(
        &self,
        new_username: &str,
    ) -> Result<(String, Vec<Box<dyn ToSql + Sync>>), String> {
        let query = "UPDATE players SET username = $1 WHERE id = $2;".to_string();
        let params: Vec<Box<dyn ToSql + Sync>> =
            vec![Box::new(new_username.to_string()), Box::new(self.id)];
        Ok((query, params))
    }

    fn update_password_query(
        &self,
        new_password: &str,
    ) -> Result<(String, Vec<Box<dyn ToSql + Sync>>), String> {
        let query = "UPDATE players SET password = $1 WHERE id = $2;".to_string();
        let params: Vec<Box<dyn ToSql + Sync>> =
            vec![Box::new(new_password.to_string()), Box::new(self.id)];
        Ok((query, params))
    }

    fn update_email_query(
        &self,
        new_email: &str,
    ) -> Result<(String, Vec<Box<dyn ToSql + Sync>>), String> {
        let query = "UPDATE players SET email = $1 WHERE id = $2;".to_string();
        let params: Vec<Box<dyn ToSql + Sync>> =
            vec![Box::new(new_email.to_string()), Box::new(self.id)];
        Ok((query, params))
    }

    fn update_games_played_query(
        &self,
        new_games_played: i32,
    ) -> (String, Vec<Box<dyn ToSql + Sync>>) {
        let query = "UPDATE players SET games_played = $1 WHERE id = $2;".to_string();
        let params: Vec<Box<dyn ToSql + Sync>> =
            vec![Box::new(new_games_played), Box::new(self.id)];
        (query, params)
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
