use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio::net::TcpStream;
use url::Url;
use futures_util:: {SinkExt, StreamExt};
use futures_util::stream::SplitStream;
use serde::Serialize;
use std::time:: {SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
pub use tokio_tungstenite::tungstenite::protocol::Message;


pub mod kraken_ticker;
pub mod kraken_orders;
pub mod kraken_book;

pub type KrakenStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type KrakenReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub const KRAKEN_PUB_URL: &str = "wss://ws.kraken.com/v2";
pub const KRAKEN_AUTH_URL: &str = "wss://ws-l3.kraken.com/v2";
pub const CHANNEL_BOOK_L2: &str = "book";
pub const CHANNEL_TICKER_L1: &str = "ticker";
pub const CHANNEL_ORDERS_L3: &str = "level3";
pub const CHANNEL_TRADES: &str = "trade";


pub async fn kraken_trade_connect<T: Serialize>(connection_request: T, _url:&str) -> KrakenReadStream {

    let url = Url::parse(_url).expect("Invalid URL");
    
    let(ws_stream, _) = connect_async(url.to_string())
        .await
        .expect("Failed to Connect");

    let (mut write, read) = ws_stream.split();
    let conn_req_json = serde_json::to_string(&connection_request)
        .expect("Failed to serialize request");

    write.send(Message::Text(conn_req_json))
        .await
        .expect("Unable to Connect");

    //returning the read stream
    read

}

// pub async fn get_kraken_ws_token()-> Result<String, anyhow::Error> {
//     let nonce = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards")
//         .as_millis()
//         .to_string();
//     let mut params = HashMap::new();
//     params.insert("nonce", nonce.as_str());
//     let api_key = std::env::var("KRAKEN_API_KEY")?;
//     let api_secret = std::env::var("KRAKEN_API_SECRET")?;


//     let post_data = serde_urlencoded::to_string(&params)?;

//     let encoded = format!("{}{}", nonce, post_data);
//     let sha256_hash = Sha256::digest(encoded.as_bytes());
// }