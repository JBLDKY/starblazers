use crate::types::UserRecord;
use actix_web::http::header::HeaderValue;
use anyhow::anyhow;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const JWT_EXPIRY: Option<chrono::TimeDelta> = chrono::TimeDelta::try_minutes(30);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub username: String,
    pub authority_level: String,
    pub uuid: String,
}

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum TokenError {
    #[error("Could not parse jwt string.")]
    ParseError,

    #[error("Invalid value, input must be implement `.to_str()`.")]
    ValueError,

    #[error("Reason: {0}")]
    Invalid(String),

    #[error("Token is expired")]
    Expired,
}

impl Claims {
    pub fn decode(jwt: &str) -> Result<Self, TokenError> {
        let secret = Self::get_jwt_secret();

        let result = jsonwebtoken::decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(&secret),
            &Validation::default(),
        );

        match result {
            Err(e) => match e.kind() {
                // We should handle this
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(TokenError::Expired),

                // We should handle this
                _ => Err(TokenError::Invalid(e.to_string())),
            },
            Ok(decoded) => Ok(decoded.claims),
        }
    }

    pub fn from_header_value(header_value: &HeaderValue) -> Result<Claims, TokenError> {
        let jwt_string = header_value
            .to_str()
            .map_err(|_| TokenError::ValueError)?
            .split(' ')
            .last()
            .ok_or(TokenError::ParseError)?
            .to_string();

        Self::decode(&jwt_string)
    }

    /// Returns a result containing a JWT or an error
    pub fn generate_jwt(user_details: UserRecord) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(JWT_EXPIRY.expect("TimeDelta is none!"))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_details.email,
            exp: expiration,
            username: user_details.username,
            authority_level: user_details.authority, // level of authorization that user has
            uuid: user_details.uuid,                 // unique uuid for this player
        };

        let secret = Self::get_jwt_secret();

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&secret),
        )
    }

    pub fn decode_jwt(jwt: &str) -> Result<TokenData<Claims>, anyhow::Error> {
        let secret = Self::get_jwt_secret();

        decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(&secret),
            &Validation::default(),
        )
        .map_err(|e| anyhow!(e))
    }

    #[inline]
    fn get_jwt_secret() -> Vec<u8> {
        let secret =
            std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable is not set");

        secret.as_bytes().to_vec()
    }
}
