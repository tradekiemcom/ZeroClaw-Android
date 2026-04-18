use std::sync::Arc;
use anyhow::Result;
use tracing::{info, warn};
use tokio::time::{interval, Duration};

use crate::state::AppState;

/// Risk Monitor chạy mỗi 60 giây kiểm tra P&L của tất cả accounts và bots
pub async fn run_risk_monitor(state: Arc<AppState>) {
    let mut ticker = interval(Duration::from_secs(60));
    info!("[SYSTEM] Risk Monitor started (interval: 60s)");

    loop {
        ticker.tick().await;
        if let Err(e) = check_risk(&state).await {
            warn!("[ERROR] Risk monitor error: {}", e);
        }
    }
}

async fn check_risk(state: &Arc<AppState>) -> Result<()> {
    // Lưu tạm các trigger để xử lý sau khi nhả lock
    let mut triggers = Vec::new();

    {
        let accounts = state.accounts.read().await;
        for acc in accounts.values() {
            if !acc.autotrade {
                continue;
            }
            if let Some(reason) = acc.should_halt_trading() {
                triggers.push((acc.id, acc.name.clone(), acc.daily_pnl, reason));
            }
        }
    } // lock dropped here

    for (acc_id, name, pnl, reason) in triggers {
        warn!("[STOP] Risk Halt | Account: {} ({}) | {}", acc_id, name, reason);

        // Disable autotrade
        state.set_all_autotrade(false).await?;

        // Notify Telegram
        if let Some(bot) = &state.telegram_bot {
            let msg = format!(
                "[RISK ALERT]\n\n\
                Account: #{} {}\n\
                STATS PnL: {:.2}\n\
                [INFO] {}\n\n\
                [OFF] Autotrade has been disabled automatically.",
                acc_id, name, pnl, reason
            );
            let _ = send_telegram_notify(bot, &state.config.telegram_notify_chat_id, &msg).await;
        }
    }

    Ok(())
}


/// Reset daily P&L lúc 00:00 UTC
pub async fn run_daily_reset(state: Arc<AppState>) {
    use chrono::{Utc, Timelike};

    loop {
        let now = Utc::now();
        // Tính thời gian đến 00:00 UTC tiếp theo
        let seconds_until_midnight = 86400 - (now.num_seconds_from_midnight() as u64);
        tokio::time::sleep(Duration::from_secs(seconds_until_midnight)).await;

        info!("[RESET] Daily reset: Clearing P&L counters...");
        if let Err(e) = crate::storage::reset_daily_pnl(&state.db).await {
            warn!("[ERROR] Daily reset error: {}", e);
        } else {
            // Reset in-memory state
            {
                let mut accounts = state.accounts.write().await;
                for acc in accounts.values_mut() {
                    acc.daily_pnl = 0.0;
                }
            }
            {
                let mut bots = state.bots.write().await;
                for bot in bots.values_mut() {
                    bot.daily_pnl = 0.0;
                    bot.trade_count_today = 0;
                }
            }
            info!("[OK] Daily reset complete");
        }
    }
}

async fn send_telegram_notify(bot: &teloxide::Bot, chat_id: &str, text: &str) -> Result<()> {
    use teloxide::prelude::*;
    use teloxide::types::ChatId;

    let cid = if chat_id.starts_with('@') {
        // Username như @TradeKiemGold
        bot.send_message(ChatId(0), text) // sẽ fail, dùng username API
    } else if let Ok(id) = chat_id.parse::<i64>() {
        bot.send_message(ChatId(id), text)
    } else {
        warn!("Invalid chat_id: {}", chat_id);
        return Ok(());
    };

    // Parse HTML/Markdown
    let _ = cid.parse_mode(teloxide::types::ParseMode::MarkdownV2).await;
    Ok(())
}
