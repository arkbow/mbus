mod client;
mod error;
mod server;

pub use client::BroadcastClient;
pub use error::BroadcastError;
pub use server::BroadcastServer;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// 可广播的数据类型必须实现这些 trait
pub trait BroadcastMessage<'a>:
    Serialize + Deserialize<'a> + Encode + Decode<()> + Clone + Send + Sync + 'static
{
}

/// 为所有实现了必要 trait 的类型自动实现 BroadcastMessage
impl<'a, T> BroadcastMessage<'a> for T where
    T: Serialize + Deserialize<'a> + Encode + Decode<()> + Clone + Send + Sync + 'static
{
}
