use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Player id is not a valid uuid: {0}")]
    PlayerIdIsNotUuid(String),

    #[error("Lobby does not exist: {0}")]
    LobbyDoesNotExist(String),

    #[error("Lobby already exists.")]
    LobbyAlreadyExists,

    #[error("Player is not in this lobby.")]
    PlayerIsNotInLobby(String),
}
