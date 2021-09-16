use poise::serenity::builder::CreateEmbed;

use einrain_utils::{Context, Error};

#[poise::command(slash_command)]
#[doc = "A ping command"]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {
    let embed = CreateEmbed::default()
        .title("pong")
        .description("Hey, I'm alive!")
        .to_owned();
    
    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            *e = embed;
            e
        })
    }).await?;
    Ok(())
}