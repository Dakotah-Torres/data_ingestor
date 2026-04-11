// use futures_util::StreamExt;
use serde::{Serialize, Deserialize};

// use tokio_tungstenite::tungstenite::protocol::Message;



// use super::{KRAKEN_AUTH_URL, CHANNEL_ORDERS_L3, kraken_trade_connect};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(into ="u32")]
pub enum OrderDepth {
    Ten = 10,
    OneHundred = 100,
    OneThousand = 1000,
}

impl From<OrderDepth> for u32 {
    fn from(depth: OrderDepth) -> u32 {
        match depth {
            OrderDepth::Ten => 10,
            OrderDepth::OneHundred => 100,
            OrderDepth::OneThousand => 1000,
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenOrdersReqInnerParams {
    channel: String,
    symbol: Vec<String>, 
    depth: OrderDepth,
    snapshot: bool,
    token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenOrdersReqOuter {
    method: String,
    params: KrakenOrdersReqInnerParams,
    req_id: i32,

}


#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenOrderBidAsk<'a> {
    order_id: &'a str,
    limit_price: f64,
    order_qty: f64,
    timestamp: &'a str
}



#[derive(Serialize, Deserialize, Debug)]
pub struct KrakenOrderResObject<'a> {
    symbol: &'a str,
    bids: Vec<KrakenOrderBidAsk<'a>>,
    asks: Vec<KrakenOrderBidAsk<'a>>,
    checksum:  i64,
    timestamp: &'a str

}

