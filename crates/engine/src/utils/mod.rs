use crate::types::SnapShot;
use reqwest::blocking::Client;
use std::error::Error;
use std::fs::File;

pub fn load_snapshot(source: &str) -> Result<SnapShot, Box<dyn Error>> {
    if source.starts_with("http") || source.starts_with("https") {
        let client = Client::new();
        let response = client.get(source).send()?;
        let data: SnapShot = response.json()?;
        Ok(data)
    } else {
        let file = File::open(source)?;
        let data: SnapShot = serde_json::from_reader(file)?;
        Ok(data)
    }
}
