use thiserror::Error;
use uuid::Uuid;

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

    #[error("The connection with ID `{0}` is not currently registered.")]
    ConnectionNotRegistered(Uuid),

    #[error("The connection with ID `{0}` Does not have a registered state. This error is critical; each connection must have a state.")]
    StateNotRegistered(Uuid),
}
