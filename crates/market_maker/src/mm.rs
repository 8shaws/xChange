use crate::api_client::{ApiClient, OrderSide};

pub struct MarketMaker {
    api_client: ApiClient,
    base_spread: f64,
    quantity: f64,
    volatility_factor: f64,
    account_balance: f64,
    risk_percentage: f64,
}

impl MarketMaker {
    pub fn new(
        api_client: ApiClient,
        base_spread: f64,
        quantity: f64,
        volatility_factor: f64,
        account_balance: f64,
        risk_percentage: f64,
    ) -> Self {
        Self {
            api_client,
            base_spread,
            quantity,
            volatility_factor,
            account_balance,
            risk_percentage,
        }
    }

    pub async fn run(&self, symbol: &str) {
        println!();
        let mut last_price = 0.0;
        loop {
            let ticker = match self.api_client.get_ticker(symbol).await {
                Ok(ticker) => ticker,
                Err(e) => {
                    eprintln!("Error fetching the ticker: {}", e);
                    continue;
                }
            };

            if ticker.last_price == last_price {
                continue;
            }

            let volatility = self
                .calculate_volatility(&ticker.symbol)
                .await
                .unwrap_or(0.01);

            let adjusted_spread = self.base_spread * (1.0 + volatility * self.volatility_factor);

            let buy_price = ticker.last_price * (1.0 - adjusted_spread);
            let sell_price = ticker.last_price * (1.0 + adjusted_spread);

            let position_size = self.calculate_position_size(
                self.account_balance,
                self.risk_percentage,
                volatility,
            );

            // Place buy order
            match self
                .api_client
                .place_order("BTCUSDT", OrderSide::BUY, position_size, buy_price)
                .await
            {
                Ok(_) => println!("Buying at: {}", buy_price),
                Err(e) => eprintln!("Error placing buy order: {}", e),
            }

            // Place sell order
            match self
                .api_client
                .place_order("BTCUSDT", OrderSide::SELL, position_size, sell_price)
                .await
            {
                Ok(_) => println!("Selling at: {}\n", sell_price),
                Err(e) => eprintln!("Error placing sell order: {}\n", e),
            }

            last_price = ticker.last_price;

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    async fn fetch_historical_data(&self, symbol: &str) -> Result<Vec<f64>, reqwest::Error> {
        Ok(vec![50000.0, 51000.0, 49500.0, 50500.0, 52000.0]) // Example data
    }

    async fn calculate_volatility(&self, _symbol: &str) -> Option<f64> {
        Some(0.01) // Example fixed volatility value
    }

    fn calculate_position_size(
        &self,
        account_balance: f64,
        risk_percentage: f64,
        volatility: f64,
    ) -> f64 {
        (account_balance * risk_percentage) / volatility
    }
}
