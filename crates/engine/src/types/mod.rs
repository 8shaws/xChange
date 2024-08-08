use crate::trade::{engine::Balances, orderbook::Orderbook};
use common::types::order::Fill;
use serde::{Deserialize, Serialize};

pub struct AddOrderResponse {
    pub executed_qty: f64,
    pub fills: Vec<Fill>,
}

#[derive(Serialize, Deserialize)]
pub struct SnapShot {
    pub orderbooks: Vec<Orderbook>,
    pub balances: Vec<Balances>,
}
