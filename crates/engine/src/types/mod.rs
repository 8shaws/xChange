use common::types::order::Fill;

pub struct AddOrderResponse {
    pub executed_qty: f64,
    pub fills: Vec<Fill>,
}
