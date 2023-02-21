use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn bang(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "BANG! https://tenor.com/view/yuno-gasai-yuno-anime-future-diary-gun-gif-17876001").await?;

    Ok(())
}
