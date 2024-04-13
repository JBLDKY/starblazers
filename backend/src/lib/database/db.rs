use anyhow::Result;
use std::sync::Arc;

use crate::{
    database::queries::Table,
    types::{LoginDetails, LoginError, PasswordRecord, Player},
};
use sqlx::postgres::PgPool;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub type ArcDb = Arc<DatabaseClient>;

#[derive(Clone)]
pub struct DatabaseClient {
    pub pool: PgPool,
}

/// Verify the password using its salt
fn verify_password(hashed: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hashed)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Returns the password's (salted) hash
fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

impl DatabaseClient {
    /// Constructor
    pub async fn new() -> DatabaseClient {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to create pool");

        DatabaseClient { pool }
    }

    /// Method to test if we receive anything at all from the database
    ///
    /// Logs a warning if the database did not respond.
    pub async fn test(&self) {
        let row: Result<(i32,), sqlx::Error> =
            sqlx::query_as("SELECT 1").fetch_one(&self.pool).await;

        if row.is_err() {
            log::warn!("Error establishing connection to the database.");
        }
    }

    /// Resets a provided table
    pub async fn reset_table(
        &self,
        table: &Table,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let sql = table.drop();
        sqlx::query(sql).execute(&self.pool).await
    }

    /// Create a new player with the provided parameters
    /// TODO: validate email
    pub async fn create_player(
        &self,
        player: &Player,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let creation_date = if let Some(creation_date) = player.creation_date {
            creation_date // Use the provided value if provided
        } else {
            chrono::Utc::now().naive_utc() // Default is the current time
        };

        let games_played = player.games_played.unwrap_or_default();

        let hashed_password = hash_password(&player.password).map_err(|e| {
            log::error!("Failed to hash password: {}", e);

            // convert error so the return type
            // doesnt need changing
            sqlx::Error::Configuration(e.to_string().into())
        })?;

        sqlx::query("INSERT INTO players (email, username, password, creation_date, games_played) VALUES ($1, $2, $3, $4, $5)")
            .bind(&player.email)
            .bind(&player.username)
            .bind(&hashed_password)
            .bind(creation_date)
            .bind(games_played)
            .execute(&self.pool)
            .await
    }

    /// Run the authentication process.
    ///
    /// Login is possible with either username or email.
    ///
    /// Returns a boolean indicating success.
    pub async fn check_login_details(
        &self,
        login_details: &LoginDetails,
    ) -> Result<bool, LoginError> {
        if login_details.username.is_none() && login_details.email.is_none() {
            log::error!("No username or email was provided.");
            return Err(LoginError::MissingCredentials);
        }

        let hashed_password = if let Some(email) = login_details.email.as_deref() {
            self.get_password_for_email(email).await
        } else {
            let username = login_details.username.as_deref().unwrap_or_default();
            self.get_password_for_username(username).await
        };

        if hashed_password.is_err() {
            log::error!("User does not exist");
            return Err(LoginError::UserDoesntExist);
        };

        let is_valid = verify_password(&hashed_password.unwrap().password, &login_details.password);

        if is_valid.is_err() {
            log::error!("Failed to verify password.");
            return Err(LoginError::PasswordHashingError(
                "Failed to verify password.".to_string(),
            ));
        }

        Ok(is_valid.unwrap())
    }

    /// Searches the database for a password linked to the provided email
    async fn get_password_for_email(&self, email: &str) -> Result<PasswordRecord, LoginError> {
        let password = sqlx::query_as!(
            PasswordRecord,
            "SELECT password FROM players WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| LoginError::UserDoesntExist)?;

        log::info!("Found password: `{:?}`", &password);

        Ok(password)
    }

    /// Searches the database for a password linked to the provided username
    async fn get_password_for_username(
        &self,
        username: &str,
    ) -> Result<PasswordRecord, LoginError> {
        let password = sqlx::query_as!(
            PasswordRecord,
            "SELECT password FROM players WHERE username = $1",
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| LoginError::UserDoesntExist)?;

        Ok(password)
    }
}
