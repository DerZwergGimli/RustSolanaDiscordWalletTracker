use std::borrow::BorrowMut;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use crate::bot::ConfigStore;

#[command]
async fn wallet(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    let arc_config = data_read.get::<ConfigStore>().expect("Expected ConfigStore in TypeMap");
    let config = arc_config.lock().await.clone();


    let message = "Hallo";

    msg.channel_id.say(&ctx.http, message).await?;

    Ok(())
}