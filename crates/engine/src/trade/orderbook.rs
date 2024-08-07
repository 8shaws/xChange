use common::types::order::{Fill, Order, OrderSide};

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
}
