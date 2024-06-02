use std::fmt;

use serde::{Deserialize, Serialize};

use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug)]
pub struct DatabaseError(pub sqlx::Error);

impl Reject for DatabaseError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub username: String,
    pub authority_level: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub creation_date: Option<chrono::NaiveDateTime>,
    pub uuid: Option<String>,
    pub authority: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Player {
    pub id: Option<i32>,
    pub uuid: String,
    pub games_played: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginDetails {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LoginMethod {
    Email,
    Username,
}

impl fmt::Display for LoginMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginMethod::Email => write!(f, "email"),
            LoginMethod::Username => write!(f, "username"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRecord {
    pub email: String,
    pub password: String,
    pub username: String,
    pub uuid: String,
    pub authority: String,
}

#[derive(Error, Debug)]
pub enum SignupError {
    #[error("Username already in use")]
    UsernameUnavailable,

    #[error("Invalid email")]
    InvalidEmail,
}

impl Reject for LoginError {}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("No username or email provided")]
    MissingCredentials,

    #[error("User does not exist")]
    UserDoesntExist,

    #[error("The password entered was incorrect")]
    InvalidPassword,

    #[error("Failed to hash the password: {0}")]
    PasswordHashingError(String),

    #[error("Encountered error: {0}")]
    Catchall(String),

    #[error("Unhandled error occurred")]
    Unhandled,

    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
}
