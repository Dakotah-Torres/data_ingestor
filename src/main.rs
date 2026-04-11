
use helix_feed::network::kraken_orders;

// use data_ingestor::parser;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    println!(" ------ Engine Starting ------ ");
    kraken_orders::kraken_order_data_feed()
        .await;


}