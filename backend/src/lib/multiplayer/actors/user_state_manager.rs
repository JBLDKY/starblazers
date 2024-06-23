use std::collections::HashMap;

use crate::multiplayer::multiplayer_error::ServiceError;
use crate::multiplayer::{
    communication::{
        message::{GetState, RegisterWebSocket},
        protocol::TransitionEvent,
        user_state::UserEvent,
    },
    UserState,
};
use actix::{Actor, Context, Handler};
use uuid::Uuid;

#[derive(Default)]
pub struct UserStateManager {
    users: HashMap<Uuid, UserState>,
    sessions: HashMap<Uuid, Uuid>, // websocket connection uuid to player_id
}
impl UserStateManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
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

        // Check if the user is already registered with the websocket
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
            log::error!("OLD SESSION REGISTERED: {}", msg.connection_id);
            *state = UserState::Unauthenticated;
            return;
        };

        self.users
            .insert(msg.connection_id, UserState::Unauthenticated);

        log::info!("New session registered: {}", msg.connection_id);
        log::info!("All sessions: {:#?}", self.users);
    }
}

impl Handler<GetState> for UserStateManager {
    type Result = Option<UserState>;

    fn handle(&mut self, msg: GetState, _: &mut Context<Self>) -> Self::Result {
        self.sessions
            .get(&msg.connection_id)
            .and_then(|player_id| self.users.get(player_id))
            .cloned()
    }
}
