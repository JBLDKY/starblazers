use std::fmt;

use actix_web::{http::StatusCode, ResponseError};
use serde::{Deserialize, Serialize};

use thiserror::Error;

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

/// A login method enum constructed with the actual value for email or username
#[derive(Serialize, Deserialize, Debug)]
pub enum LoginMethod {
    Email(String),
    Username(String),
}

impl fmt::Display for LoginMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginMethod::Email(_) => write!(f, "email"),
            LoginMethod::Username(_) => write!(f, "username"),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUserRecord {
    pub email: String,
    pub username: String,
    pub uuid: String,
    pub authority: String,
}

impl From<UserRecord> for PublicUserRecord {
    fn from(user_record: UserRecord) -> PublicUserRecord {
        PublicUserRecord {
            email: user_record.email,
            username: user_record.username,
            uuid: user_record.uuid,
            authority: user_record.authority,
        }
    }
}

impl From<sqlx::Error> for SignupError {
    fn from(err: sqlx::Error) -> Self {
        SignupError::SqlError(err.to_string())
    }
}

impl ResponseError for SignupError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            SignupError::InvalidPassword
            | SignupError::InvalidUsername
            | SignupError::InvalidEmail => StatusCode::UNPROCESSABLE_ENTITY,
            SignupError::UsernameUnavailable => StatusCode::CONFLICT,
            SignupError::Catchall(_)
            | SignupError::SqlError(_)
            | SignupError::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SignupError {
    #[error("Username already in use")]
    UsernameUnavailable,

    #[error("Invalid email")]
    InvalidEmail,

    #[error("Username cannot be empty")]
    InvalidUsername,

    #[error("Password cannot be empty")]
    InvalidPassword,

    #[error("Failed to hash the password: {0}")]
    PasswordHashingError(String),

    #[error("Encountered error: {0}")]
    Catchall(String),

    #[error("User input does not match expect format because: {0}")]
    SqlError(String),
}

impl From<sqlx::Error> for LoginError {
    fn from(err: sqlx::Error) -> Self {
        LoginError::SqlError(err.to_string())
    }
}

// error_response returns json text with the below error text
impl ResponseError for LoginError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            LoginError::InvalidPassword
            | LoginError::InvalidInputSentByUser(_)
            | LoginError::UserDoesntExist
            | LoginError::MissingCredentials => StatusCode::UNAUTHORIZED,
            LoginError::Unhandled
            | LoginError::Catchall(_)
            | LoginError::SqlError(_)
            | LoginError::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Error, Debug, Serialize, Deserialize)]
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

    #[error("User input does not match expect format because: {0}")]
    InvalidInputSentByUser(String),

    #[error("User input does not match expect format because: {0}")]
    SqlError(String),
}
