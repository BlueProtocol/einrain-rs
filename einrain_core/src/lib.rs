mod handlers;

use std::collections::HashSet;

use poise::serenity_prelude as serenity;

use tracing::{error, info};

use einrain_commands::*;
use einrain_config::BotConfig;
use einrain_utils::Data;

use handlers::*;

pub async fn start() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    info!("Starting Einrain...");
    
    info!("Loading configuration...");
    BotConfig::set("config.toml");
    let config = BotConfig::get().expect("Couldn't access BOT_CONFIG to get the token");
    info!("Loaded configuration!");

    info!("Initializing client...");
    let options = poise::FrameworkOptions {
        commands: vec![
            cmd_class::class(),
            cmd_help::help(),
            cmd_id::id(),
            cmd_ping::ping(),
            cmd_register::register(),
            cmd_skills::skills(),
        ],
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        allowed_mentions: Some({
            let mut f = serenity::CreateAllowedMentions::default();
            // Only support direct user pings by default
            f.empty_parse().parse(serenity::ParseValue::Users);
            f
        }),
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            ..Default::default()
        },
        owners: {
            let mut owners = HashSet::new();
            owners.insert(serenity::UserId(config.owner_id()));
            owners
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(config.token())
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {Ok(Data {})})
        });

    info!("Starting client...");
    if let Err(e) = framework.run().await {
        error!("Client unable to start: {:?}", e);
    };
}