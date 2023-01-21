mod solana;
mod config;
mod coingecko;
mod bot;
mod commands;

use std::{env, task, thread};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use dotenv::dotenv;
use log::info;
use serenity::Client;
use serenity::prelude::GatewayIntents;
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_response::Response;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use crate::bot::init_bot;


#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let config = config::config::get_config();
    let mut wallet = solana::wallet::Wallet::new(config.clone());
    wallet.fetch_solana_balance();
    wallet.fetch_solana_balance();
    wallet.fetch_token_accounts_balances();


    init_bot(config.clone(), wallet).await;
    // loop {
    //     wallet.fetch_solana_balance();
    //     wallet.fetch_token_accounts_balances();
    //     wallet.fetch_token_account_prices().await;
    //
    //     wallet.print_wallet();
    //
    //     wallet.fetch_transactions();
    //     wallet.print_transaction_queue();
    //     wallet.clear_transaction_queue();
    //
    //     thread::sleep(Duration::from_millis(config.clone().update_timeout));
    // }


    println!("--- EXIT ---");
}

