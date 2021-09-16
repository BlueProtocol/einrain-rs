use einrain_utils::{Error, PrefixContext};

#[poise::command(owners_only, hide_in_help)]
pub async fn unregister(ctx: PrefixContext<'_>, #[flag] global: bool) -> Result<(), Error> {
    let guild_id = ctx.msg.guild_id.ok_or("Must be called in guild")?;
    let http = &ctx.discord.http;

    let commands = if global {
        http.get_global_application_commands().await?
    } else {
        http.get_guild_application_commands(guild_id.0).await?
    };
    
    poise::say_prefix_reply(ctx, format!("Unregistering {} commands...", commands.len())).await?;
    for cmd in commands {
        if global {
            http.delete_global_application_command(cmd.id.0).await?;
        } else {
            http.delete_guild_application_command(guild_id.0, cmd.id.0).await?;
        }
    }

    poise::say_prefix_reply(ctx, "Done!".to_owned()).await?;

    Ok(())
}