use einrain_utils::{Error, Context};

/// Register or unregister slash commands in this guild or globally
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}