use anyhow::Result;
use email_address::EmailAddress;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    claims::Claims,
    database::queries::Table,
    types::{LoginDetails, LoginError, LoginMethod, SignupError, User, UserRecord},
};
use sqlx::{postgres::PgPool, Postgres, Transaction};

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

    /// Create a new user with the provided parameters
    pub async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        log::info!("Creating new account....");
        let creation_date = if let Some(creation_date) = user.creation_date {
            creation_date // Use the provided value if provided
        } else {
            chrono::Utc::now().naive_utc() // Default is the current time
        };

        if !EmailAddress::is_valid(&user.email) {
            return Err(SignupError::InvalidEmail)
                .map_err(|e| sqlx::Error::Configuration(e.to_string().into()));
        }

        let games_played = 0;

        let authority = "user";

        let hashed_password = hash_password(&user.password).map_err(|e| {
            log::error!("Failed to hash password: {}", e);

            // convert error so the return type
            // doesnt need changing
            sqlx::Error::Configuration(e.to_string().into())
        })?;

        // generate UUID for player identification
        let uuid = Uuid::new_v4();

        // Transaction started so that both queries execute successfully for db commits to be
        // created
        let mut transaction: Transaction<'_, Postgres> = self.pool.begin().await?;

        // entry for client
        sqlx::query("INSERT INTO users (email, username, password, creation_date, uuid, authority) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&user.email)
            .bind(&user.username)
            .bind(&hashed_password)
            .bind(creation_date)
            .bind(uuid.to_string())
            .bind(authority)
            .execute(&mut *transaction)
            .await?;
        // entry for player specific information
        sqlx::query("INSERT INTO players (uuid, games_played) VALUES ($1, $2)")
            .bind(uuid.to_string())
            .bind(games_played)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?; // finish successful transacation or error
        Ok(())
    }

    /// Run the authentication process.
    ///
    /// Login is possible with either username or email.
    ///
    /// Returns a boolean indicating success.
    pub async fn check_login_details(
        &self,
        login_details: &LoginDetails,
    ) -> Result<String, LoginError> {
        // Determine if username or password was used
        // Right now the Ok below is unused, only really checking the error
        let login_method = match (&login_details.email, &login_details.username) {
            (Some(_email), _) => Ok(LoginMethod::Email),
            (None, Some(_username)) => Ok(LoginMethod::Username),
            (None, None) => Err(LoginError::MissingCredentials),
        }?;

        // Get user data
        let user_details = self
            .get_details_by_login_method(&login_method, login_details)
            .await?;

        // Verify the password using Argon2
        let is_valid = verify_password(&user_details.password, &login_details.password);

        // Not sure when this would be the case
        if is_valid.is_err() {
            log::error!("Failed to verify password.");
            return Err(LoginError::PasswordHashingError(
                "Failed to verify password.".to_string(),
            ));
        }

        let success = is_valid.unwrap(); // safe

        // If we get a `False` it means the entered password
        // does not match the found password
        if !success {
            return Err(LoginError::InvalidPassword);
        }

        // Since we early return in the case of a wrong password,
        // we should create a JWT cuz the password seems valid
        let jwt = Claims::generate_jwt(user_details);

        // Convert the error to a LoginError
        jwt.map_err(|e| LoginError::Catchall(e.to_string()))
    }

    /// Searches the database for a password matching the provided login method (email or username) , returns all detail
    async fn get_details_by_login_method(
        &self,
        login_method: &LoginMethod,
        login_details: &LoginDetails,
    ) -> Result<UserRecord, LoginError> {
        let user_data = match login_method {
            // obtain data using email
            LoginMethod::Email => sqlx::query_as!(
                UserRecord,
                "SELECT email, username, password, uuid, authority FROM users WHERE email = $1",
                login_details.email,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|_| LoginError::UserDoesntExist)?,
            // obtain data using username
            LoginMethod::Username => sqlx::query_as!(
                UserRecord,
                "SELECT email, username, password, uuid, authority FROM users WHERE username = $1",
                login_details.username,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|_| LoginError::UserDoesntExist)?,
        };

        Ok(user_data)
    }
}
