use data_ingestor::network::kraken;
// use data_ingestor::parser;


#[tokio::main]
async fn main() {
    println!(" ------ Engine Starting ------ ");
    kraken::kraken_book_data_feed()
        .await;


}