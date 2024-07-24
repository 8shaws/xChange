mod api_client;
mod mm;

use api_client::ApiClient;
use mm::MarketMaker;

#[tokio::main]
async fn main() {
    let api_client = ApiClient::new("https://www.binance.com/api/v3");
    let market_maker = MarketMaker::new(api_client, 0.001);

    market_maker.run("BTCUSDC").await;
}
