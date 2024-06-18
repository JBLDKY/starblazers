#![allow(dead_code)]
use uuid::Uuid;

struct Player {
    id: Uuid,
}

struct InLobby {
    lobby_name: String,
}
struct InMenu;
struct InGame {
    game_session_id: Uuid,
}

trait PlayerState {}
impl PlayerState for InLobby {}
impl PlayerState for InMenu {}
impl PlayerState for InGame {}

trait HasPlayerId {
    fn id(&self) -> Uuid;
}

pub struct PlayerContext<S> {
    player: Player,
    state: S,
}

impl<S> HasPlayerId for PlayerContext<S> {
    fn id(&self) -> Uuid {
        self.player.id
    }
}

impl PlayerContext<InMenu> {
    fn new(player: Player) -> Self {
        PlayerContext {
            player,
            state: InMenu,
        }
    }

    fn join_lobby(self, lobby_name: String) -> PlayerContext<InLobby> {
        PlayerContext {
            player: self.player,
            state: InLobby { lobby_name },
        }
    }
}

impl PlayerContext<InLobby> {
    fn leave_lobby(self) -> PlayerContext<InMenu> {
        PlayerContext {
            player: self.player,
            state: InMenu,
        }
    }

    fn get_lobby_name(&self) -> &String {
        &self.state.lobby_name
    }
}

impl PlayerContext<InGame> {
    fn leave_game(self) -> PlayerContext<InMenu> {
        PlayerContext {
            player: self.player,
            state: InMenu,
        }
    }

    fn get_game_session_id(&self) -> Uuid {
        self.state.game_session_id
    }
}
