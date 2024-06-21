use std::collections::HashMap;

use actix::{Actor, Context, Handler};
use uuid::Uuid;

use crate::{
    multiplayer::{
        communication::{
            message::RegisterWebSocket, protocol::TransitionEvent, user_state::UserEvent,
        },
        UserState,
    },
    types::User,
};

#[derive(Default)]
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

/// Make actor from `LobbyServer`
impl Actor for UserStateManager {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<TransitionEvent> for UserStateManager {
    type Result = ();

    fn handle(&mut self, event: TransitionEvent, _: &mut Context<Self>) {
        let mut connection_id = Uuid::default();
        let mut already_connected = false;

        if let UserEvent::Login(user_id) = event.event {
            for (cid, user_state) in &self.users {
                match user_state {
                    UserState::Unauthenticated => (),
                    UserState::Authenticated { player_id }
                    | UserState::InLobby { player_id, .. }
                    | UserState::InGame { player_id, .. } => {
                        if user_id == *player_id {
                            connection_id = *cid;
                            already_connected = true;
                            break;
                        }
                    }
                }
            }

            if already_connected {
                log::warn!(
                    "Player {} is already connected with connection ID: {}",
                    user_id,
                    connection_id
                );
                return;
            }

            log::info!("User login event: {}", user_id);
        }

        if let Some(state) = self.users.get_mut(&event.connection_id) {
            state.transition(event.event);
        } else {
            log::warn!("Connection ID not found: {}", event.connection_id);
        }

        log::info!("User transitioned: {}", event.connection_id);
        log::info!("All sessions: {:#?}", self.users);
    }
}

impl Handler<RegisterWebSocket> for UserStateManager {
    type Result = ();

    fn handle(&mut self, msg: RegisterWebSocket, _: &mut Context<Self>) {
        // TODO: This should probably fail if the user is already connected, or rather
        // the old connection should be restored instead
        if let Some(state) = self.users.get_mut(&msg.connection_id) {
            log::info!("OLD SESSION REGISTERED: {}", msg.connection_id);
            *state = UserState::Unauthenticated;
            return;
        };

        self.users
            .insert(msg.connection_id, UserState::Unauthenticated);

        log::info!("New session registered: {}", msg.connection_id);
        log::info!("All sessions: {:#?}", self.users);
    }
}
