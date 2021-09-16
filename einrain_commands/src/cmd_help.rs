use einrain_utils::{Context, Error};

/// Show this help menu
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    poise::defaults::help(
        ctx,
        command.as_deref(),
        "This is an example bot made to showcase features of my custom Discord bot framework",
        poise::defaults::HelpResponseMode::Ephemeral,
    )
    .await?;
    Ok(())
}