use crate::claims::Claims;
use actix::prelude::*;

/// This file defines various messages and data structures used in the
/// Actix actor-based communication system of the application. These
/// messages facilitate interactions between different components,
/// such as chat sessions, game state synchronization, and client-server
/// communication in a multiplayer environment. Each message is tailored
/// to be used within the Actix actor framework, ensuring type-safe
/// and asynchronous communication.

/// A simple message structure wrapping a String
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message sent when a new chat session is created
#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    /// Address of the recipient actor for this session
    pub addr: Recipient<Message>,
    /// Claims containing user-specific data (e.g., authentication information)
    pub claims: Claims,
}

/// Message sent when a session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    /// Unique identifier for the disconnected session
    pub id: String,
}

/// Message sent to a specific room containing a client's message
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Unique identifier for the client session
    pub id: usize,
    /// The message content sent by the peer
    pub msg: String,
    /// The name of the room the message is sent to
    pub lobby: String,
}

/// Message to join a room; creates the room if it doesn't exist
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Unique identifier for the client session
    pub id: usize,
    /// The name of the room to join
    pub name: String,
}
