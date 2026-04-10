use tokio_tungstenite:: {connect_async, Connector, MaybeTlsStream, WebsocketStream};
use tokio::net::TcpStream;
use futures_util::SinkExt;
use serde::{Serialize, Deserialize};
use url::Url;


const KRAKEN_PUB_URL: &str = "wss://ws.kraken.com/v2";
const KRAKEN_AUTH_URL: &str = "wss://ws.kraken.com/v2";
const CHANNEL_TICKER_L1: &str = "ticker";
const CHANNEL_BOOK_L2: &str = "book";
const CHANNEL_ORDERS_L3: &str = "level3";
const CHANNEL_TRADES: &str = "trade";

pub type KrakenStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub use tokio_tungstenite::tungstenite::protocol::Message;
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTickerReqInner {
    channel: String, 
    symbol: Vec<String>,
    snapshot: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct  KrakenTickerReqOutter {
    method: String,
    params: KrakenTickerReqInner,
    req_id: u64, 
    
}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTickerResInner<'a> {
    ask: f64,
    ask_qty: f64,
    bid: f64,
    bid_qyt: f64,
    change: f64,
    change_pct:f64,
    high: f64,
    last: f64,
    low:f64,
    symbol: &'a str,
    timestamp: &'a str,
    volume: f64,
    vwap: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTickerResOutter<'a> {
    channel: &'a str,
    #[serde(rename = "type")]
    res_type: &'a str,
    data: Vec<KrakenTickerResInner<'a>>
    

} 

#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenBookReq {

} 
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenBookRes {

}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenOrdersReq {

}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenOrdersRes {

}



pub async fn kraken_trade_connect(connection_request: KrakenTickerReqOutter ) -> KrakenStream {
    let url = Url::parse(KRAKEN_PUB_URL).expect("Invalid URL");
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
