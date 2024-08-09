use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use tokio::time::{self, Duration};

use super::orderbook::Orderbook;
use crate::{types::SnapShot, utils::load_snapshot};
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Engine {
    orderbooks: Vec<Orderbook>,
    balances: Vec<Balances>,
}

impl Engine {
    pub async fn new() -> Engine {
        let mut engine = if let Ok(path) = env::var("SNAPSHOT_PATH") {
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
        };

        let mut engine_clone = engine.clone();
        tokio::spawn(async move {
            engine_clone.start_saving_snapshots().await;
        });

        engine
    }

    async fn save_snapshot(&self) {
        let snap = SnapShot {
            orderbooks: self.orderbooks.clone(),
            balances: self.balances.clone(),
        };

        if let Ok(path) = env::var("SNAPSHOT_PATH") {
            let snapshot = serde_json::to_string(&snap).unwrap();
            fs::write(path, snapshot).unwrap();
        } else {
            println!("SNAPSHOT_PATH not set, snapshot not saved");
        }
    }

    async fn start_saving_snapshots(&mut self) {
        let mut interval = time::interval(Duration::from_secs(3));
        loop {
            interval.tick().await;
            self.save_snapshot().await;
        }
    }
}
