use mbus::broadcast::BroadcastClient;
use mbus::model::Position;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启动客户端
    let client = BroadcastClient::<Position>::new("/tmp/market.sock");

    // 运行客户端并持续接收消息
    tokio::spawn(async move {
        if let Err(e) = client.run(|pos| async move {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros();
            println!("[{}] Position Update:", now);
            println!("  Pair: {}/{}", pos.symbol_x, pos.symbol_y);
            println!("  Current Price: {}", pos.current_price);
            println!("  Active Bin ID: {}", pos.active_bin_id);
            println!("  Total {}: {}", pos.symbol_x, pos.total_x_amount());
            println!("  Total {}: {}", pos.symbol_y, pos.total_y_amount());
            println!("  Total Fee {}: {}", pos.symbol_x, pos.total_fee_x_amount());
            println!("  Total Fee {}: {}", pos.symbol_y, pos.total_fee_y_amount());
            println!("  Number of Bins: {}", pos.bins.len());
            println!("----------------------------------------");
            Ok(())
        }).await {
            eprintln!("Client error: {}", e);
        }
    });

    // 保持主线程运行
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
