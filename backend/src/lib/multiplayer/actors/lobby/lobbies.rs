use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use uuid::Uuid;

use crate::multiplayer::ServiceError;

#[derive(Debug)]
pub struct Lobbies {
    lobbies: HashMap<String, HashSet<Uuid>>, // HashSet with Player IDs
    player_to_lobby: HashMap<Uuid, String>,
}

impl Lobbies {
    pub fn get(&self) -> &HashMap<String, HashSet<Uuid>> {
        &self.lobbies
    }

    pub fn new() -> Self {
        Self {
            lobbies: HashMap::new(),
            player_to_lobby: HashMap::new(),
        }
    }

    pub fn new_lobby(&mut self, lobby_name: String) -> Result<(), ServiceError> {
        if self.lobbies.contains_key(&lobby_name) {
            return Err(ServiceError::LobbyAlreadyExists);
        }

        self.lobbies.insert(lobby_name, HashSet::new());

        Ok(())
    }

    pub fn empty_lobbies(&self) -> Vec<String> {
        self.lobbies
            .iter()
            .filter_map(|(key, value)| {
                if value.is_empty() {
                    Some(key.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove_empty(&mut self) {
        for key in self.empty_lobbies() {
            log::info!("Removing empty lobby: {}", &key);
            self.lobbies.remove(&key);
        }
    }

    pub fn add_player_to(&mut self, player: Uuid, lobby_name: String) -> Result<(), ServiceError> {
        if let Some(players) = self.lobbies.get_mut(&lobby_name) {
            players.insert(player);
            self.player_to_lobby.insert(player, lobby_name);
            Ok(())
        } else {
            Err(ServiceError::LobbyDoesNotExist(lobby_name))
        }
    }

    pub fn remove_player_from_lobby(
        &mut self,
        player: Uuid,
        lobby_name: String,
    ) -> Result<(), ServiceError> {
        self.lobbies
            .get_mut(&lobby_name)
            .ok_or_else(|| ServiceError::LobbyDoesNotExist(lobby_name.clone()))
            .and_then(|players| {
                if players.remove(&player) {
                    self.player_to_lobby.remove(&player);
                    Ok(())
                } else {
                    Err(ServiceError::PlayerIsNotInLobby(lobby_name))
                }
            })
    }
}

impl fmt::Display for Lobbies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lobbies: {:?}\nPlayer-To-Lobby: {:?}",
            self.lobbies, self.player_to_lobby
        )
    }
}
