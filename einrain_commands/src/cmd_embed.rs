use poise::Modal;
use poise::serenity_prelude as serenity;
use serenity::{builder::CreateEmbed, CacheHttp};
use chrono::prelude::Utc;

use einrain_utils::{Context, Error};

#[poise::command(slash_command, ephemeral, subcommands("create", "edit"))]
#[doc = "Create and edit embeds. Use an embed builder like https://glitchii.github.io/embedbuilder/."]
pub async fn embed(
    _: Context<'_>
) -> Result<(), Error> {
    Ok(())
}

#[derive(Debug, poise::Modal)]
#[name = "Create Embed"] // Struct name by default
struct CreateEmbedModal {
    #[name = "Enter Embed JSON"] // Field name by default
    #[placeholder = "Paste JSON here"] // No placeholder by default
    #[paragraph] // Switches from single-line input to multiline text box
    embed_json: String,
}

#[poise::command(slash_command, ephemeral)]
#[doc = "Create embeds"]
pub async fn create(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let ephemeral_embed = CreateEmbed::default()
        .title("Embed Command")
        .description("Embed created!")
        .color(39129)
        .timestamp(Utc::now())
        .url("https://blue-protocol-db.com/")
        .author(|a| {
            a.name("Einrain Bot");
            a.url("https://blue-protocol-db.com/");
            a.icon_url("https://cdn.discordapp.com/avatars/877854575472820254/372e33189e5afde5e530f796318a668b.webp?size=1024")
        })
        .thumbnail("https://cdn.discordapp.com/avatars/877854575472820254/372e33189e5afde5e530f796318a668b.webp?size=1024")
        .footer(|f| {
            f.text("Einrain Bot");
            f.icon_url("https://cdn.discordapp.com/avatars/877854575472820254/372e33189e5afde5e530f796318a668b.webp?size=1024")
        })
        .to_owned();

    let Context::Application(context_interaction) = ctx else {
        return Ok(());
    };
    let Some(modal_data) = CreateEmbedModal::execute(context_interaction).await? else {
        return Ok(());
    };
    
    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            *e = ephemeral_embed;
            e
        })
    }).await?;

    let embed_json = serde_json::from_str(modal_data.embed_json.as_str()).unwrap();
    ctx.http().send_message(ctx.channel_id().0, &embed_json).await?;

    Ok(())
}

#[derive(Debug, poise::Modal)]
#[name = "Edit Embed"] // Struct name by default
struct EditEmbedModal {
    #[name = "Enter Embed JSON"] // Field name by default
    #[placeholder = "Paste JSON here"] // No placeholder by default
    #[paragraph] // Switches from single-line input to multiline text box
    embed_json: String,
}

#[poise::command(slash_command, ephemeral)]
#[doc = "Edit embeds"]
pub async fn edit(
    ctx: Context<'_>,
    #[description = "Message ID"] message: poise::serenity_prelude::Message,
) -> Result<(), Error> {
    let ephemeral_embed = CreateEmbed::default()
        .title("Embed Command")
        .description("Embed edited!")
        .color(39129)
        .timestamp(Utc::now())
        .url("https://blue-protocol-db.com/")
        .author(|a| {
            a.name("Einrain Bot");
            a.url("https://blue-protocol-db.com/");
            a.icon_url("https://cdn.discordapp.com/avatars/877854575472820254/372e33189e5afde5e530f796318a668b.webp?size=1024")
        })
        .thumbnail("https://cdn.discordapp.com/avatars/877854575472820254/372e33189e5afde5e530f796318a668b.webp?size=1024")
        .footer(|f| {
            f.text("Einrain Bot");
            f.icon_url("https://cdn.discordapp.com/avatars/877854575472820254/372e33189e5afde5e530f796318a668b.webp?size=1024")
        })
        .to_owned();

    let Context::Application(context_interaction) = ctx else {
        return Ok(());
    };
    let Some(modal_data) = EditEmbedModal::execute(context_interaction).await? else {
        return Ok(());
    };
    
    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            *e = ephemeral_embed;
            e
        })
    }).await?;

    let embed_json = serde_json::from_str(modal_data.embed_json.as_str()).unwrap();
    ctx.http().edit_message(ctx.channel_id().0, message.id.0, &embed_json).await?;

    Ok(())
}