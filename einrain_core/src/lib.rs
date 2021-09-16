mod listeners;

use std::collections::HashSet;

use poise::serenity_prelude as serenity;
use poise::serenity::client::parse_token;

use tracing::{error, info};

use einrain_commands::*;
use einrain_config::BotConfig;
use einrain_utils::Data;

use listeners::*;

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
    let token_info = parse_token(&config.token()).expect("invalid token");
    let application_id = *token_info.bot_user_id.as_u64();

    let mut options = poise::FrameworkOptions {
        on_error: |error, ctx| Box::pin(on_error(error, ctx)),
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        allowed_mentions: Some({
            let mut f = serenity::CreateAllowedMentions::default();
            // Only support direct user pings by default
            f.empty_parse().parse(serenity::ParseValue::Users);
            f
        }),
        listener: |ctx, event, framework, dfd| Box::pin(listener(ctx, event, framework, dfd)),
        owners: {
            let mut owners = HashSet::new();
            owners.insert(serenity::UserId(config.owner_id()));
            owners
        },
        ..Default::default()
    };
    
    options.command(cmd_class::class(), |f| f);
    options.command(cmd_help::help(), |f| f);
    options.command(cmd_id::id(), |f| f);
    options.command(cmd_ping::ping(), |f| f);
    options.command(cmd_register::register(), |f| f);
    options.command(cmd_skills::skills(), |f| f);
    options.command(cmd_unregister::unregister(), |f| f);
    
    let framework = poise::Framework::new(
        "~".to_owned(), // prefix
        serenity::ApplicationId(application_id),
        move |_ctx, _ready, _framework| {
            Box::pin(async {Ok(Data {})})
        },
        options,
    );

    info!("Starting client...");
    // Finally, start a single shard, and start listening to events.
    if let Err(e) = framework.start(serenity::ClientBuilder::new(&config.token())).await {
        error!("Client unable to start: {:?}", e);
    };
}