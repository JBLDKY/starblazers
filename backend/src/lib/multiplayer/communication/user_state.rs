use uuid::Uuid;

#[derive(Debug)]
pub enum UserState {
    Unauthenticated,
    Authenticated { player_id: Uuid },
    InLobby { player_id: Uuid, lobby_id: Uuid },
    InGame { player_id: Uuid, game_id: Uuid },
}

#[derive(Debug)]
pub enum UserEvent {
    Login(Uuid),
    Logout,
    JoinLobby(Uuid), // lobbyId
    StartGame(Uuid), // game_id
    Exit,            // go to a menu
}

impl UserState {
    pub fn transition(&mut self, event: UserEvent) {
        match (&self, event) {
            // Initial handshake with websocket
            (UserState::Unauthenticated, UserEvent::Login(player_id)) => {
                *self = UserState::Authenticated { player_id }
            }

            // Join a lobby after authenticating
            (UserState::Authenticated { player_id }, UserEvent::JoinLobby(lobby_id)) => {
                *self = UserState::InLobby {
                    player_id: *player_id,
                    lobby_id,
                }
            }

            // Leave a lobby while remaining authenticated
            (UserState::InLobby { player_id, .. }, UserEvent::Exit) => {
                *self = UserState::Authenticated {
                    player_id: *player_id,
                }
            }

            // Join a game session after authenticating (bypassing lobby)
            (UserState::Authenticated { player_id }, UserEvent::StartGame(game_id)) => {
                *self = UserState::InGame {
                    player_id: *player_id,
                    game_id,
                }
            }

            // Leave a game session while remaining authenticated
            (UserState::InGame { player_id, .. }, UserEvent::Exit) => {
                *self = UserState::Authenticated {
                    player_id: *player_id,
                }
            }

            (state, _) => {
                log::error!(
                    "Transition not possible or not handled from state: {:?}",
                    state
                );
            }
        }
    }

    pub fn user_id(&self) -> Option<Uuid> {
        match self {
            UserState::Unauthenticated => None,
            UserState::Authenticated { player_id } => Some(*player_id),
            UserState::InLobby { player_id, .. } => Some(*player_id),
            UserState::InGame { player_id, .. } => Some(*player_id),
        }
    }
}
