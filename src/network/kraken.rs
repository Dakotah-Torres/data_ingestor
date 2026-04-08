use tokio_tungstenite:: {connect_async, Connector};
use futures_util::SinkExt;
use serde::{Serialize, Deserialize};



pub use tokio_tungstenite::tungstenite::protocol::Message;


#[derive(Serialize, Deserialize, Debug)]
pub struct  KarakenTradeReq {
    method: String,
    channel: String,
    symbol: Vec<String>,
    snapshot: bool,
    req_id: u64, 
    
}
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTradeResInner {
    ask: f64,
    ask_qty: f64,
    bid: f64,
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
#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenTradeResOutter {
    channel: String,
    #[serde(rename = "type")]
    res_type: String,
    data: Vec<KrakenTradeResInner>
    

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
pub struct KrakenOrdersReq {

}



pub async fn connect() {}