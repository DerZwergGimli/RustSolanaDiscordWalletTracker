use std::str::FromStr;
use std::thread::sleep;
use log::{debug, error, info, warn};
use prettytable::{row, Table};
use solana_client::pubsub_client::{AccountSubscription, PubsubClient};
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcTransactionConfig};
use solana_client::rpc_response::Response;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiTransactionStatusMeta, UiTransactionTokenBalance};
use solana_transaction_status::option_serializer::OptionSerializer;
use crate::coingecko::coingecko_api::get_coingecko_price;
use crate::config::config::Config;


const SOLANA_DECIMALS: u8 = 9;

pub struct Wallet {
    rpc_url: String,
    client: RpcClient,
    wallet_address: Pubkey,
    solana_balance: u64,
    token_accounts: Vec<TokenAccount>,
    transaction_queue: Vec<TokenTransaction>,
}

#[derive(Clone)]
pub struct TokenAccount {
    pub symbol: String,
    pub address: String,
    pub ui_amount: f64,
    pub coingecko_price: f64,
    coingecko_name: String,
    pub last_signature: Option<Signature>,
}

#[derive(Clone)]
pub struct TokenTransaction {
    pub signature: String,
    pub address: String,
    pub symbol: String,
    pub ui_amount: f64,
    pub block_time: i64,
}

impl Wallet {
    pub fn new(config: Config) -> Wallet {
        info!("Installing wallet");
        info!("RPC URL is: {}", config.rpc_url);

        let mut token_accounts: Vec<TokenAccount> = vec![];
        config.account_configs.into_iter().for_each(|account| {
            token_accounts.push(TokenAccount {
                symbol: account.symbol,
                address: account.account_address,
                ui_amount: 0.0,
                coingecko_price: 0.0,
                coingecko_name: account.coingecko_name,
                last_signature: Some(Signature::from_str(&*account.last_signature).unwrap()),

            })
        });

        Wallet {
            rpc_url: config.rpc_url.clone(),
            client: RpcClient::new(config.rpc_url),
            wallet_address: Pubkey::from_str(&*config.wallet_address).unwrap(),
            solana_balance: 0,
            token_accounts,
            transaction_queue: vec![],
        }
    }

    pub fn fetch_solana_balance(&mut self) {
        match self.client.get_balance(&self.wallet_address) {
            Ok(value) => {
                self.solana_balance = value;
            }
            Err(err) => {
                error!("{:?}", err);
                error!("Unable to fetch balance!");
                self.solana_balance = 0;
            }
        }
    }

    pub fn fetch_token_accounts_balances(&mut self) {
        for (index, token_account) in self.token_accounts.clone().into_iter().enumerate() {
            match self.client.get_token_account_balance(&Pubkey::from_str(&*token_account.address).unwrap()) {
                Ok(data) => {
                    self.token_accounts[index].ui_amount = data.ui_amount.unwrap_or(0.0);
                }
                Err(err) => {
                    error!("{:?}", err);
                    error! {"Unable to fetch token account: {}", token_account.address}
                }
            }
        }
    }

    pub async fn fetch_token_account_prices(&mut self) {
        for (index, token_account) in self.token_accounts.clone().into_iter().enumerate() {
            self.token_accounts[index].coingecko_price = get_coingecko_price(token_account.coingecko_name).await;
        }
    }

    pub fn fetch_transactions(&mut self) {
        for (index, token_account) in self.token_accounts.clone().into_iter().enumerate() {
            match self.client.get_signatures_for_address_with_config(&Pubkey::from_str(&*token_account.address).unwrap(), GetConfirmedSignaturesForAddress2Config {
                before: None,
                until: token_account.last_signature,
                limit: Some(10),
                commitment: Some(CommitmentConfig::finalized()),
            }) {
                Ok(signatures) => {
                    info!("Fetched {:} signatures", signatures.len());
                    if signatures.len() > 0 {
                        self.token_accounts[index].last_signature = Some(Signature::from_str(&*signatures[0].signature).unwrap());
                    }
                    signatures.into_iter().for_each(|signature| {
                        match self.client.get_transaction_with_config(&Signature::from_str(&*signature.signature).unwrap(), RpcTransactionConfig {
                            encoding: None,
                            commitment: Some(CommitmentConfig::finalized()),
                            max_supported_transaction_version: Some(0),
                        }) {
                            Ok(transaction) => {
                                self.transaction_queue.push(TokenTransaction {
                                    signature: signature.signature.clone(),
                                    address: token_account.address.clone(),
                                    symbol: token_account.symbol.clone(),
                                    ui_amount: self.parse_balance_change(transaction),
                                    block_time: signature.block_time.unwrap(),
                                });
                            }
                            Err(err) => {
                                error!("Unable to fetch transaction {:}", err);
                            }
                        }
                    });
                }
                Err(err) => {
                    error!("Unable to fetch signatures {:}", err);
                }
            }
        }
    }

