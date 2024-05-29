use serde::{Deserialize, Serialize};

use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug)]
pub struct DatabaseError(pub sqlx::Error);

impl Reject for DatabaseError {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Player {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub creation_date: Option<chrono::NaiveDateTime>,
    pub games_played: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginDetails {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordRecord {
    pub password: String,
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
