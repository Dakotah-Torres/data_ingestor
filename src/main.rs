
use helix_feed::connectors::kraken::orders;

// use data_ingestor::parser;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    println!(" ------ Engine Starting ------ ");
    orders::kraken_order_data_feed()
        .await;


}