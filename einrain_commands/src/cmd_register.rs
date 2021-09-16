use einrain_utils::{Error, PrefixContext};

/// Register slash commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(owners_only, hide_in_help)]
pub async fn register(ctx: PrefixContext<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::defaults::register_slash_commands(ctx, global).await?;
    Ok(())
}