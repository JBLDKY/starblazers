use actix::prelude::*;
use serde::{Deserialize, Serialize};

/// This file defines common data structures used in the Actix actor-based
/// communication system of the application. These structures are designed
/// to be shared across different modules, facilitating synchronization
/// and communication between actors. Additionally, these data structures
/// are serialized and sent across the WebSocket in literal form for
/// real-time updates.

/// Structure representing the game state, used for synchronization
#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GameState {
    /// Type of the game state update
    r#type: String,
    /// X position of the player in the game
    position_x: usize,
    /// Y position of the player in the game
    position_y: usize,
    /// Unique identifier for the player
    player_id: String,
    /// Timestamp of the game state update
    timestamp: String,
}

impl GameState {
    /// Returns a reference to the player ID
    pub fn player_id(&self) -> &str {
        &self.player_id
    }

    /// Returns a cloned version of the player ID as a String
    pub fn into_player_id(&self) -> String {
        self.player_id.clone()
    }
}
