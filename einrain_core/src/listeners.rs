use poise::serenity_prelude::{self as serenity};
use poise::{Context, ErrorContext, Event, Framework};
use tracing::{error, info};

use einrain_utils::{Data, Error};

pub async fn on_error(error: Error, ctx: ErrorContext<'_, Data, Error>) {
    match ctx {
        ErrorContext::Setup => error!("Error in user data setup: {}", error),
        ErrorContext::Listener(event) => error!(
            "User event listener encountered an error on {} event: {}",
            event.name(),
            error
        ),
        ErrorContext::Command(poise::CommandErrorContext::Prefix(ctx)) => {
            error!(
                "Error in prefix command \"{}\" from message \"{}\": {}",
                &ctx.command.name, &ctx.ctx.msg.content, error
            );
        }
        ErrorContext::Command(poise::CommandErrorContext::Slash(ctx)) => {
            error!("Error in slash command \"{}\": {}", ctx.command.name, error);
        }
    }
}

pub async fn pre_command(ctx: Context<'_, Data, Error>) {
    let user;
    let mut command;
    let guild;

    match ctx {
        Context::Slash(slash) => {
            user = slash.interaction.user.id.0;

            command = String::from("/");
            command.push_str(slash.interaction.data.name.as_str());
            for data_option in &slash.interaction.data.options {
                command.push(' ');
                command.push('\'');
                command.push_str(data_option.name.as_str());
                command.push(':');
                if let Some(value) = &data_option.value {
                    command.push_str(&format!("{:?}", value));
                }
                command.push('\'');
            }
            
            guild = match slash.interaction.guild_id {
                Some(guild_id) => guild_id.to_string(),
                None => String::from("DM")
            };
        },
        Context::Prefix(prefix) => {
            user = prefix.msg.author.id.0;

            command = String::from(&prefix.msg.content);

            guild = match prefix.msg.guild_id {
                Some(guild_id) => guild_id.to_string(),
                None => String::from("DM")
            };
        }
    }
    
    info!("User {} sends command \"{}\" from guild {}", user, command, guild);
}

pub async fn listener(_ctx: &serenity::Context, _event: &Event<'_>, _framework: &'_ Framework<Data, Error>, _data: &Data) -> Result<(), Error> {
    Ok(())
}