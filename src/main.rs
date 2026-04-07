use data_ingestor::network;
use data_ingestor::parser;
use data_ingestor::models;
use futures_util::StreamExt;


#[tokio::main]
async fn main() {
    println!(" ------ Engine Starting ------ ");

    let url = "wss://stream.binance.us:9443/ws/btcusdt@trade";
    let stream = network::connect_to_binance_stream(url).await;

    println!("Connected to Binance stream. Listening for data...");

    let (_write, mut read) = stream.split();
    while let Some(message) = read.next().await {
        if let Ok(msg) = message {
            if let Ok(text) = msg.to_text() {
                let price_opt = parser::extract_price(text);
                let quant_opt = parser::extract_quantity(text);

                if let (Some(price_opt), Some(quant_opt)) = (price_opt, quant_opt) {
                    if let Some(trade) = models::parse_trade(price_opt, quant_opt) {
                        println!("Received Trade - Price: {}, Quantity: {}", trade.price, trade.quant);
                    }
                }

            }
        }
    }
}