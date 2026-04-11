use anyhow::{Context, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    // cTrader
    pub ctrader_client_id: String,
    pub ctrader_secret: String,
    pub ctrader_host: String,
    pub ctrader_port: u16,
    pub ctrader_mode: String, // "mock" | "live"

    // Telegram
    pub telegram_bot_token: String,
    pub telegram_admin_ids: Vec<i64>,
    pub telegram_notify_chat_id: String,

    // REST API
    pub api_key: String,
    pub api_port: u16,

    // Storage
    pub database_url: String,

    // Logging
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            ctrader_client_id: env::var("CTRADER_CLIENT_ID")
                .context("CTRADER_CLIENT_ID is required")?,
            ctrader_secret: env::var("CTRADER_SECRET")
                .context("CTRADER_SECRET is required")?,
            ctrader_host: env::var("CTRADER_HOST")
                .unwrap_or_else(|_| "openapi.ctrader.com".to_string()),
            ctrader_port: env::var("CTRADER_PORT")
                .unwrap_or_else(|_| "5035".to_string())
                .parse()
                .context("CTRADER_PORT must be a number")?,
            ctrader_mode: env::var("CTRADER_MODE")
                .unwrap_or_else(|_| "mock".to_string()),

            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")
                .context("TELEGRAM_BOT_TOKEN is required")?,
            telegram_admin_ids: env::var("TELEGRAM_ADMIN_IDS")
                .unwrap_or_default()
                .split(',')
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .collect(),
            telegram_notify_chat_id: env::var("TELEGRAM_NOTIFY_CHAT_ID")
                .unwrap_or_default(),

            api_key: env::var("API_KEY")
                .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()),
            api_port: env::var("API_PORT")
                .unwrap_or_else(|_| "7381".to_string())
                .parse()
                .context("API_PORT must be a number")?,

            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://iztrade.db".to_string()),

            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }

    pub fn is_mock(&self) -> bool {
        self.ctrader_mode == "mock"
    }

    pub fn is_admin(&self, user_id: i64) -> bool {
        self.telegram_admin_ids.is_empty() || self.telegram_admin_ids.contains(&user_id)
    }
}
