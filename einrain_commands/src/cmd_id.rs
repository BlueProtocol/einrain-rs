use poise::serenity_prelude as serenity;
use poise::serenity_prelude::builder::CreateEmbed;

use einrain_utils::{Context, Error};

#[poise::command(slash_command)]
#[doc = "Get a user id"]
pub async fn id(
    ctx: Context<'_>,
    #[description = "The user to lookup"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or(ctx.author());
    let resp = format!("{}'s id is {}", user.tag(), user.id);

    let embed = CreateEmbed::default()
        .title("id")
        .description(resp)
        .to_owned();
    
    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            *e = embed;
            e
        })
    }).await?;
    Ok(())
}