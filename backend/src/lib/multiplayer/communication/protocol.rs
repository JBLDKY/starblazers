use serde::{Deserialize, Serialize};

use crate::claims::{Claims, TokenError};

/// This file defines data structures specifically used for sending and
/// receiving data over WebSocket connections. These structures are
/// serialized and deserialized for communication between the client
/// and server in real-time.

/// Structure representing a WebSocket authentication message containing a JWT
#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketAuthJwt {
    /// Type of the WebSocket message
    r#type: String,
    /// JSON Web Token (JWT) for authentication
    jwt: String,
}

impl WebsocketAuthJwt {
    /// Decodes the JWT to extract claims
    ///
    /// # Returns
    ///
    /// A result containing the decoded `Claims` if successful,
    /// or a `TokenError` if decoding fails.
    pub fn claims(&self) -> Result<Claims, TokenError> {
        let token = Claims::extract_token(&self.jwt)?;
        Claims::decode(token)
    }
}
