use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use crate::bot::ConfigStore;

#[command]
async fn address(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigStore>().expect("Expected ConfigStore in TypeMap");
    let config = arc_config.lock().await.clone();

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Wallet-Address")
                .color(Color::ORANGE)
                .field("Address", config.wallet_address.clone(), false)
                .field("Solscan", format!("https://solscan.io/account/{:}", config.wallet_address.clone()), false)
                .field("SolanaBeach", format!("https://solanabeach.io/address/{:}", config.wallet_address.clone()), false)
                .field("SolanaFM", format!("https://solana.fm/address/{:}", config.wallet_address.clone()), false)
                .field("STEP.Finance", format!("https://app.step.finance/en/dashboard?watching={:}", config.wallet_address), false)
        })
    }).await;

    Ok(())
}