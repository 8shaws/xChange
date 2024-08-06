use common::types::order::Order;

pub struct Orderbook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub base_asset: String,
    pub quote_asset: String,
    pub current_price: f64,
    pub last_trade_id: String,
}

impl Orderbook {
    pub fn new(
        bids: &Vec<Order>,
        asks: &Vec<Order>,
        base_asset: &str,
        quote_asset: &str,
        current_price: f64,
        last_trade_id: &str,
    ) -> Orderbook {
        Orderbook {
            bids: bids.clone(),
            asks: asks.clone(),
            base_asset: base_asset.to_string(),
            quote_asset: quote_asset.to_string(),
            current_price,
            last_trade_id: last_trade_id.to_string(),
        }
    }

    pub fn ticker(&self) -> String {
        format!("{}_{}", self.base_asset, self.quote_asset)
    }

    pub fn get_snapshot(&self) -> Orderbook {
        Orderbook {
            bids: self.bids.clone(),
            asks: self.asks.clone(),
            base_asset: self.base_asset.clone(),
            quote_asset: self.quote_asset.clone(),
            current_price: self.current_price,
            last_trade_id: self.last_trade_id.clone(),
        }
    }
}
