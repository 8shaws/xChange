use super::orderbook::Orderbook;
use std::collections::HashMap;

pub struct Bs {
    pub available: f64,
    pub locked: f64,
}

type UserBalance = HashMap<String, Bs>;
type Balances = HashMap<String, UserBalance>;

pub struct Engine {
    orderbook: Orderbook,
    balances: Balances,
}
