use std::borrow::BorrowMut;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use crate::bot::ConfigStore;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let config = data_read.get::<ConfigStore>().expect("Expected ConfigStore in TypeMap");
    let prefix = config.lock().await.clone().discord_prefix;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Wallet Tracker Help")
                .description("This are all commands available for the wallet-tracker-bot.")
                .color(Color::ORANGE)
                .field(prefix.clone() + "help", "Shows this message", false)
                .field(prefix.clone() + "address", "Prints the wallet-address", false)
                .field(prefix.clone() + "domain", "Prints the wallet-domain", false)
                .field(prefix.clone() + "solscan", "Prints a link to solcan.io", false)
                .field(prefix.clone() + "solanaBeach", "Prints a link to solanabeach.io", false)
                .field(prefix.clone() + "step", "Prints a link to step.finance", false)
                .field(prefix.clone() + "wallet", "Prints a wallet info", false)

                .timestamp(Timestamp::now())
        })
    }).await?;

    Ok(())
}