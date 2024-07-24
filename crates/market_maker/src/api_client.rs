use reqwest::Client;
use serde::de::{self, Deserializer};
use serde::Deserialize;

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde(rename = "lastPrice", deserialize_with = "string_to_f64")]
    pub last_price: f64,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_ticker(&self, symbol: &str) -> Result<Ticker, reqwest::Error> {
        let url = format!("{}/ticker?symbol={}", self.base_url, symbol);
        let resp = self.client.get(&url).send().await?.json::<Ticker>().await?;
        Ok(resp)
    }
}
