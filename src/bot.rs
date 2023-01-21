use std::borrow::BorrowMut;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::collections::HashSet;
use std::sync::Arc;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::prelude::*;
use chrono::offset::Utc;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::*;
use serenity::framework::standard::macros::group;

use crate::commands::ping::*;
use crate::commands::help::*;
use crate::commands::wallet::*;

use crate::config::config::Config;
use crate::solana::wallet::Wallet;

struct WalletStore;

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
#[commands(ping, help, wallet)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        let ctx = Arc::new(ctx);


        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);

            tokio::spawn(async move {
                loop {
                    // We clone Context again here, because Arc is owned, so it moves to the
                    // new function.
                    check_wallet(Arc::clone(&ctx1)).await;
                    let data_read = ctx1.data.read().await;
                    data_read.get::<WalletStore>().unwrap().lock().await.print_wallet();
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            });

            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    set_status_to_current_time(Arc::clone(&ctx2)).await;
                    let mut data_read = ctx2.data.read().await;
                    let mut wallet = data_read.get::<WalletStore>().unwrap();
                    wallet.lock().await.fetch_solana_balance();
                    wallet.lock().await.fetch_token_accounts_balances();
                    //data_read.insert::<WalletStore>(Arc::new(wallet));
                    //data_read.entry();
                    //wallet.fetch_solana_balance();
                    //wallet.fetch_solana_balance();
                    //wallet.fetch_token_accounts_balances();


                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn check_wallet(ctx: Arc<Context>) {
    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    let message = ChannelId(381926291785383946)
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("System Resource Load")
                    .field("CPU Load Average", format!("{:.2}%", 10.0 * 10.0), false)
                    .field(
                        "Memory Usage",
                        format!(
                            "{:.2} MB Free out of {:.2} MB",
                            100 as f32 / 1000.0,
                            100 as f32 / 1000.0
                        ),
                        false,
                    )
            })
        })
        .await;
    if let Err(why) = message {
        eprintln!("Error sending message: {:?}", why);
    };
}

async fn set_status_to_current_time(ctx: Arc<Context>) {
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
