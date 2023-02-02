use std::env;
use std::fs::File;
use std::io::{Read, Write};
use log::{error, info, warn};
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub(crate) discord_token: String,
    pub(crate) discord_prefix: String,
    pub(crate) update_timeout: u64,
    pub(crate) rpc_url: String,
    pub(crate) wallet_address: String,
    pub(crate) account_configs: Vec<AccountConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountConfig {
    pub account_address: String,
    pub token_mint: String,
    pub symbol: String,
    pub dc_channel_id: u64,
    #[serde(default)]
    pub coingecko_name: String,
    pub last_signature: String,
}


pub fn get_config() -> Config {
    let mut file = File::open(env::var("CONFIG_PATH").expect("ENV: CONFIG_PATH is not present")).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str::<Config>(&*data).expect("Unable to read app_config.json file!")
}

pub fn update_config(config: Config) {
    if env::var("WRITE_CONFIG").expect("ENV: WRITE_CONFIG is not present").parse::<bool>().unwrap_or_default() {
        let mut file = File::create(env::var("CONFIG_PATH").expect("ENV: CONFIG_PATH is not present")).unwrap();
        match file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes()) {
            Ok(_) => {
                info!("Config file written");
            }
            Err(err) => {
                error!("Unable to write config file: {:}", err);
            }
        }
    } else { warn!("Write config is disabled!") }
}