    pub fn print_transaction_queue(&self) {
        println!("Transaction Queue:");
        let mut table_info = Table::new();
        table_info.add_row(row!["RPC", "Amount"]);
        self.transaction_queue.clone().into_iter().for_each(|transaction| {
            table_info.add_row(row![transaction.signature, transaction.ui_amount]);
        });
        table_info.printstd();
    }

    pub fn table_token_accounts(&self) -> String {
        println!("Token-Balances");
        let mut table_balances = Table::new();
        table_balances.add_row(row!["Symbol", "Balance", "USD-Value"]);
        self.token_accounts.clone().into_iter().for_each(|account| {
            table_balances.add_row(row![account.symbol, format!("{:.2}",account.ui_amount), format!("{:.2}",account.ui_amount * account.coingecko_price)]);
        });
        table_balances.to_string()
    }
    pub fn get_transaction_queue(&self) -> Vec<TokenTransaction> {
        self.transaction_queue.clone()
    }
    pub fn get_token_accounts(&self) -> Vec<TokenAccount> {
        self.token_accounts.clone()
    }


    pub fn clear_transaction_queue(&mut self)
    {
        self.transaction_queue = vec![];
    }

    pub fn print_wallet(&self) {
        println!("Wallet-Overview:");
        let mut table_info = Table::new();
        table_info.add_row(row!["RPC", "Wallet", "Solana"]);
        table_info.add_row(row![self.client.url(), self.wallet_address, self.format_decimals(self.solana_balance, SOLANA_DECIMALS)]);
        table_info.printstd();


        println!("Token-Balances");
        let mut table_balances = Table::new();
        table_balances.add_row(row!["Address", "Symbol", "Balance", "TokenPrice", "USD-Value"]);
        self.token_accounts.clone().into_iter().for_each(|account| {
            table_balances.add_row(row![account.address, account.symbol, account.ui_amount, account.coingecko_price, account.ui_amount * account.coingecko_price]);
        });
        table_balances.printstd();
    }

    pub fn parse_balance_change(&self, transaction: EncodedConfirmedTransactionWithStatusMeta) -> f64 {
        let mut pre_balance = Some(0.0);
        let mut post_balance = Some(0.0);
        match transaction.transaction.meta {
            Some(metadata) => {
                match metadata.pre_token_balances {
                    OptionSerializer::Some(balances) => {
                        balances.into_iter().for_each(|balance| {
                            match balance.owner {
                                OptionSerializer::Some(data) => {
                                    if data.contains(&self.wallet_address.to_string()) {
                                        pre_balance = balance.ui_token_amount.ui_amount;
                                    };
                                }
                                OptionSerializer::None => { warn!("Found 'None' while serializing owner!"); }
                                OptionSerializer::Skip => { warn!("Found 'Skip' while serializing owner!"); }
                            }
                        })
                    }
                    _ => { warn!("While serializing tx!") }
                };
                match metadata.post_token_balances {
                    OptionSerializer::Some(balances) => {
                        balances.into_iter().for_each(|balance| {
                            match balance.owner {
                                OptionSerializer::Some(data) => {
                                    if data.contains(&self.wallet_address.to_string()) {
                                        post_balance = balance.ui_token_amount.ui_amount;
                                    };
                                }
                                OptionSerializer::None => { warn!("Found 'None' while serializing owner!"); }
                                OptionSerializer::Skip => { warn!("Found 'Skip' while serializing owner!"); }
                            }
                        })
                    }
                    _ => { warn!("While serializing tx!") }
                };
            }
            None => {
                error!("Transaction has no meta parameter!");
            }
        }
        post_balance.unwrap_or(0.0) - pre_balance.unwrap_or(0.0)
    }

    fn format_decimals(&self, number: u64, decimals: u8) -> f64 {
        number as f64 * (10i32 as f32).powi(-(decimals as i32)) as f64
    }
}