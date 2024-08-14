use std::collections::HashMap;

use common::types::order::{Fill, Order, OrderSide};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Orderbook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub base_asset: String,
    pub quote_asset: String,
    pub current_price: f64,
    pub symbol: String,
    pub last_trade_id: String,
}

impl Orderbook {
    pub fn new(
        bids: &Vec<Order>,
        asks: &Vec<Order>,
        base_asset: &str,
        quote_asset: &str,
        current_price: f64,
        symbol: &str,
        last_trade_id: &str,
    ) -> Orderbook {
        Orderbook {
            bids: bids.clone(),
            asks: asks.clone(),
            base_asset: base_asset.to_string(),
            quote_asset: quote_asset.to_string(),
            current_price,
            last_trade_id: last_trade_id.to_string(),
            symbol: symbol.to_string(),
        }
    }

    pub fn ticker(&self) -> String {
        format!("{}_{}", self.base_asset, self.quote_asset)
    }

    pub fn get_snapshot(&self) -> &Orderbook {
        self
    }

    pub fn add_order(&mut self, order: &mut Order) {
        match order.side {
            OrderSide::BUY => {
                self.match_bid(order);

                if order.executed_qty == order.orig_qty {
                    return;
                }
                self.bids.push(order.clone());
                return;
            }
            OrderSide::SELL => {
                self.match_asks(order);

                if order.executed_qty == order.orig_qty {
                    return;
                }
                self.asks.push(order.clone());
                return;
            }
        }
    }

    fn match_bid(&mut self, order: &mut Order) {
        order.executed_qty = 0.0;
        let mut filled_ask_order_id: Vec<u64> = vec![];
        for ask in self.asks.iter_mut() {
            if ask.price <= order.price && order.executed_qty < order.orig_qty {
                let fill_qty = ask.orig_qty.min(order.orig_qty - order.executed_qty);
                let fill = Fill {
                    price: ask.price,
                    qty: fill_qty,
                    commission: 0.0,
                    client_order_id: order.client_order_id.clone(),
                    side: order.side.clone(),
                    filled: fill_qty,
                };
                ask.executed_qty += fill_qty;
                order.fills.push(fill);
                order.executed_qty += fill_qty;

                if ask.executed_qty == ask.orig_qty {
                    filled_ask_order_id.push(ask.order_id.clone());
                }
            }
        }
        self.asks
            .retain(|ask| !filled_ask_order_id.contains(&ask.order_id.clone()));
    }

    fn match_asks(&mut self, order: &mut Order) {
        order.executed_qty = 0.0;
        let mut filled_bid_order_id: Vec<u64> = vec![];
        for bids in self.bids.iter_mut() {
            if bids.price >= order.price && order.executed_qty < order.orig_qty {
                let fill_qty = bids.orig_qty.min(order.orig_qty - order.executed_qty);
                let fill = Fill {
                    price: bids.price,
                    qty: fill_qty,
                    commission: 0.0,
                    client_order_id: order.client_order_id.clone(),
                    side: order.side.clone(),
                    filled: fill_qty,
                };
                bids.executed_qty += fill_qty;
                order.fills.push(fill);
                order.executed_qty += fill_qty;
                if bids.executed_qty == bids.orig_qty {
                    filled_bid_order_id.push(bids.order_id.clone());
                }
            }
        }

        self.bids
            .retain(|bid| !filled_bid_order_id.contains(&bid.order_id.clone()));
    }

    pub fn get_depth(&self) -> (Vec<(String, String)>, Vec<(String, String)>) {
        let mut bids_obj: HashMap<String, f64> = HashMap::new();
        let mut asks_obj: HashMap<String, f64> = HashMap::new();
        for order in &self.bids {
            let price_str = order.price.to_string();
            let entry = bids_obj.entry(price_str.clone()).or_insert(0.0);
            *entry += order.orig_qty;
        }

        for order in &self.asks {
            let price_str = order.price.to_string();
            let entry = asks_obj.entry(price_str.clone()).or_insert(0.0);
            *entry += order.orig_qty;
        }

        let bids: Vec<(String, String)> = bids_obj
            .into_iter()
            .map(|(price, quantity)| (price, quantity.to_string()))
            .collect();

        let asks: Vec<(String, String)> = asks_obj
            .into_iter()
            .map(|(price, quantity)| (price, quantity.to_string()))
            .collect();

        (bids, asks)
    }

    pub fn get_open_orders(&self, client_id: String) -> Vec<Order> {
        let mut open_orders: Vec<Order> = vec![];
        for order in self.bids.iter() {
            if order.client_id == client_id {
                open_orders.push(order.clone());
            }
        }

        for order in self.asks.iter() {
            if order.client_id == client_id {
                open_orders.push(order.clone());
            }
        }

        open_orders
    }

    pub fn cancel_bid(&mut self, order_id: u64) {
        self.bids.retain(|order| order.order_id != order_id);
    }

    pub fn cancel_ask(&mut self, order_id: u64) {
        self.asks.retain(|order| order.order_id != order_id);
    }
}
