use data_ingestor::network;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    println!(" ------ Engine Starting ------ ");

    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";
    let stream = network::connect_to_binance_stream(url).await;

    println!("Connected to Binance stream. Listening for data...");

    let (_write, mut read) = stream.split();
    while let Some(message) = read.next().await {
        if let Ok(msg) = message {
            // We will move the 'Slicer' logic into parser/mod.rs next
            println!("Received packet: {}", msg.to_text().unwrap_or(""));
        }
    }
}