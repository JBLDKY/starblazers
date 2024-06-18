#![allow(dead_code)]
use uuid::Uuid;

#[derive(Debug)]
pub struct Player {
    id: Uuid,
}

#[derive(Debug)]
pub struct InLobby {
    player: Player,
    lobby_name: String,
}

#[derive(Debug)]
pub struct InMenu {
    player: Player,
}

#[derive(Debug)]
pub struct InGame {
    player: Player,
    game_session_id: Uuid,
}

#[derive(Debug)]
pub struct Initializing;

pub trait PlayerState {}
impl PlayerState for Initializing {}
impl PlayerState for InLobby {}
impl PlayerState for InMenu {}
impl PlayerState for InGame {}

pub trait HasPlayer {
    fn get_player(&self) -> &Player;
}

macro_rules! impl_has_player {
    ($($state:ty),*) => {
        $(
            impl HasPlayer for $state {
                fn get_player(&self) -> &Player {
                    &self.player
                }
            }
        )*
    };
}

impl_has_player!(InMenu, InLobby, InGame);

impl<S> PlayerContext<S>
where
    S: HasPlayer,
{
    pub fn id(&self) -> Uuid {
        self.state.get_player().id
    }
}

#[derive(Debug)]
pub struct PlayerContext<S> {
    state: S,
}

impl PlayerContext<Initializing> {
    pub fn new() -> Self {
        PlayerContext {
            state: Initializing,
        }
    }

    pub fn set_player(self, player: Player) -> PlayerContext<InMenu> {
        PlayerContext {
            state: InMenu { player },
        }
    }
}

impl PlayerContext<InMenu> {
    fn new(player: Player) -> Self {
        PlayerContext {
            state: InMenu { player },
        }
    }

    fn join_lobby(self, lobby_name: String) -> PlayerContext<InLobby> {
        PlayerContext {
            state: InLobby {
                player: self.state.player,
                lobby_name,
            },
        }
    }
}

impl PlayerContext<InLobby> {
    fn leave_lobby(self) -> PlayerContext<InMenu> {
        PlayerContext {
            state: InMenu {
                player: self.state.player,
            },
        }
    }

    fn get_lobby_name(&self) -> &String {
        &self.state.lobby_name
    }
}

impl PlayerContext<InGame> {
    fn leave_game(self) -> PlayerContext<InMenu> {
        PlayerContext {
            state: InMenu {
                player: self.state.player,
            },
        }
    }

    fn get_game_session_id(&self) -> Uuid {
        self.state.game_session_id
    }
}
