mod solana;
mod config;
mod coingecko;
mod bot;
mod commands;

use std::{task, thread};
use dotenv::dotenv;
use crate::bot::init_bot;


#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let config = config::config::get_config();
    let wallet = solana::wallet::Wallet::new(config.clone());

    init_bot(config.clone(), wallet).await;

    println!("--- EXIT ---");
}

