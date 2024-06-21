use std::collections::HashMap;

use actix::{Actor, Context};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserStateStore {
    user_id: Uuid,
    current_lobby_id: Option<Uuid>,
    current_game_id: Option<Uuid>,
}

impl UserStateStore {
    pub fn new(
        uuid: Uuid,
        current_lobby_id: Option<Uuid>,
        current_game_id: Option<Uuid>,
    ) -> UserStateStore {
        UserStateStore {
            user_id: uuid,
            current_lobby_id,
            current_game_id,
        }
    }
}

pub struct UserStateManager {
    users: HashMap<Uuid, UserStateStore>,
}

/// Make actor from `LobbyServer`
impl Actor for UserStateManager {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}
