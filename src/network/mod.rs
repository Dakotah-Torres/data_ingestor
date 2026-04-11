use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio::net::TcpStream;
use url::Url;

pub mod kraken;

pub type BinanceStream = WebSocketStream<MaybeTlsStream<TcpStream>>;


pub async fn connect_to_binance_stream(url_str: &str) -> BinanceStream {
    let url = Url::parse(url_str).expect("Invalid URL");
    let (ws_stream, _) = connect_async(url.to_string())
        .await
        .expect("Failed to connect");
        ws_stream
}



