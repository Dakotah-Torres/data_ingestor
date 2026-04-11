
use data_ingestor::network::kraken_book;
// use data_ingestor::parser;


#[tokio::main]
async fn main() {
    println!(" ------ Engine Starting ------ ");
    kraken_book::kraken_book_data_feed()
        .await;


}