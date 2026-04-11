use std::sync::Arc;
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{Message, ParseMode};
use tracing::{info, warn};

use crate::state::AppState;
use crate::models::order::{OrderRequest, OrderAction, TradeSource, TradeSide, AccountScope};
use crate::engine::dispatch;

/// Khởi động Telegram Bot polling
pub async fn run_telegram_bot(state: Arc<AppState>) {
    let bot = Bot::new(&state.config.telegram_bot_token);
    info!("🤖 Telegram Bot started | Polling...");

    let state_clone = state.clone();
    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let state = state_clone.clone();
        async move {
            if let Err(e) = handle_message(&bot, &msg, state).await {
                warn!("⚠️ Telegram handler error: {}", e);
            }
            Ok(())
        }
    })
    .await;
}

async fn handle_message(bot: &Bot, msg: &Message, state: Arc<AppState>) -> Result<()> {
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);
    let text = match msg.text() {
        Some(t) => t,
        None => return Ok(()),
    };

    // Chỉ xử lý lệnh có /
    if !text.starts_with('/') {
        return Ok(());
    }

    // Kiểm tra admin
    if !state.is_admin(user_id) {
        reply(bot, msg, "❌ Bạn không có quyền sử dụng bot này.").await?;
        return Ok(());
    }

    info!("📨 Telegram cmd | User: {} | Cmd: {}", user_id, text);

    let response = parse_and_execute(text, state.clone()).await;
    reply_md(bot, msg, &response).await?;
    Ok(())
}

/// Parse lệnh Telegram và dispatch đến Core Engine
async fn parse_and_execute(text: &str, state: Arc<AppState>) -> String {
    let parts: Vec<&str> = text.split_whitespace().collect();
    let cmd = parts[0].to_lowercase();

    match cmd.as_str() {
        "/help" | "/start" => help_text(),

        // ── Autotrade ──────────────────────────────────────────────────
        "/a" => {
            let req = OrderRequest::new(TradeSource::Telegram, "system".to_string(), OrderAction::EnableAutotrade);
            run_dispatch(state, req).await
        }
        "/d" => {
            let req = OrderRequest::new(TradeSource::Telegram, "system".to_string(), OrderAction::DisableAutotrade);
            run_dispatch(state, req).await
        }

        // ── Close ──────────────────────────────────────────────────────
        "/c" => {
            if parts.len() > 1 {
                // /c <bot>
                let bot_id = parts[1].to_string();
                let req = OrderRequest::new(TradeSource::Telegram, bot_id, OrderAction::Close);
                run_dispatch(state, req).await
            } else {
                // /c → close all
                let req = OrderRequest::new(TradeSource::Telegram, "system".to_string(), OrderAction::CloseAll);
                run_dispatch(state, req).await
            }
        }

        // ── Bot Control ────────────────────────────────────────────────
        "/on" => {
            if parts.len() < 2 { return "❌ Dùng: /on <bot_id>".to_string(); }
            let bot_id = parts[1].to_string();
            let req = OrderRequest::new(TradeSource::Telegram, bot_id, OrderAction::EnableBot);
            run_dispatch(state, req).await
        }
        "/off" => {
            if parts.len() < 2 { return "❌ Dùng: /off <bot_id>".to_string(); }
            let bot_id = parts[1].to_string();
            let req = OrderRequest::new(TradeSource::Telegram, bot_id, OrderAction::DisableBot);
            run_dispatch(state, req).await
        }

        // ── Positions & Orders ─────────────────────────────────────────
        "/p" | "/positions" => {
            let positions = state.get_open_positions().await;
            if positions.is_empty() {
                "📭 Không có lệnh nào đang mở.".to_string()
            } else {
                let mut lines = vec![format!("📊 *Open Positions* ({})", positions.len())];
                for pos in &positions {
                    lines.push(format!(
                        "• `{}` {} {} {:.2}L@{:.2} | P&L: {:.2}",
                        pos.bot_id, pos.side, pos.symbol, pos.volume, pos.open_price, pos.pnl
                    ));
                }
                lines.join("\n")
            }
        }

        // ── Reports ────────────────────────────────────────────────────
        "/r" | "/report" => state.format_accounts_summary().await,
        "/rp" => state.format_bots_summary().await,

        // ── Bots list ──────────────────────────────────────────────────
        "/bots" => state.format_bots_summary().await,
        "/accounts" => state.format_accounts_summary().await,

        // ── Direct Trade ───────────────────────────────────────────────
        "/buy" => parse_trade(TradeSource::Telegram, TradeSide::Buy, &parts, state).await,
        "/sell" => parse_trade(TradeSource::Telegram, TradeSide::Sell, &parts, state).await,

        _ => format!("❓ Lệnh không xác định: `{}`\nGõ /help để xem danh sách lệnh.", cmd),
    }
}

