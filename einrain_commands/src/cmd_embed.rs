use poise::serenity_prelude::{builder::CreateEmbed, CacheHttp};
use einrain_utils::{Context, Error};

#[poise::command(slash_command, ephemeral, subcommands("create", "edit"))]
#[doc = "Create and edit embeds"]
pub async fn embed(
    _: Context<'_>
) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
#[doc = "Create embeds"]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Title"] title: String,
    #[description = "Content"] content: String,
) -> Result<(), Error> {
    let embed = CreateEmbed::default()
        .title(title)
        .description(content)
        .to_owned();

    let ephemeral_embed = CreateEmbed::default()
        .title("embed")
        .description("Embed created!")
        .to_owned();
    
    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            *e = ephemeral_embed;
            e
        })
    }).await?;
    
    ctx.channel_id().send_message(ctx.http(), |m| {
        m.embed(|e| {
            *e = embed;
            e
        })
    }).await?;
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
#[doc = "Edit embeds"]
pub async fn edit(
    ctx: Context<'_>,
    #[description = "Message ID"] message: poise::serenity_prelude::Message,
    #[description = "Title"] title: String,
    #[description = "Content"] content: String,
) -> Result<(), Error> {
    let embed = CreateEmbed::default()
        .title(title)
        .description(content)
        .to_owned();

    let ephemeral_embed = CreateEmbed::default()
        .title("embed")
        .description("Embed edited!")
        .to_owned();
    
    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            *e = ephemeral_embed;
            e
        })
    }).await?;
    
    ctx.channel_id().edit_message(ctx.http(), message.id, |m| {
        m.embed(|e| {
            *e = embed;
            e
        })
    }).await?;
    Ok(())
}