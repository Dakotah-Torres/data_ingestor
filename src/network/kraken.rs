use tokio_tungstenite:: {connect_async, MaybeTlsStream, WebSocketStream};
use tokio::net::TcpStream;
use futures_util:: {SinkExt, StreamExt};
use futures_util::stream::SplitStream;
use serde::{Serialize, Deserialize};
use url::Url;


pub const KRAKEN_PUB_URL: &str = "wss://ws.kraken.com/v2";
pub const KRAKEN_AUTH_URL: &str = "wss://ws.kraken.com/v2";
pub const CHANNEL_BOOK_L2: &str = "book";
pub const CHANNEL_TICKER_L1: &str = "ticker";
pub const CHANNEL_ORDERS_L3: &str = "level3";
pub const CHANNEL_TRADES: &str = "trade";

pub type KrakenStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type KrakenReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub use tokio_tungstenite::tungstenite::protocol::Message;
#[derive(Serialize, Deserialize, Debug)]

pub enum BookDepth {
    #[serde(rename = "10")]
    Ten, 
    #[serde(rename = "25")]
    TwintyFive,
    #[serde(rename = "100")]
    OneHundred,
    #[serde(rename = "500")]
    FiveHundred, 
    #[serde(rename = "1000")]
    OneThoudand

}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTickerReqInner {
    pub channel: String, 
    pub symbol: Vec<String>,
    pub snapshot: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct  KrakenTickerReqOuter {
    pub method: String,
    pub params: KrakenTickerReqInner,
    pub req_id: u64, 
    
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
pub struct KrakenTickerResOuter<'a> {
    pub channel: &'a str,
    #[serde(rename = "type")]
    pub res_type: &'a str,
    pub data: Vec<KrakenTickerResInner<'a>>
    

} 

#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenBookReqInner {
    channel: String,
    symbol: Vec<String>,
    depth: BookDepth,
    snapshot: bool,
} 
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenBookReqOuter {
    method: String,
    params: KrakenBookReqInner,
    req_id: u64
    
}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenBookBidAsk {
    price: f64,
    qty: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenBookObject<'a> {
    asks: Vec<KrakenBookBidAsk>,
    bids: Vec<KrakenBookBidAsk>,
    checksum: i64,
    symbol: &'a str,
    timestamp: &'a str 
}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenBookResOuter <'a>{
    channel: &'a str, 
    #[serde(rename="type")]
    res_type: &'a str,
    data: Vec<KrakenBookObject<'a>>
}








#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenOrdersReq {

}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenOrdersRes {

}




pub async fn kraken_trade_connect(connection_request: KrakenTickerReqOuter ) -> KrakenReadStream {
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

pub async fn kraken_ticker_data(){
    println!(" ------ Engine Starting ------ ");
    let inner = KrakenTickerReqInner {
        channel: CHANNEL_TICKER_L1.to_string(),
        symbol: vec!["BTC/USD".to_string()],
        snapshot: false,
    };
    let outer = KrakenTickerReqOuter {
        method: "subscribe".to_string(),
        params: inner,
        req_id: 231,
    };

    let mut stream = kraken_trade_connect(outer)
            .await;

    while let Some(message) = stream.next().await {
        if let Ok(Message::Text(msg)) = message {
            println!("{}", msg);
        }
    }
}
