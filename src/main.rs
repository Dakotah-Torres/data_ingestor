use data_ingestor::network;
use data_ingestor::parser;
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
            if let Some(text) = msg.to_text().ok(){
                if let (Some(price), Some(quant)) = (parser::extract_price(text), parser::extract_quantity(text)){
                    println!("Price: {} | Qt {}", price, quant );
                }
                
            }
        }
    }
}