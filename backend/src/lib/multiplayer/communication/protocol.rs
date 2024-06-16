use serde::{Deserialize, Serialize};

use crate::claims::{Claims, TokenError};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketAuthJwt {
    r#type: String,
    jwt: String,
}
impl WebsocketAuthJwt {
    pub fn claims(&self) -> Result<Claims, TokenError> {
        Claims::decode(self.jwt.split(' ').last().ok_or(TokenError::ParseError)?)
    }
}

