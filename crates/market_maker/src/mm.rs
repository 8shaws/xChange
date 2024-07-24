use crate::api_client::ApiClient;

pub struct MarketMaker {
    api_client: ApiClient,
    spread: f64,
}

impl MarketMaker {
    pub fn new(api_client: ApiClient, spread: f64) -> Self {
        Self { api_client, spread }
    }

    pub async fn run(&self, symbol: &str) {
        println!();
        let mut last_price = 0.0;
        loop {
            // Fetch market data
            let ticker = self.api_client.get_ticker(symbol).await.unwrap();

            if ticker.last_price == last_price {
                continue;
            }

            let buy_price = ticker.last_price * (1.0 - self.spread);
            let sell_price = ticker.last_price * (1.0 + self.spread);

            println!("Last Price is at: {}", ticker.last_price);
            println!("Buy: {} Sell: {}\n", buy_price, sell_price);

            last_price = ticker.last_price;

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }
}
