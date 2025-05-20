use bincode::{Decode, Encode};
use mbus::broadcast::{BroadcastClient, BroadcastMessage, BroadcastServer};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
struct MarketData {
    symbol: String,
    price: f64,
    timestamp: u128,
}

#[tokio::main]
async fn main() {
    // 启动服务器
    let server = BroadcastServer::<MarketData>::new("/tmp/market.sock", 100);
    let sender = server.sender();

    // 启动数据生成任务
    tokio::spawn(async move {
        let mut price = 42000.0;
        loop {
            let data = MarketData {
                symbol: "BTC/USD".into(),
                price,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_micros(),
            };

            let _ = sender.send(data);
            price += (rand::random::<f64>() - 0.5) * 100.0;
            tokio::time::sleep(Duration::from_micros(100)).await;
        }
    });

    // 启动客户端
    let client = BroadcastClient::<MarketData>::new("/tmp/market.sock");

    // 运行服务器和客户端
    let (server_result, client_result) = tokio::join!(
        server.run(),
        client.run(|data| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros();
            println!("[{}] Received: {:?}", now - data.timestamp, data);
            Ok(())
        })
    );

    if let Err(e) = server_result {
        eprintln!("Server error: {:?}", e);
    }
    if let Err(e) = client_result {
        eprintln!("Client error: {:?}", e);
    }
}
