use std::borrow::BorrowMut;
use prettytable::{row, Table};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use solana_sdk::signature::Signature;
use crate::bot::{WalletStore};

#[command]
async fn store(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let arc_wallet = data_read.get::<WalletStore>().expect("Expected ConfigStore in TypeMap");

    let tokens = arc_wallet.lock().await.get_token_accounts();

    let mut table_signatures_stored = Table::new();
    table_signatures_stored.add_row(row!["Symbol", "Signature"]);

    tokens.into_iter().for_each(|token| {
        table_signatures_stored.add_row(row![token.symbol, token.last_signature.unwrap_or_default().to_string()]);
    });

    let message =
        format!("Stored-Signatures: \n```\n{:}```", table_signatures_stored.to_string());

    msg.channel_id.say(&ctx.http, message).await?;

    Ok(())
}