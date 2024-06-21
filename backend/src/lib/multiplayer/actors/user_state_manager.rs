use std::collections::HashMap;

use actix::{Actor, Context, Handler};
use uuid::Uuid;

use crate::multiplayer::{communication::message::RegisterWebSocket, UserState};

pub struct UserStateManager {
    users: HashMap<Uuid, UserState>,
}
impl UserStateManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

impl Default for UserStateManager {
    fn default() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

/// Make actor from `LobbyServer`
impl Actor for UserStateManager {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<RegisterWebSocket> for UserStateManager {
    type Result = ();

    fn handle(&mut self, msg: RegisterWebSocket, _: &mut Context<Self>) {
        self.users
            .insert(msg.connection_id, UserState::Unauthenticated);
    }
}
