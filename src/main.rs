mod solana;
mod config;
mod coingecko;

use std::{env, task, thread};
use std::str::FromStr;
use std::time::Duration;
use dotenv::dotenv;
use log::info;
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_response::Response;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;


#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let config = config::config::get_config();

    let rpc_url = env::var("RPCURL").expect("RPURL not set!");
    let wss_url = env::var("WSSURL").expect("WSSURL not set!");
    let wallet_address = env::var("WALLETADDRESS").expect("WALLETADDRESS not set!");


    let mut wallet = solana::wallet::Wallet::new(config.clone());

    loop {
        wallet.fetch_solana_balance();
        wallet.fetch_token_accounts_balances();
        wallet.fetch_token_account_prices().await;

        wallet.print_wallet();
        wallet.fetch_transactions();

        wallet.fetch_transactions();
        thread::sleep(Duration::from_millis(config.clone().update_timeout));
    }


    println!("--- EXIT ---");
}

