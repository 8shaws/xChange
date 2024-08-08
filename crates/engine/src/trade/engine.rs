use serde::{Deserialize, Serialize};
use std::env;

use super::orderbook::Orderbook;
use crate::utils::load_snapshot;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bs {
    pub available: f64,
    pub locked: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBalance {
    user_id: String,
    assets: HashMap<String, Bs>,
}
pub type Balances = Vec<Vec<UserBalance>>;

pub struct Engine {
    orderbooks: Vec<Orderbook>,
    balances: Vec<Balances>,
}

// TODO: save the snapshot after an interval and set the user balance
impl Engine {
    pub fn new() -> Engine {
        if let Ok(path) = env::var("SNAPSHOT_PATH") {
            let snapshot = load_snapshot(path.as_str());
            match snapshot {
                Ok(snapshot) => Engine {
                    orderbooks: snapshot.orderbooks,
                    balances: snapshot.balances,
                },
                Err(e) => {
                    println!("Error loading snapshot: {}", e);
                    Engine {
                        orderbooks: vec![Orderbook::new(&vec![], &vec![], "BTC", "USDT", 0.0, "0")],
                        balances: vec![Balances::new()],
                    }
                }
            }
        } else {
            println!("SNAPSHOT_PATH not set, using default snapshot");
            Engine {
                orderbooks: vec![Orderbook::new(&vec![], &vec![], "BTC", "USDT", 0.0, "0")],
                balances: vec![Balances::new()],
            }
        }
    }
}
