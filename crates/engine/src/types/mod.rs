use crate::trade::{engine::Balances, orderbook::Orderbook};
use common::types::order::{Fill, Order};
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MessageType {
    AddOrder,
    CancelOrder,
    UpdateOrder,
    ProcessOrder,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub type_: MessageType,
    pub data: Order,
}

#[derive(Serialize, Deserialize)]
pub struct Process {
    pub message: Message,
    pub client_id: String,
}
