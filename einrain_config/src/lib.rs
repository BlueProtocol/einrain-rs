mod config;

pub use config::*;
use once_cell::sync::OnceCell;

pub static BOT_CONFIG: OnceCell<BotConfig> = OnceCell::new();