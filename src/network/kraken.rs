use tokio_tungstenite:: {connect_async, Connector};
use futures_util::SinkExt;



pub use tokio_tungstenite::tungstenite::protocol::Message;



pub struct  KarakenTradeReq {
    method: String,
    channel: String,
    symbol: Vec<String>,
    snapshot: bool,
    req_id: u64, 
}

pub struct KrakenTradeResInner {
    ask: f64,
    ask_qty: f64,
    bit: f64,
    bid_qyt: f64,
    change: f64,
    change_pct:f64,
    high: f64,
    last: f64,
    low:f64,
    symbol: String,
    timestamp: String,
    volume: f64,
    vwap: f64,
}

pub struct KrakenTradeResOutter {
    channel: String,
    res_type: String,
    

} 

pub struct KrakenBookReq {

} 

pub struct KrakenBookRes {

}

pub struct KrakenOrdersReq {

}

pub struct KrakenOrdersReq {

}



pub async fn connect() {}