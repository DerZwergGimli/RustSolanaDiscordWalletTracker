use std::borrow::BorrowMut;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use crate::bot::{WalletStore};

#[command]
async fn wallet(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let arc_wallet = data_read.get::<WalletStore>().expect("Expected ConfigStore in TypeMap");
    let table_string = arc_wallet.lock().await.table_token_accounts();
    let sol_balance = arc_wallet.lock().await.get_sol();
    let tokens = arc_wallet.lock().await.get_token_accounts();

    let mut sum = 0.0;
    tokens.clone().into_iter().for_each(|token| {
        sum += token.ui_amount * token.coingecko_price;
    });

    let message =
        format!("Wallet-Tokens: \n```\nSOL: {:}```\n````\n{:}```\n Total: \n```\n{:.2} USD```", sol_balance, table_string, sum);

    msg.channel_id.say(&ctx.http, message).await?;

    Ok(())
}