/// Parse lệnh /buy hoặc /sell
/// Format: /buy XAUUSD 0.1 gold_scalper sl=2300 tp=2350
async fn parse_trade(
    source: TradeSource,
    side: TradeSide,
    parts: &[&str],
    state: Arc<AppState>,
) -> String {
    if parts.len() < 4 {
        return format!(
            "❌ Format:\n`/{} SYMBOL VOLUME BOT_ID [sl=X] [tp=X]`\n\nVí dụ:\n`/buy XAUUSD 0.1 gold_scalper sl=3280 tp=3320`",
            if side == TradeSide::Buy { "buy" } else { "sell" }
        );
    }

    let symbol = parts[1].to_uppercase();
    let volume: f64 = match parts[2].parse() {
        Ok(v) => v,
        Err(_) => return "❌ Volume không hợp lệ".to_string(),
    };
    let bot_id = parts[3].to_string();

    let mut sl: Option<f64> = None;
    let mut tp: Option<f64> = None;

    for &part in &parts[4..] {
        if let Some(val) = part.strip_prefix("sl=") {
            sl = val.parse().ok();
        } else if let Some(val) = part.strip_prefix("tp=") {
            tp = val.parse().ok();
        }
    }

    let req = OrderRequest::market_order(source, bot_id, symbol, side, volume, sl, tp);
    run_dispatch(state, req).await
}

async fn run_dispatch(state: Arc<AppState>, req: OrderRequest) -> String {
    match dispatch(state.clone(), req).await {
        Ok(result) => {
            let mut msg = result.to_telegram_msg();
            if !result.messages.is_empty() {
                msg.push('\n');
                msg.push_str(&result.messages.join("\n"));
            }
            // Announce positions opened
            for pos in &result.positions {
                notify_trade_opened(&state, pos).await;
            }
            msg
        }
        Err(e) => format!("❌ Lỗi: {}", e),
    }
}

/// Push thông báo lệnh mở về group
async fn notify_trade_opened(state: &Arc<AppState>, pos: &crate::models::Position) {
    if state.config.telegram_notify_chat_id.is_empty() {
        return;
    }
    if let Some(bot) = &state.telegram_bot {
        let msg = format!(
            "🟢 *OPEN*\n\
            Account: #{}\n\
            Bot: `{}`\n\
            📈 {} {} @ {:.5}\n\
            💰 Vol: {:.2}\n\
            🛡 SL: {} \\| TP: {}",
            pos.account_id,
            pos.bot_id,
            pos.side.to_uppercase(),
            pos.symbol,
            pos.open_price,
            pos.volume,
            pos.sl.map(|v| format!("{:.5}", v)).unwrap_or("—".to_string()),
            pos.tp.map(|v| format!("{:.5}", v)).unwrap_or("—".to_string()),
        );
        let _ = send_notify(bot, &state.config.telegram_notify_chat_id, &msg).await;
    }
}

/// Gửi notification về channel/group
pub async fn send_notify(bot: &Bot, chat_id: &str, text: &str) -> Result<()> {
    use teloxide::types::ChatId;

    if chat_id.is_empty() { return Ok(()); }

    // Parse chat_id: có thể là @username hoặc numeric id
    let result = if let Ok(id) = chat_id.trim_start_matches('@').parse::<i64>() {
        bot.send_message(ChatId(id), text)
            .parse_mode(ParseMode::MarkdownV2)
            .await
    } else {
        // dùng username string
        bot.send_message(chat_id.to_string(), text)
            .parse_mode(ParseMode::MarkdownV2)
            .await
    };

    if let Err(e) = result {
        warn!("📢 Notify failed: {}", e);
    }
    Ok(())
}

fn help_text() -> String {
    r#"🤖 *iZFx\.Trade v2\.0*

*Autotrade:*
`/a` — Bật autotrade
`/d` — Tắt autotrade

*Bot Control:*
`/on <bot>` — Enable bot
`/off <bot>` — Disable bot
`/bots` — Danh sách bots

*Close:*
`/c` — Đóng tất cả \+ tắt autotrade
`/c <bot>` — Đóng theo bot

*Positions & Report:*
`/p` — Xem positions đang mở
`/r` — Account report
`/rp` — Report theo bot

*Trade trực tiếp:*
`/buy XAUUSD 0\.1 gold_scalper sl=3280 tp=3320`
`/sell BTCUSD 0\.01 trend_bot`

*System:*
`/accounts` — Xem accounts"#.to_string()
}

async fn reply(bot: &Bot, msg: &Message, text: &str) -> Result<()> {
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

async fn reply_md(bot: &Bot, msg: &Message, text: &str) -> Result<()> {
    // Thử gửi Markdown, nếu lỗi thì gửi plain text
    let result = bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await;
    if result.is_err() {
        // Fallback: gửi plain text
        let plain = text.replace('*', "").replace('`', "").replace('_', "");
        bot.send_message(msg.chat.id, plain).await?;
    }
    Ok(())
}
