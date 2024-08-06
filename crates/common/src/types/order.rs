use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    SELL,
    BUY,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    MARKET,
    LIMIT,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    NEW,
    PARTIALLY_FILLED,
    FILLED,
    CANCELED,
    PENDING_CANCEL,
    REJECTED,
    EXPIRED,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewOrder {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub type_: OrderType,
    pub time_in_force: TimeInForce,
    pub quantity: f64,
    pub quote_order_qty: f64,
    pub price: f64,
    pub stop_price: f64,
    pub timestamp: i64,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    pub transact_time: u64,
    pub price: f64,
    pub orig_qty: f64,
    pub executed_ty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub type_: OrderType,
    pub side: OrderSide,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fill {
    pub price: f64,
    pub qty: f64,
    pub commission: f64,
    pub client_order_id: String,
    pub side: OrderSide,
    pub filled: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    pub transact_time: u64,
    pub price: f64,
    pub orig_qty: f64,
    pub executed_ty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub type_: OrderType,
    pub side: OrderSide,
    pub fills: Vec<Fill>,
}
