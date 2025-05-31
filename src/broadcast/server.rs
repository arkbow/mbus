use crate::broadcast::{BroadcastError, BroadcastMessage};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixListener;
use tokio::sync::broadcast;
use log::{debug, error};

pub struct BroadcastServer<T: for<'a> BroadcastMessage<'a>> {
    socket_path: String,
    tx: broadcast::Sender<T>,
}

impl<T: for<'a> BroadcastMessage<'a>> BroadcastServer<T> {
    pub fn new(socket_path: impl Into<String>, channel_size: usize) -> Self {
        let (tx, _) = broadcast::channel(channel_size);
        Self {
            socket_path: socket_path.into(),
            tx,
        }
    }

    pub fn sender(&self) -> broadcast::Sender<T> {
        self.tx.clone()
    }

    pub async fn run(&self) -> Result<(), BroadcastError> {
        // 清理可能存在的旧socket文件
        if Path::new(&self.socket_path).exists() {
            std::fs::remove_file(&self.socket_path)?;
        }

        // 启动服务器
        let listener = UnixListener::bind(&self.socket_path)?;
        
        // 设置socket文件权限，允许所有用户读写
        let mut perms = std::fs::metadata(&self.socket_path)?.permissions();
        perms.set_mode(0o666);
        std::fs::set_permissions(&self.socket_path, perms)?;
        
        debug!("Broadcast server started at {}", self.socket_path);

        loop {
            let (mut socket, _) = listener.accept().await?;
            let mut rx = self.tx.subscribe();

            // 为每个连接创建一个新的任务
            tokio::spawn(async move {
                debug!("New client connected");
                while let Ok(data) = rx.recv().await {
                    let bytes = bincode::encode_to_vec(&data, bincode::config::standard()).unwrap();
                    if let Err(e) = socket.write_all(&bytes).await {
                        error!("Error sending data: {:?}", e);
                        break;
                    }
                }
            });
        }
    }
}
