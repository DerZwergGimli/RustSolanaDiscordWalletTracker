use std::borrow::BorrowMut;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::collections::HashSet;
use std::sync::Arc;
use chrono::{DateTime, format, NaiveDateTime};
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::prelude::*;
use chrono::offset::Utc;
use log::{error, info, warn};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::utils::Color;

use crate::commands::ping::*;
use crate::commands::help::*;
use crate::commands::wallet::*;
use crate::commands::config::*;
use crate::commands::store::*;
use crate::commands::address::*;

use crate::config::config::{AccountConfig, Config};
use crate::solana::wallet::Wallet;

pub struct WalletStore;

impl TypeMapKey for WalletStore {
    type Value = Arc<Mutex<Wallet>>;
}

pub struct ConfigStore;

impl TypeMapKey for ConfigStore {
    type Value = Arc<Mutex<Config>>;
}

pub struct Handler {
    pub(crate) is_loop_running: AtomicBool,
}

#[group]
#[commands(ping, help, wallet, config, store, address)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        let ctx = Arc::new(ctx);


        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);

            // Update-Wallet Task
            tokio::spawn(async move {
                loop {
                    update_wallet(Arc::clone(&ctx1)).await;
                    let data_read = ctx1.data.read().await;
                    let arc_config = data_read.get::<ConfigStore>().expect("Expected ConfigStore in TypeMap");
                    let config = arc_config.lock().await.clone();

                    update_nickname(Arc::clone(&ctx1), _guilds.clone()).await;
                    tokio::time::sleep(Duration::from_millis(config.update_timeout)).await;
                }
            });

            //Check TX Queue Task
            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    check_tx_queue(Arc::clone(&ctx2)).await;
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn update_wallet(ctx: Arc<Context>) {
    let mut data_read = ctx.data.read().await;
    let mut wallet = data_read.get::<WalletStore>().unwrap();
    wallet.lock().await.fetch_solana_balance();
    wallet.lock().await.fetch_token_accounts_balances();
    wallet.lock().await.fetch_token_account_prices().await;
    wallet.lock().await.fetch_transactions();
    info!("wallet-updated!");
}

async fn check_tx_queue(ctx: Arc<Context>) {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigStore>().expect("Expected ConfigStore in TypeMap");

    let config = arc_config.lock().await.clone();

    let mut arc_wallet = data_read.get::<WalletStore>().expect("Expected WalletStore in TypeMap");

    let queue = arc_wallet.lock().await.get_transaction_queue();
    warn!("Len {:}", queue.len());
    for (index, transaction) in queue.into_iter().enumerate() {
        let direction_emote = if transaction.ui_amount >= 0.0 { ":inbox_tray:" } else { ":outbox_tray:" };
        let info_message = format!("{:} {:.2} {:}", direction_emote, transaction.ui_amount, transaction.symbol);
        let channel_id = match config.account_configs.clone().into_iter().find(|account| {
            account.symbol == transaction.symbol
        }) {
            None => { 0 }
            Some(account) => { account.dc_channel_id }
        };

        let message = ChannelId(channel_id).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(":information_source: SPL-Transaction :information_source:")
                    .color(Color::ORANGE)
                    .field(info_message, "", false)
                    .field("Timestamp", format!("{}", DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(transaction.block_time, 1000000).unwrap(), Utc)), false)
                    .field("Signature", transaction.signature.clone(), false)
                    .field("Link", "https://solscan.io/tx/".to_owned() + &*transaction.signature, false)
            })
        }).await;
    }
    arc_wallet.lock().await.clear_transaction_queue();


    info!("checked-queue!");
}

async fn update_nickname(ctx: Arc<Context>, _guilds: Vec<GuildId>) {
    let data_read = ctx.data.read().await;
    let mut arc_wallet = data_read.get::<WalletStore>().expect("Expected WalletStore in TypeMap");
    let tokens = arc_wallet.lock().await.get_token_accounts();

    let mut sum = 0.0;
    tokens.clone().into_iter().for_each(|token| {
        sum += token.ui_amount * token.coingecko_price;
    });


    let name_text: String = format!("💰 {:.2} 💰 ", sum);
    for _guild in _guilds.iter() {
        match _guild.edit_nickname(&ctx.http, Some(name_text.as_str())).await {
            Ok(_) => { info!("Changed Bot nickname!") }
            Err(_) => { error!("Unable to change bot nickname!") }
        };
    }
    let current_time = Utc::now();
    let formatted_time = current_time.to_rfc2822();

    ctx.set_activity(Activity::playing(&formatted_time)).await;
}


pub async fn init_bot(config: Config, wallet: Wallet) {
    let http = Http::new(&*config.clone().discord_token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix(config.clone().discord_prefix)).group(&GENERAL_GROUP);


    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(config.clone().discord_token, intents)
        .framework(framework)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<WalletStore>(Arc::new(Mutex::new(wallet)));
        data.insert::<ConfigStore>(Arc::new(Mutex::new(config)));
    }


    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
