use std::sync::Arc;
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{Message, ParseMode};
use tracing::{info, warn};

use crate::state::AppState;
use crate::models::{ApiClient, OrderRequest, OrderAction, TradeSource, TradeSide, AccountScope};
use crate::engine::dispatch;

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
    if !text.starts_with('/') { return Ok(()); }
    if !state.is_admin(user_id) {
        reply(bot, msg, "❌ Bạn không có quyền sử dụng bot này.").await?;
        return Ok(());
    }

    info!("📨 TG cmd | User: {} | {}", user_id, text);
    let response = parse_and_execute(text, state.clone()).await;
    reply_md(bot, msg, &response).await?;
    Ok(())
}

async fn parse_and_execute(text: &str, state: Arc<AppState>) -> String {
    let parts: Vec<&str> = text.split_whitespace().collect();
    let cmd = parts[0].to_lowercase();

    match cmd.as_str() {
        "/help" | "/start" => help_text(),

        // ── Status hệ thống ────────────────────────────────────────────
        "/status" | "/s" => format_status(&state).await,

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
                let bot_id = parts[1].to_string();
                let req = OrderRequest::new(TradeSource::Telegram, bot_id, OrderAction::Close);
                run_dispatch(state, req).await
            } else {
                let req = OrderRequest::new(TradeSource::Telegram, "system".to_string(), OrderAction::CloseAll);
                run_dispatch(state, req).await
            }
        }

        // ── Bot Control ────────────────────────────────────────────────
        "/on" => {
            if parts.len() < 2 { return "❌ `/on <bot_id>`".to_string(); }
            let req = OrderRequest::new(TradeSource::Telegram, parts[1].to_string(), OrderAction::EnableBot);
            run_dispatch(state, req).await
        }
        "/off" => {
            if parts.len() < 2 { return "❌ `/off <bot_id>`".to_string(); }
            let req = OrderRequest::new(TradeSource::Telegram, parts[1].to_string(), OrderAction::DisableBot);
            run_dispatch(state, req).await
        }

        // ── Positions & Report ─────────────────────────────────────────
        "/p" | "/positions" => format_positions(&state).await,
        "/r" | "/report" => state.format_accounts_summary().await,
        "/rp" => state.format_bots_summary().await,
        "/bots" => state.format_bots_summary().await,
        "/accounts" => state.format_accounts_summary().await,

        // ── API Key Management ─────────────────────────────────────────
        "/key" => handle_key_command(&parts, state).await,

        // ── Direct Trade ───────────────────────────────────────────────
        "/buy" => parse_trade(TradeSource::Telegram, TradeSide::Buy, &parts, state).await,
        "/sell" => parse_trade(TradeSource::Telegram, TradeSide::Sell, &parts, state).await,

        _ => format!("❓ Lệnh không xác định: `{}`\nDùng /help để xem danh sách.", cmd),
    }
}

// ── /status ───────────────────────────────────────────────────────────────────

async fn format_status(state: &Arc<AppState>) -> String {
    let s = state.get_system_status().await;
    let uptime = AppState::format_uptime(s.uptime_secs);
    let mode_icon = if state.config.is_mock() { "🧪 Mock" } else { "🔴 Live" };

    // Float P&L icon
    let float_icon = if s.total_float_profit >= 0.0 { "🟢" } else { "🔴" };
    let daily_icon = if s.total_daily_pnl >= 0.0 { "📈" } else { "📉" };

    format!(
        "📊 *iZFx\\.Trade Status*\n\
        ━━━━━━━━━━━━━━━━━━━━\n\
        🔌 Mode: {} \\| ⏱ Uptime: {}\n\n\
        💼 *Tài khoản:*\n\
        • Tổng: {} \\(Real: {} \\| Demo: {}\\)\n\
        • Kết nối: {}/{} \\| Autotrade: {}/{}\n\n\
        💰 *Vốn Quản Lý \\(Real\\):*\n\
        • Balance: `${:.2}`\n\
        • Equity: `${:.2}`\n\
        • {} Float P&L: `${:.2}`\n\
        • {} Daily P&L: `${:.2}`\n\n\
        🤖 *Bots:* {}/{} active\n\
        📈 *Positions đang mở:* {}\n\n\
        🔑 *API Keys:* {}/{} active",
        mode_icon, uptime,
        s.total_accounts, s.real_accounts, s.demo_accounts,
        s.connected_accounts, s.total_accounts,
        s.active_accounts, s.total_accounts,
        s.total_real_balance,
        s.total_real_equity,
        float_icon, s.total_float_profit,
        daily_icon, s.total_daily_pnl,
        s.active_bots, s.total_bots,
        s.open_positions,
        s.active_api_clients, s.total_api_clients,
    )
}

