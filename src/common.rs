use serde::{Serialize, Deserialize};

use std::net::{SocketAddr};

#[derive(Serialize, Deserialize)]
pub enum Message {
    RegisterClient(String, SocketAddr),
    UnregisterClient(String),

    DataTransferMessage(String, String), // name and message
}