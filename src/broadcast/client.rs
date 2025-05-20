use crate::broadcast::{BroadcastError, BroadcastMessage};
use tokio::io::AsyncReadExt;
use tokio::net::UnixStream;

pub struct BroadcastClient<T: for<'a> BroadcastMessage<'a>> {
    socket_path: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: for<'a> BroadcastMessage<'a>> BroadcastClient<T> {
    pub fn new(socket_path: impl Into<String>) -> Self {
        Self {
            socket_path: socket_path.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn run<F>(&self, mut callback: F) -> Result<(), BroadcastError>
    where
        F: FnMut(T) -> Result<(), BroadcastError> + Send + 'static,
    {
        let mut socket = UnixStream::connect(&self.socket_path).await?;
        let mut buf = [0u8; 1024];

        loop {
            match socket.read(&mut buf).await {
                Ok(0) => {
                    println!("Connection closed by server");
                    break;
                }
                Ok(n) => {
                    match bincode::decode_from_slice::<T, _>(&buf[..n], bincode::config::standard())
                    {
                        Ok((data, _)) => {
                            if let Err(e) = callback(data) {
                                println!("Callback error: {:?}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            println!("Decode error: {:?}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("Read error: {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }
}
