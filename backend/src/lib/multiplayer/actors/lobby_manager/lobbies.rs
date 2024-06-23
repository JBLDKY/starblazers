use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use uuid::Uuid;

use crate::multiplayer::InvalidDataError;

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

    pub fn new_lobby(&mut self, lobby_name: String) -> Result<(), InvalidDataError> {
        if self.lobbies.contains_key(&lobby_name) {
            return Err(InvalidDataError::LobbyAlreadyExists);
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

    pub fn add_player_to(
        &mut self,
        player: Uuid,
        lobby_name: String,
    ) -> Result<(), InvalidDataError> {
        if let Some(players) = self.lobbies.get_mut(&lobby_name) {
            players.insert(player);
            Ok(())
        } else {
            Err(InvalidDataError::LobbyDoesNotExist(lobby_name))
        }
    }

    pub fn remove_player_from_lobby(
        &mut self,
        player: Uuid,
        lobby_name: String,
    ) -> Result<(), InvalidDataError> {
        self.lobbies
            .get_mut(&lobby_name)
            .ok_or_else(|| InvalidDataError::LobbyDoesNotExist(lobby_name.clone()))
            .and_then(|players| {
                if players.remove(&player) {
                    Ok(())
                } else {
                    Err(InvalidDataError::PlayerIsNotInLobby(lobby_name))
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
