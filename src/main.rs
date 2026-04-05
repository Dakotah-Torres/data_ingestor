use tokio_tungstenite::connect_async;
use url::Url;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let socket_url = "wss://stream.binance.us:9443/ws/btcusdt@trade";
    let url = Url::parse(socket_url).expect("Invalid URL");

    // This is the 'Handshake'
    let (ws_stream, _) = match connect_async(url.to_string()).await {
        Ok(stream) => stream, 
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    };
    println!("Connected to Binance Firehose!");

    let (_write, mut read) = ws_stream.split();

    // The 'while let' loop stays open as long as Binance keeps sending data
    while let Some(message) = read.next().await {
        println!("--- Heartbeat: Packet Received ---");
        match message {
            Ok(msg) => {
                let text = msg.to_text().unwrap_or("Empty Message");
                println!("RAW DATA: {}", text);
            }
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }
}