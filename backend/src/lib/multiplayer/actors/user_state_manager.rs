use std::collections::HashMap;

use crate::multiplayer::communication::common::JoinLobbyRequest;
use crate::multiplayer::communication::message::{
    CheckExistingConnection, DeleteState, SetState, UpdateState,
};
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

/// Maps websocket connection UUIDs to player UUIDs and UserStates
#[derive(Default)]
pub struct UserStateManager {
    states: HashMap<Uuid, UserState>, // ws connection uuid -> user state
}

impl UserStateManager {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
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
            for (cid, user_state) in &self.states {
                match user_state {
                    UserState::Authenticated { player_id }
                    | UserState::InLobby { player_id, .. }
                    | UserState::InGame { player_id, .. } => {
                        if &user_id == player_id {
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
        }

        if let Some(state) = self.states.get_mut(&event.connection_id) {
            state.transition(event.event);
        } else {
            log::warn!("Connection ID not found: {}", event.connection_id);
        }

        log::info!("User transitioned: {}", event.connection_id);
        log::info!("All users: {:#?}", self.states);
    }
}

impl Handler<RegisterWebSocket> for UserStateManager {
    type Result = ();

    fn handle(&mut self, msg: RegisterWebSocket, _: &mut Context<Self>) {
        self.states.insert(
            msg.connection_id,
            UserState::Authenticated {
                player_id: msg.user_id,
            },
        );

        log::info!("New session registered: {}", msg.connection_id);
        log::info!("All sessions: {:#?}", self.states);
    }
}

impl Handler<GetState> for UserStateManager {
    type Result = Option<UserState>;

    fn handle(&mut self, msg: GetState, _: &mut Context<Self>) -> Self::Result {
        self.states.get(&msg.connection_id).cloned()
    }
}

impl Handler<CheckExistingConnection> for UserStateManager {
    type Result = bool;
    fn handle(&mut self, msg: CheckExistingConnection, _: &mut Context<Self>) -> Self::Result {
        for state in self.states.values() {
            let player_uuid = match state {
                UserState::Authenticated { player_id } => player_id,
                UserState::InGame { player_id, .. } => player_id,
                UserState::InLobby { player_id, .. } => player_id,
            };
            if *player_uuid == msg.user_id {
                return true;
            }
        }
        false
    }
}

impl Handler<SetState> for UserStateManager {
    type Result = Result<(), ServiceError>;

    fn handle(&mut self, msg: SetState, _: &mut Context<Self>) -> Self::Result {
        let old_state = self.states.insert(msg.connection_id, msg.state.clone());

        if let Some(s) = old_state {
            log::info!(
                "State update for connection: `{}`, old: `{}`, new: `{}`",
                msg.connection_id,
                s,
                msg.state.clone()
            );
        } else {
            log::info!(
                "New state added for connection: `{}`, new: `{}`",
                msg.connection_id,
                msg.state.clone()
            );
        }

        Ok(())
    }
}

impl Handler<JoinLobbyRequest> for UserStateManager {
    type Result = ();
    fn handle(&mut self, req: JoinLobbyRequest, _ctx: &mut Self::Context) {
        // TODO:  Handle errors
        let player_id =
            uuid::Uuid::parse_str(&req.player_id).unwrap_or_else(|_| return Default::default());

        let lobby_id =
            uuid::Uuid::parse_str(&req.lobby_name).unwrap_or_else(|_| return Default::default());

        if let Some(state) = self.states.get_mut(&player_id) {
            state.transition(UserEvent::JoinLobby(lobby_id))
        }
    }
}
