use std::num::ParseFloatError;
use log::{error, warn};
use serde_json::Value;
use solana_client::client_error::reqwest;

pub async fn get_coingecko_price(cg_token_name: String) -> f64 {
    let mut text = "".to_string();
    match reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=".to_owned() + &*cg_token_name + "&vs_currencies=usd").await {
        Ok(data) => {
            match data.text()
                .await {
                Ok(data) => { text = data }
                Err(err) => { error!("body message: {:}", err) }
            }
        }
        Err(err) => { error!("connecting using GET: {:}", err) }
    };

    let json: Value = serde_json::from_str(&*text).unwrap();

    let mut value = 0.0;
    match json[cg_token_name]["usd"].to_string().parse::<f64>() {
        Ok(data) => { value = data }
        Err(err) => {
            warn!("Unable to parse CoingeckoPrice: {:}", err)
        }
    }
    value
}