use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidDataError {
    #[error("Player id is not a valid uuid: {0}")]
    PlayerIdIsNotUuid(String),
}