// ── /key management ───────────────────────────────────────────────────────────

async fn handle_key_command(parts: &[&str], state: Arc<AppState>) -> String {
    if parts.len() < 2 {
        return key_help();
    }

    match parts[1].to_lowercase().as_str() {
        "list" | "ls" => {
            let clients = state.list_api_clients().await;
            if clients.is_empty() {
                return "🔑 *API Keys:* Chưa có key nào\\.\n\nDùng `/key add <name> <source>` để thêm\\.".to_string();
            }
            let mut lines = vec![format!("🔑 *API Keys* ({})", clients.len())];
            lines.push("━━━━━━━━━━━━━━━━━━━━".to_string());
            for client in &clients {
                lines.push(client.format_list_item());
            }
            lines.push("━━━━━━━━━━━━━━━━━━━━".to_string());
            lines.push("Quản lý: `/key on|off|del <id>`".to_string());
            lines.join("\n")
        }

        "add" => {
            // /key add <name> <source>
            // source: MT5, TRADINGVIEW, ZEROCLAW, OPENCLAW, WEBHOOK, API, WEB
            if parts.len() < 3 {
                return "❌ Dùng: `/key add <tên> [source]`\n\nVí dụ:\n`/key add MT5_EA MT5`\n`/key add TradingView_Alert TRADINGVIEW`\n`/key add ZeroClaw_Bot ZEROCLAW`".to_string();
            }
            let name = parts[2].to_string();
            let source = parts.get(3).map(|s| s.to_uppercase()).unwrap_or_else(|| "API".to_string());

            let client = ApiClient::new(name.clone(), source.clone());
            let key_display = client.api_key.clone();
            let id_display = client.id[..8].to_string();

            match state.add_api_client(client).await {
                Ok(_) => format!(
                    "✅ *API Key mới đã tạo\\!*\n\n\
                    📛 Tên: `{}`\n\
                    📡 Source: `{}`\n\
                    🆔 ID: `{}`\n\
                    🔑 Key: `{}`\n\n\
                    ⚠️ Lưu key này ngay, không hiển thị lại\\!",
                    escape_md(&name), escape_md(&source), id_display, key_display
                ),
                Err(e) => format!("❌ Lỗi: {}", e),
            }
        }

        "del" | "delete" | "rm" => {
            if parts.len() < 3 {
                return "❌ Dùng: `/key del <id>`".to_string();
            }
            let id = parts[2];
            match state.delete_api_client(id).await {
                Ok(true) => format!("✅ Đã xóa key `{}`", &id[..8.min(id.len())]),
                Ok(false) => format!("❌ Không tìm thấy key ID `{}`", id),
                Err(e) => format!("❌ Lỗi: {}", e),
            }
        }

        "on" => {
            if parts.len() < 3 { return "❌ Dùng: `/key on <id>`".to_string(); }
            let id = parts[2];
            match state.set_client_enabled(id, true).await {
                Ok(true) => format!("🟢 Key `{}...` đã BẬT", &id[..8.min(id.len())]),
                Ok(false) => format!("❌ Không tìm thấy key `{}`", id),
                Err(e) => format!("❌ Lỗi: {}", e),
            }
        }

        "off" => {
            if parts.len() < 3 { return "❌ Dùng: `/key off <id>`".to_string(); }
            let id = parts[2];
            match state.set_client_enabled(id, false).await {
                Ok(true) => format!("🔴 Key `{}...` đã TẮT", &id[..8.min(id.len())]),
                Ok(false) => format!("❌ Không tìm thấy key `{}`", id),
                Err(e) => format!("❌ Lỗi: {}", e),
            }
        }

        _ => key_help(),
    }
}

fn key_help() -> String {
    "🔑 *Quản lý API Keys*\n\n\
    `/key list` — Xem tất cả keys\n\
    `/key add <tên> <source>` — Thêm key mới\n\
    `/key del <id>` — Xóa key\n\
    `/key on <id>` — Bật key\n\
    `/key off <id>` — Tắt key\n\n\
    *Sources hỗ trợ:*\n\
    `MT5` `TRADINGVIEW` `ZEROCLAW` `OPENCLAW` `WEBHOOK` `API` `WEB`".to_string()
}

// ── Positions display ─────────────────────────────────────────────────────────

