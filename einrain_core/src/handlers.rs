use poise::serenity_prelude::{self as serenity};
use poise::{Context, Event, FrameworkContext};
use tracing::{error, info};

use einrain_utils::{Data, Error};

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => error!("Error in user data setup: {}", error),
        poise::FrameworkError::EventHandler { error, event, .. }  => {
            error!(
                "User event listener encountered an error on {} event: {}",
                event.name(),
                error
            )
        }
        poise::FrameworkError::Command { error, ctx } => {
            error!(
                "Error in command \"{}\" from message \"{}\": {}",
                ctx.command().name, &ctx.invocation_string(), error
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

pub async fn pre_command(ctx: Context<'_, Data, Error>) {
    let user;
    let command;
    let guild;

    user = ctx.author().id;
    command = ctx.invocation_string();
    guild = match ctx.guild_id() {
        Some(guild_id) => guild_id.to_string(),
        None => String::from("DM")
    };
    
    info!("User {} sends command \"{}\" from guild {}", user, command, guild);
}

pub async fn event_handler(_ctx: &serenity::Context, _event: &Event<'_>, _framework: FrameworkContext<'_, Data, Error>, _data: &Data) -> Result<(), Error> {
    Ok(())
}