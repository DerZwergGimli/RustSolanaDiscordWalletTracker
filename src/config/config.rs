use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub(crate) update_timeout: u64,
    pub(crate) rpc_url: String,
    pub(crate) wallet_address: String,
    pub(crate) account_configs: Vec<AccountConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AccountConfig {
    pub account_address: String,
    pub symbol: String,
    pub dc_channel_id: String,
    #[serde(default)]
    pub coingecko_name: String,
}


pub fn get_config() -> Config {
    let mut file = File::open("app_config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str::<Config>(&*data).expect("Unable to read app_config.json file!")
}