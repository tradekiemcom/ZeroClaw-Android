use iztrade::{config, state, models, storage, telegram, api, risk, engine, cli};

use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber::{EnvFilter, fmt};
use std::net::SocketAddr;
use clap::{Parser, Subcommand};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "iztrade")]
#[command(about = "iZFx.Trade – Trading Execution Hub CLI", long_about = None)]
struct Cli {
    /// Chạy lệnh trực tiếp mà không vào Console (Ví dụ: -r, -p, -c)
    #[arg(short, long)]
    report: bool,

    #[arg(short, long)]
    positions: bool,

    #[arg(short = 'L', long)]
    accounts: bool,

    #[arg(short, long)]
    bots: bool,

    #[arg(short = 'c', long)]
    close_all: bool,

    #[arg(short = 'a', long)]
    active_all: bool,

    #[arg(short = 'd', long)]
    disable_all: bool,

    /// Lệnh bắt đầu bằng # (Ví dụ: "#BUY XAUUSD 0.01")
    #[arg(long)]
    exec: Option<String>,

    /// Account ID để lọc lệnh
    #[arg(long)]
    id: Option<i64>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Khởi động hệ thống chạy ngầm
    Up,
    /// Tắt toàn bộ hệ thống
    Down,
    /// Vào chế độ Console tương tác (Mặc định)
    Console,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // ── Init logging ──────────────────────────────────────────────
    let config = config::Config::from_env()?;
    fmt()
        .with_env_filter(EnvFilter::new(&config.log_level))
        .init();

    // ── Database ──────────────────────────────────────────────────
    let db = storage::init_pool(&config.database_url).await?;
    let state = state::AppState::new(config.clone(), db).await;

    // ── Handle Service Commands (Up/Down) ─────────────────────────
    if let Some(cmd) = &cli.command {
        match cmd {
            Commands::Up => {
                println!("[STARTUP] Starting iZFx.Trade in background...");
                // Note: Thực tế trên Android/Termux có thể dùng nohup hoặc pm2
                // Ở đây ta mô phỏng bằng cách in hướng dẫn hoặc xử lý PID
                return Ok(());
            }
            Commands::Down => {
                println!("[SHUTDOWN] Stopping iZFx.Trade...");
                return Ok(());
            }
            _ => {}
        }
    }

    // ── Handle Direct Commands (Non-interactive) ──────────────────
    if let Some(res) = handle_direct_commands(&cli, state.clone()).await? {
        println!("{}", res);
        return Ok(());
    }

    // ── Banner ────────────────────────────────────────────────────
    println!(r#"
  ╔══════════════════════════════════════════╗
  ║   iZFx.Trade – Trading Execution Hub    ║
  ║   Version 2.0.0  │  Multi-Account       ║
  ║   cTrader Open API + Telegram + REST    ║
  ╚══════════════════════════════════════════╝
"#);

    // ── Launch services in background ─────────────────────────────
    
    // cTrader Connector
    let state_ctr = state.clone();
    tokio::spawn(async move {
        if let Err(e) = state_ctr.pool.connect_all(state_ctr.clone()).await {
            error!("cTrader connection pool error: {}", e);
        }
    });

    // REST API
    let addr: SocketAddr = format!("0.0.0.0:{}", config.api_port).parse()?;
    let router = api::build_router(state.clone());
    tokio::spawn(async move {
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                info!("[REST] REST API listening on http://{}", addr);
                if let Err(e) = axum::serve(listener, router).await {
                    error!("REST API service error: {}", e);
                }
            }
            Err(e) => {
                error!("[ERROR] Failed to bind REST API to {}: {}. Is another instance running?", addr, e);
            }
        }
    });

    // Telegram Bot
    let state_tg = state.clone();
    tokio::spawn(async move {
        telegram::run_telegram_bot(state_tg).await;
    });

    // Risk Monitor
    let state_risk = state.clone();
    tokio::spawn(async move {
        risk::run_risk_monitor(state_risk).await;
    });

    // Daily Reset
    let state_reset = state.clone();
    tokio::spawn(async move {
        risk::run_daily_reset(state_reset).await;
    });

    // ── Start iZ-Console (REPL) ───────────────────────────────────
    let mut console = cli::IzConsole::new(state.clone());
    if let Err(e) = console.run().await {
        error!("Console error: {}", e);
    }

    info!("[INFO] iZFx.Trade stopped.");
    Ok(())
}

async fn handle_direct_commands(cli: &Cli, state: Arc<state::AppState>) -> Result<Option<String>> {
    use crate::models::{OrderRequest, OrderAction, TradeSource, AccountScope};
    use crate::engine::dispatch;

    let mut action_opt = None;

    if cli.report { 
        let accounts = state.accounts.read().await;
        let mut report = "[GLOBAL REPORT]\n".to_string();
        for (_, acc) in accounts.iter() {
            report.push_str(&format!("Acc {}: PNL {:.2} | Equity {:.2}\n", acc.id, acc.daily_pnl, acc.equity));
        }
        return Ok(Some(report));
    }
    
    if cli.positions {
        let positions = state.positions.read().await;
        let mut list = "[OPEN POSITIONS]\n".to_string();
        for pos in positions.iter().filter(|p| p.status == crate::models::PositionStatus::Open) {
            list.push_str(&format!("[{}] {} {} {:.2} @ {:.5}\n", pos.account_id, pos.side, pos.symbol, pos.volume, pos.open_price));
        }
        return Ok(Some(list));
    }

    if cli.accounts {
        let accounts = state.accounts.read().await;
        let mut list = "[ACCOUNTS]\n".to_string();
        for (_, acc) in accounts.iter() {
            list.push_str(&format!("[{}] {} {:?} | PNL: {:.2}\n", acc.id, acc.name, acc.account_type, acc.daily_pnl));
        }
        return Ok(Some(list));
    }
    
    if cli.close_all { action_opt = Some(OrderAction::CloseAll); }
    if cli.active_all { action_opt = Some(OrderAction::EnableAutotrade); }
    if cli.disable_all { action_opt = Some(OrderAction::DisableAutotrade); }

    if let Some(action) = action_opt {
        let mut req = OrderRequest::new(TradeSource::Api, "cli".to_string(), action);
        if let Some(id) = cli.id {
            req.account_scope = AccountScope::Single;
            req.account_ids = vec![id];
        }
        let res = dispatch(state, req).await?;
        return Ok(Some(res.messages.join("\n")));
    }

    if let Some(hash_cmd) = &cli.exec {
        if let Some(mut req) = crate::engine::IzParser::parse(hash_cmd, TradeSource::Api) {
            if let Some(id) = cli.id {
                req.account_scope = AccountScope::Single;
                req.account_ids = vec![id];
            }
            let res = dispatch(state, req).await?;
            return Ok(Some(res.to_telegram_msg()));
        }
    }

    Ok(None)
}