async fn format_positions(state: &Arc<AppState>) -> String {
    let positions = state.get_open_positions().await;
    if positions.is_empty() {
        return "📭 Không có lệnh nào đang mở\\.".to_string();
    }
    let total_float: f64 = positions.iter().map(|p| p.pnl).sum();
    let float_icon = if total_float >= 0.0 { "🟢" } else { "🔴" };

    let mut lines = vec![format!("📊 *Open Positions* \\({}\\)", positions.len())];
    lines.push("━━━━━━━━━━━━━━━━━━━━".to_string());
    for pos in &positions {
        let source_emoji = TradeSource::from_str(&pos.source).emoji();
        lines.push(format!(
            "{} `{}` {} {:.2}L@{:.5}",
            source_emoji, pos.bot_id, pos.side, pos.volume, pos.open_price
        ));
    }
    lines.push("━━━━━━━━━━━━━━━━━━━━".to_string());
    lines.push(format!("{} Float P&L: `{:.2}`", float_icon, total_float));
    lines.join("\n")
}

// ── Trade parse & dispatch ────────────────────────────────────────────────────

async fn parse_trade(
    source: TradeSource,
    side: TradeSide,
    parts: &[&str],
    state: Arc<AppState>,
) -> String {
    if parts.len() < 4 {
        let cmd = if side == TradeSide::Buy { "buy" } else { "sell" };
        return format!("❌ Format:\n`/{} SYMBOL VOLUME BOT_ID [sl=X] [tp=X]`", cmd);
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
        if let Some(v) = part.strip_prefix("sl=") { sl = v.parse().ok(); }
        else if let Some(v) = part.strip_prefix("tp=") { tp = v.parse().ok(); }
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
    if state.config.telegram_notify_chat_id.is_empty() { return; }
    if let Some(bot) = &state.telegram_bot {
        let source_emoji = TradeSource::from_str(&pos.source).emoji();
        let msg = format!(
            "🟢 *OPEN* {} `{}`\n\
            💼 Account: #{} \\| Bot: `{}`\n\
            📈 {} {} @ `{:.5}`\n\
            💰 Vol: `{:.2}` \\| SL: {} \\| TP: {}",
            source_emoji, pos.source,
            pos.account_id, escape_md(&pos.bot_id),
            pos.side.to_uppercase(), escape_md(&pos.symbol), pos.open_price,
            pos.volume,
            pos.sl.map(|v| format!("`{:.5}`", v)).unwrap_or_else(|| "—".to_string()),
            pos.tp.map(|v| format!("`{:.5}`", v)).unwrap_or_else(|| "—".to_string()),
        );
        let _ = send_notify(bot, &state.config.telegram_notify_chat_id, &msg).await;
    }
}

pub async fn send_notify(bot: &Bot, chat_id: &str, text: &str) -> Result<()> {
    use teloxide::types::ChatId;
    if chat_id.is_empty() { return Ok(()); }

    let result = if let Ok(id) = chat_id.trim_start_matches('@').parse::<i64>() {
        bot.send_message(ChatId(id), text)
            .parse_mode(ParseMode::MarkdownV2)
            .await
    } else {
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

*System:*
`/status` — Trạng thái hệ thống chi tiết
`/accounts` — Danh sách tài khoản
`/bots` — Danh sách bots

*Autotrade:*
`/a` — Bật autotrade tất cả
`/d` — Tắt autotrade tất cả

*Bot Control:*
`/on <bot>` — Bật bot
`/off <bot>` — Tắt bot

*Close:*
`/c` — Đóng tất cả \+ tắt autotrade
`/c <bot>` — Đóng theo bot

*Positions & Report:*
`/p` — Positions đang mở
`/r` — Account report
`/rp` — Report theo bot

*Trade trực tiếp:*
`/buy XAUUSD 0\.1 bot_id sl=3280 tp=3320`
`/sell BTCUSD 0\.01 bot_id`

*API Key Management:*
`/key list` — Xem tất cả keys
`/key add <tên> <source>` — Tạo key mới
`/key del <id>` — Xóa key
`/key on|off <id>` — Bật/tắt key"#.to_string()
}

// Escape ký tự đặc biệt cho MarkdownV2
fn escape_md(s: &str) -> String {
    s.chars().flat_map(|c| {
        if "_*[]()~`>#+-=|{}.!".contains(c) {
            vec!['\\', c]
        } else {
            vec![c]
        }
    }).collect()
}

async fn reply(bot: &Bot, msg: &Message, text: &str) -> Result<()> {
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

async fn reply_md(bot: &Bot, msg: &Message, text: &str) -> Result<()> {
    let result = bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await;
    if result.is_err() {
        // Fallback: plain text (strip markdown)
        let plain = text.chars().filter(|&c| c != '*' && c != '`' && c != '\\').collect::<String>();
        let _ = bot.send_message(msg.chat.id, plain).await;
    }
    Ok(())
}
