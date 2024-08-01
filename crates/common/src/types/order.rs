use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    SELL,
    BUY,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    MARKET,
    LIMIT,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub type_: OrderType,
    pub time_in_force: TimeInForce,
    pub quantity: f64,
    pub quote_order_qty: f64,
    pub price: f64,
    pub stop_price: f64,
    pub timestamp: i64,
}
