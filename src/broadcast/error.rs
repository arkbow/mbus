use thiserror::Error;

#[derive(Error, Debug)]
pub enum BroadcastError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Bincode error: {0}")]
    Bincode(#[from] bincode::error::EncodeError),

    #[error("Socket already exists")]
    SocketExists,

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Broadcast error: {0}")]
    Broadcast(String),
}
