mod config;
mod state;
mod models;
mod storage;
mod ctrader;
mod telegram;
mod api;
mod risk;
mod engine;

use anyhow::Result;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    // ── Init logging ──────────────────────────────────────────────
    let config = config::Config::from_env()?;
    fmt()
        .with_env_filter(EnvFilter::new(&config.log_level))
        .init();

    // ── Banner ────────────────────────────────────────────────────
    println!(r#"
  ╔══════════════════════════════════════════╗
  ║   iZFx.Trade – Trading Execution Hub    ║
  ║   Version 2.0.0  │  Multi-Account       ║
  ║   cTrader Open API + Telegram + REST    ║
  ╚══════════════════════════════════════════╝
"#);

    info!("🚀 Starting iZFx.Trade v2.0.0");
    info!("📡 Mode: {}", config.ctrader_mode.to_uppercase());
    info!("🔌 API Port: {}", config.api_port);
    info!("🤖 Telegram: configured");

    // ── Database ──────────────────────────────────────────────────
    info!("📦 Initializing database: {}", config.database_url);
    let db = storage::init_pool(&config.database_url).await?;
    info!("✅ Database ready");

    // ── App State ─────────────────────────────────────────────────
    let state = state::AppState::new(config.clone(), db).await;

    // ── Connect cTrader ───────────────────────────────────────────
    state.ctrader.connect().await?;

    // ── REST API server ───────────────────────────────────────────
    let router = api::build_router(state.clone());
    let addr: SocketAddr = format!("0.0.0.0:{}", config.api_port).parse()?;
    info!("🌐 REST API listening on http://{}", addr);

    // ── Launch all services concurrently ──────────────────────────
    tokio::select! {
        // REST API
        result = axum::serve(
            tokio::net::TcpListener::bind(addr).await?,
            router
        ) => {
            if let Err(e) = result {
                tracing::error!("REST API error: {}", e);
            }
        }

        // Telegram Bot (long polling)
        _ = telegram::run_telegram_bot(state.clone()) => {
            tracing::warn!("Telegram bot stopped");
        }

        // Risk Monitor (60s interval)
        _ = risk::run_risk_monitor(state.clone()) => {
            tracing::warn!("Risk monitor stopped");
        }

        // Daily Reset (00:00 UTC)
        _ = risk::run_daily_reset(state.clone()) => {
            tracing::warn!("Daily reset task stopped");
        }

        // Shutdown signal
        _ = tokio::signal::ctrl_c() => {
            info!("⛔ Shutting down iZFx.Trade...");
        }
    }

    info!("👋 iZFx.Trade stopped.");
    Ok(())
}
