use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn binary(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name: String = args.single()?;
    let mut n_in_bin = "Tu palabra en binario es: ".to_string();

    for c in name.clone().into_bytes() {
        n_in_bin += &format!("0{:b} ", c);
    }

    msg.channel_id.say(&ctx.http, n_in_bin).await?;

    Ok(())
}
