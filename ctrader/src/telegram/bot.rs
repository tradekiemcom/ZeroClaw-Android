use std::sync::Arc;
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{
    Message, CallbackQuery, ParseMode, ReplyMarkup,
    KeyboardMarkup, InlineKeyboardMarkup,
};
use tracing::{info, warn};
use chrono::Utc;

use crate::state::AppState;
use crate::models::{
    Account, Bot, Position, ApiClient,
    OrderRequest, OrderAction, TradeSource, TradeSide, AccountScope,
};
use crate::engine::dispatch;
use super::keyboards;
use super::session::{UserSession, CurrentView};

// ── Bot Entry Point ───────────────────────────────────────────────────────────

pub async fn run_telegram_bot(state: Arc<AppState>) {
    let bot = Bot::new(&state.config.telegram_bot_token);
    info!("🤖 Telegram Bot started | Dispatcher mode");

    let state_msg = state.clone();
    let state_cbk = state.clone();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .endpoint(move |bot: Bot, msg: Message| {
                    let s = state_msg.clone();
                    async move { message_handler(bot, msg, s).await }
                }),
        )
        .branch(
            Update::filter_callback_query()
                .endpoint(move |bot: Bot, q: CallbackQuery| {
                    let s = state_cbk.clone();
                    async move { callback_handler(bot, q, s).await }
                }),
        );

    Dispatcher::builder(bot, handler)
        .default_handler(|_| async {})
        .build()
        .dispatch()
        .await;
}

// ── Message Handler ───────────────────────────────────────────────────────────

async fn message_handler(bot: Bot, msg: Message, state: Arc<AppState>) -> Result<()> {
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);
    let text = match msg.text() { Some(t) => t, None => return Ok(()) };

    if !state.is_admin(user_id) {
        bot.send_message(msg.chat.id, "❌ Không có quyền truy cập.").await?;
        return Ok(());
    }

    info!("📨 TG msg | User:{} | {}", user_id, &text[..text.len().min(50)]);

    let session = state.get_user_session(user_id).await;
    handle_text_input(&bot, &msg, text, user_id, session, state).await
}

async fn handle_text_input(
    bot: &Bot,
    msg: &Message,
    text: &str,
    user_id: i64,
    mut session: UserSession,
    state: Arc<AppState>,
) -> Result<()> {
    // ── Navigation commands ────────────────────────────────────────────
    match text {
        "/top" | "⬅️ Top Menu" => {
            session.set_app_scope();
            state.set_user_session(user_id, session.clone()).await;
            send_app_home(bot, msg.chat.id, &state).await?;
            return Ok(());
        }
        "/help" | "❓ Trợ giúp" => {
            let scope = session.scope_badge();
            send_msg_with_keyboard(bot, msg.chat.id, &scope, &help_text(), &session, None).await?;
            return Ok(());
        }
        _ => {}
    }

    // ── Navigation với params ──────────────────────────────────────────
    let parts: Vec<&str> = text.split_whitespace().collect();
    let cmd = parts[0].to_lowercase();

    match cmd.as_str() {
        "/list" | "💼 tài khoản" | "/accounts" => {
            handle_list_accounts(bot, msg.chat.id, &session, &state).await?;
        }
        "/account" => {
            if parts.len() < 2 {
                handle_list_accounts(bot, msg.chat.id, &session, &state).await?;
            } else if let Ok(acc_id) = parts[1].parse::<i64>() {
                enter_account_scope(bot, msg.chat.id, user_id, acc_id, &state, &mut session).await?;
            } else {
                send_error(bot, msg.chat.id, "❌ ID không hợp lệ. Dùng: `/account <id>`", &session).await?;
            }
        }
        "/status" | "📊 status" => {
            let content = format_status(&state).await;
            send_msg_with_keyboard(bot, msg.chat.id, &session.scope_badge(), &content, &session, None).await?;
        }

        // ── Context-sensitive: App scope vs Account scope ───────────────
        "/position" | "/positions" | "📈 positions" => {
            handle_positions(bot, msg.chat.id, user_id, &session, &state).await?;
        }
        "/pending" | "⏳ pending" => {
            handle_pending(bot, msg.chat.id, &session, &state).await?;
        }
        "/bots" | "🤖 bots" => {
            handle_bots(bot, msg.chat.id, &session, &state).await?;
        }
        "/report" | "📑 report" | "📊 report" => {
            handle_report(bot, msg.chat.id, &session, &state).await?;
        }

        // ── Autotrade ───────────────────────────────────────────────────
        "/a" | "🟢 auto on" => {
            handle_autotrade(bot, msg.chat.id, user_id, &session, &state, true).await?;
        }
        "/d" | "🔴 auto off" => {
            handle_autotrade(bot, msg.chat.id, user_id, &session, &state, false).await?;
        }

        // ── Close All ───────────────────────────────────────────────────
        "/c" | "🔴 đóng tất cả" => {
            handle_close_all(bot, msg.chat.id, user_id, &session, &state, "ca").await?;
        }

        // ── Bot ON/OFF ──────────────────────────────────────────────────
        "/on" => {
            if parts.len() < 2 { return send_error(bot, msg.chat.id, "❌ Dùng `/on <bot_id>`", &session).await; }
            let req = make_req(&session, parts[1].to_string(), OrderAction::EnableBot);
            run_and_reply(bot, msg.chat.id, user_id, &session, req, &state).await?;
        }
        "/off" => {
            if parts.len() < 2 { return send_error(bot, msg.chat.id, "❌ Dùng `/off <bot_id>`", &session).await; }
            let req = make_req(&session, parts[1].to_string(), OrderAction::DisableBot);
            run_and_reply(bot, msg.chat.id, user_id, &session, req, &state).await?;
        }

        // ── Trade commands ──────────────────────────────────────────────
        "/buy" => {
            let result = parse_and_trade(user_id, &session, TradeSide::Buy, &parts, &state).await;
            send_trade_reply(bot, msg.chat.id, &session, &result).await?;
        }
        "/sell" => {
            let result = parse_and_trade(user_id, &session, TradeSide::Sell, &parts, &state).await;
            send_trade_reply(bot, msg.chat.id, &session, &result).await?;
        }

        // ── API Key management (app scope only) ─────────────────────────
        "/key" | "🔑 api keys" => {
            handle_key_command(bot, msg.chat.id, &parts, &session, &state).await?;
        }

        // ── Account info ────────────────────────────────────────────────
        "ℹ️ thông tin" => {
            handle_account_info(bot, msg.chat.id, &session, &state).await?;
        }

        // ── Refresh ─────────────────────────────────────────────────────
        "🔄 refresh" => {
            // Re-send current context view
            match session.current_view {
                CurrentView::Positions => handle_positions(bot, msg.chat.id, user_id, &session, &state).await?,
                CurrentView::Bots => handle_bots(bot, msg.chat.id, &session, &state).await?,
                CurrentView::Report => handle_report(bot, msg.chat.id, &session, &state).await?,
                CurrentView::Pending => handle_pending(bot, msg.chat.id, &session, &state).await?,
                _ => send_app_home(bot, msg.chat.id, &state).await?,
            }
        }

        _ => {
            send_error(bot, msg.chat.id, &format!("❓ Không hiểu lệnh: `{}`\nDùng ❓ Trợ giúp để xem danh sách.", escape_md(&cmd)), &session).await?;
        }
    }

    Ok(())
}

// ── Callback Handler ──────────────────────────────────────────────────────────

async fn callback_handler(bot: Bot, q: CallbackQuery, state: Arc<AppState>) -> Result<()> {
    let user_id = q.from.id.0 as i64;
    let data = match &q.data { Some(d) => d.clone(), None => return Ok(()) };

    info!("📲 TG callback | User:{} | {}", user_id, data);

    // Ack the button click
    bot.answer_callback_query(&q.id).await?;

    if !state.is_admin(user_id) {
        return Ok(());
    }

    let chat_id = match &q.message {
        Some(m) => m.chat().id,
        None => return Ok(()),
    };

    let mut session = state.get_user_session(user_id).await;

    // Parse callback data
    let parts: Vec<&str> = data.splitn(4, ':').collect();
    let result = match parts.as_slice() {
        // ── Navigation ─────────────────────────────────────────────────
        ["top"] | ["app", "home"] => {
            session.set_app_scope();
            state.set_user_session(user_id, session.clone()).await;
            send_app_home(&bot, chat_id, &state).await?;
            return Ok(());
        }
        ["app", "list"] => {
            handle_list_accounts(&bot, chat_id, &session, &state).await?;
            return Ok(());
        }
        ["cancel"] => {
            edit_or_send(&bot, &q, "❌ Đã hủy thao tác.").await?;
            return Ok(());
        }

        // ── Select account ─────────────────────────────────────────────
        ["acc", "sel", acc_id_str] => {
            if let Ok(acc_id) = acc_id_str.parse::<i64>() {
                enter_account_scope(&bot, chat_id, user_id, acc_id, &state, &mut session).await?;
            }
            return Ok(());
        }

        // ── Account info ───────────────────────────────────────────────
        ["acc", "inf", acc_id_str] => {
            if let Ok(acc_id) = acc_id_str.parse::<i64>() {
                ensure_account_scope(&mut session, acc_id, &state).await;
                state.set_user_session(user_id, session.clone()).await;
                handle_account_info_by_id(&bot, chat_id, acc_id, &session, &state).await?;
            }
            return Ok(());
        }

        // ── Positions ───────────────────────────────────────────────────

        // App: all positions
        ["app", "pos", action] => {
            let msg = handle_position_action(user_id, None, action, &session, &state).await;
            edit_or_send(&bot, &q, &msg).await?;
            // Re-send positions view after action
            handle_positions(&bot, chat_id, user_id, &session, &state).await?;
            return Ok(());
        }

        // Account: positions view
        ["acc", "pos", acc_id_str] if !matches!(acc_id_str, &"cp"|&"cl"|&"cb"|&"cs"|&"ca"|&"ref") => {
            if let Ok(acc_id) = acc_id_str.parse::<i64>() {
                ensure_account_scope(&mut session, acc_id, &state).await;
                state.set_user_session(user_id, session.clone()).await;
                handle_positions(&bot, chat_id, user_id, &session, &state).await?;
            }
            return Ok(());
        }

        // Account: position actions (4-part: acc:pos:action:acc_id)
        ["acc", "pos", action, acc_id_str] => {
            if let Ok(acc_id) = acc_id_str.parse::<i64>() {
                let msg = handle_position_action(user_id, Some(acc_id), action, &session, &state).await;
                edit_or_send(&bot, &q, &msg).await?;
                // Refresh positions
                ensure_account_scope(&mut session, acc_id, &state).await;
                state.set_user_session(user_id, session.clone()).await;
                handle_positions(&bot, chat_id, user_id, &session, &state).await?;
            }
            return Ok(());
        }

        // ── Pending ────────────────────────────────────────────────────
        ["pnd", action, acc_id_str] => {
            if let Ok(acc_id) = acc_id_str.parse::<i64>() {
                let msg = handle_pending_action(user_id, acc_id, action, &session, &state).await;
                edit_or_send(&bot, &q, &msg).await?;
                ensure_account_scope(&mut session, acc_id, &state).await;
                state.set_user_session(user_id, session.clone()).await;
                handle_pending(&bot, chat_id, &session, &state).await?;
            }
            return Ok(());
        }

        // ── Autotrade ──────────────────────────────────────────────────
        ["acc", "ao", acc_id_str] => {
            if let Ok(acc_id) = acc_id_str.parse::<i64>() {
                ensure_account_scope(&mut session, acc_id, &state).await;
                state.set_user_session(user_id, session.clone()).await;
                handle_autotrade(&bot, chat_id, user_id, &session, &state, true).await?;
            }
            return Ok(());
        }
        ["acc", "ad", acc_id_str] => {
            if let Ok(acc_id) = acc_id_str.parse::<i64>() {
                ensure_account_scope(&mut session, acc_id, &state).await;
                state.set_user_session(user_id, session.clone()).await;
                handle_autotrade(&bot, chat_id, user_id, &session, &state, false).await?;
            }
            return Ok(());
        }

        // ── Bots ───────────────────────────────────────────────────────
        ["acc", "bot", acc_id_str] => {
            if let Ok(acc_id) = acc_id_str.parse::<i64>() {
                ensure_account_scope(&mut session, acc_id, &state).await;
                state.set_user_session(user_id, session.clone()).await;
                handle_bots(&bot, chat_id, &session, &state).await?;
            }
            return Ok(());
        }
        ["acc", action @ ("bon" | "bof"), acc_bot] => {
            // Format: acc:bon:accid:botid combined as acc_bot
            // Actually split is acc:bon:123_botid → need different parse
            // Using _ separator for combined acc+bot
            let ab: Vec<&str> = acc_bot.splitn(2, '_').collect();
            if ab.len() == 2 {
                if let Ok(acc_id) = ab[0].parse::<i64>() {
                    let bot_id = ab[1];
                    let enabled = *action == "bon";
                    let found = state.set_bot_enabled(bot_id, enabled).await.unwrap_or(false);
                    let msg = if found {
                        format!("{} Bot `{}` đã {}", if enabled { "✅" } else { "⏸️" }, bot_id, if enabled { "BẬT" } else { "TẮT" })
                    } else {
                        format!("❌ Không tìm thấy bot `{}`", bot_id)
                    };
                    edit_or_send(&bot, &q, &msg).await?;
                    ensure_account_scope(&mut session, acc_id, &state).await;
                    state.set_user_session(user_id, session.clone()).await;
                    handle_bots(&bot, chat_id, &session, &state).await?;
                }
            }
            return Ok(());
        }

        // ── Report ─────────────────────────────────────────────────────
        ["rpt", "all", acc_id_str] => {
            let acc_id = acc_id_str.parse::<i64>().ok().filter(|&id| id != 0);
            handle_report_display(&bot, chat_id, acc_id, None, &session, &state).await?;
            return Ok(());
        }

        _ => {
            edit_or_send(&bot, &q, "⚠️ Callback không xác định.").await?;
            return Ok(());
        }
    };

    Ok(())
}

// ── View Handlers ─────────────────────────────────────────────────────────────

/// App home screen
async fn send_app_home(bot: &Bot, chat_id: ChatId, state: &Arc<AppState>) -> Result<()> {
    let status = format_status(state).await;
    let text = format!(
        "🌐 *APP SCOPE* — Tất cả tài khoản\n━━━━━━━━━━━━━━━━━━━━\n{}",
        status
    );
    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboards::app_reply_keyboard())
        .await?;
    Ok(())
}

/// Danh sách tài khoản + inline keyboard chọn
async fn handle_list_accounts(
    bot: &Bot, chat_id: ChatId,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let accounts = state.accounts.read().await;
    let acc_list: Vec<Account> = accounts.values().cloned().collect();
    drop(accounts);

    let scope = session.scope_badge();
    let mut content = "💼 *Danh sách tài khoản*\n━━━━━━━━━━━━━━━━━━━━\n".to_string();

    if acc_list.is_empty() {
        content.push_str("❌ Chưa có tài khoản nào\\.\nDùng `/account add` để thêm\\.");
    } else {
        for acc in &acc_list {
            let atype = if acc.is_real() { "🔴 REAL" } else { "🔵 DEMO" };
            let auto = if acc.autotrade { "🟢" } else { "⏸️" };
            content.push_str(&format!(
                "{} \\#{} *{}* \\[{}\\]\n   Balance: `${:.2}` \\| Equity: `${:.2}` \\| P&L: `{:+.2}`\n",
                auto, acc.id, escape_md(&acc.name), atype,
                acc.balance, acc.equity, acc.daily_pnl
            ));
        }
        content.push_str("\n👇 *Chọn tài khoản để vào quản lý chi tiết:*");
    }

    let inline_kb = keyboards::accounts_list_keyboard(&acc_list);
    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", scope, content);

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

/// Vào Account scope
async fn enter_account_scope(
    bot: &Bot, chat_id: ChatId, user_id: i64,
    acc_id: i64, state: &Arc<AppState>, session: &mut UserSession,
) -> Result<()> {
    let accounts = state.accounts.read().await;
    let acc = match accounts.get(&acc_id) {
        Some(a) => a.clone(),
        None => {
            drop(accounts);
            bot.send_message(chat_id, format!("❌ Không tìm thấy account \\#{}\\.", acc_id))
                .parse_mode(ParseMode::MarkdownV2).await?;
            return Ok(());
        }
    };
    drop(accounts);

    session.set_account_scope(acc_id, acc.name.clone());
    state.set_user_session(user_id, session.clone()).await;

    let atype = if acc.is_real() { "🔴 REAL" } else { "🔵 DEMO" };
    let auto = if acc.autotrade { "🟢 Autotrade ON" } else { "⏸️ Autotrade OFF" };

    let text = format!(
        "💼 *{}* \\[\\#{}\\] — {}\n━━━━━━━━━━━━━━━━━━━━\n\
        💰 Balance: `${:.2}`\n\
        📊 Equity: `${:.2}` \\| Float: `{:+.2}`\n\
        📅 Daily P&L: `{:+.2}`\n\
        {}\n━━━━━━━━━━━━━━━━━━━━\n\
        ✅ Đang quản lý account này\\. Dùng bàn phím bên dưới\\.",
        escape_md(&acc.name), acc_id, atype,
        acc.balance, acc.equity, acc.float_profit,
        acc.daily_pnl, auto
    );

    let inline_kb = keyboards::account_info_inline_keyboard(acc_id);
    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    // Send account reply keyboard
    bot.send_message(chat_id, "📋 Bàn phím account đã kích hoạt\\.")
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboards::account_reply_keyboard())
        .await?;
    Ok(())
}

/// Account info (từ reply keyboard)
async fn handle_account_info(
    bot: &Bot, chat_id: ChatId,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    match session.account_id {
        Some(acc_id) => handle_account_info_by_id(bot, chat_id, acc_id, session, state).await,
        None => handle_list_accounts(bot, chat_id, session, state).await,
    }
}

async fn handle_account_info_by_id(
    bot: &Bot, chat_id: ChatId, acc_id: i64,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let accounts = state.accounts.read().await;
    let Some(acc) = accounts.get(&acc_id).cloned() else {
        drop(accounts);
        return send_error(bot, chat_id, "❌ Không tìm thấy account\\.", session).await;
    };
    drop(accounts);

    let positions = state.get_open_positions().await;
    let acc_positions: Vec<_> = positions.iter().filter(|p| p.account_id == acc_id).collect();
    let atype = if acc.is_real() { "🔴 REAL" } else { "🔵 DEMO" };
    let auto = if acc.autotrade { "🟢 ON" } else { "🔴 OFF" };

    let text = format!(
        "{}\n━━━━━━━━━━━━━━━━━━━━\n\
        🏦 *Thông tin tài khoản*\n\n\
        🆔 `{}` \\| {} \\| Auto: {}\n\
        💰 Balance: `${:.2}`\n\
        📊 Equity: `${:.2}`\n\
        Float P&L: `{:+.2}`\n\
        Daily P&L: `{:+.2}`\n\
        📈 Positions đang mở: {}\n\
        📊 Daily target: `${:.2}` \\| Max loss: `${:.2}`",
        session.scope_badge(), acc_id, atype, auto,
        acc.balance, acc.equity, acc.float_profit, acc.daily_pnl,
        acc_positions.len(), acc.daily_target_profit, acc.daily_max_loss
    );

    let inline_kb = keyboards::account_info_inline_keyboard(acc_id);
    send_msg_with_keyboard(bot, chat_id, "", &text, session, Some(inline_kb)).await
}

/// Handle positions view
async fn handle_positions(
    bot: &Bot, chat_id: ChatId, user_id: i64,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    // Update session view
    let mut s = session.clone();
    s.current_view = CurrentView::Positions;
    state.set_user_session(user_id, s.clone()).await;

    let all_positions = state.get_open_positions().await;
    let positions: Vec<_> = match session.account_id {
        Some(acc_id) => all_positions.into_iter().filter(|p| p.account_id == acc_id).collect(),
        None => all_positions,
    };

    let float_pnl: f64 = positions.iter().map(|p| p.pnl).sum();
    let float_icon = if float_pnl >= 0.0 { "🟢" } else { "🔴" };

    let mut content = format!("📈 *Positions đang mở* \\({} lệnh\\)\n", positions.len());
    content.push_str("━━━━━━━━━━━━━━━━━━━━\n");

    if positions.is_empty() {
        content.push_str("📭 Không có lệnh nào đang mở\\.");
    } else {
        for pos in &positions {
            let pnl_icon = if pos.pnl >= 0.0 { "💰" } else { "📉" };
            let src_emoji = crate::models::TradeSource::from_str(&pos.source).emoji();
            content.push_str(&format!(
                "{}{} `{}` {} {:.2}L @ `{:.5}`\n   {}: `{:+.2}` \\| Acc\\#{}",
                src_emoji, pnl_icon, escape_md(&pos.bot_id),
                pos.side, pos.volume, pos.open_price,
                if pos.pnl >= 0.0 { "Lời" } else { "Lỗ" }, pos.pnl,
                pos.account_id
            ));
            content.push('\n');
        }
        content.push_str("━━━━━━━━━━━━━━━━━━━━\n");
        content.push_str(&format!("{} Float P&L: `{:+.2} USD`", float_icon, float_pnl));
    }

    let inline_kb = match session.account_id {
        Some(acc_id) => keyboards::positions_account_inline_keyboard(acc_id),
        None => keyboards::positions_app_inline_keyboard(),
    };

    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), content);
    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

/// Handle pending orders view
async fn handle_pending(
    bot: &Bot, chat_id: ChatId,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let acc_id = session.account_id.unwrap_or(0);
    let content = "⏳ *Pending Orders*\n━━━━━━━━━━━━━━━━━━━━\n📭 Mock mode: Chưa kết nối cTrader thật\\.".to_string();

    let inline_kb = keyboards::pending_account_inline_keyboard(acc_id);
    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), content);
    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

/// Handle bots view
async fn handle_bots(
    bot: &Bot, chat_id: ChatId,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let bots_map = state.bots.read().await;
    let mut bots: Vec<_> = bots_map.values().cloned().collect();
    drop(bots_map);
    bots.sort_by(|a, b| a.id.cmp(&b.id));

    let mut content = format!("🤖 *Bots* \\({} bots\\)\n━━━━━━━━━━━━━━━━━━━━\n", bots.len());
    for bot_item in &bots {
        let st = if bot_item.enabled { "✅ ON" } else { "⏸️ OFF" };
        content.push_str(&format!(
            "{} `{}` — {} \\| P&L: `{:+.2}` \\| Trades: {}\n",
            st, escape_md(&bot_item.id), escape_md(&bot_item.symbol),
            bot_item.daily_pnl, bot_item.trade_count_today
        ));
    }

    let inline_kb = keyboards::bots_inline_keyboard(&bots, session.account_id);
    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), content);
    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

/// Handle report view
async fn handle_report(
    bot: &Bot, chat_id: ChatId,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    handle_report_display(bot, chat_id, session.account_id, None, session, state).await
}

async fn handle_report_display(
    bot: &Bot, chat_id: ChatId,
    acc_id: Option<i64>, bot_id: Option<&str>,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let accounts = state.accounts.read().await;
    let bots_map = state.bots.read().await;
    let bots: Vec<_> = bots_map.values().cloned().collect();
    drop(bots_map);

    let mut content = "📊 *Report*\n━━━━━━━━━━━━━━━━━━━━\n".to_string();

    match acc_id {
        Some(id) => {
            if let Some(acc) = accounts.get(&id) {
                let atype = if acc.is_real() { "REAL" } else { "DEMO" };
                content.push_str(&format!(
                    "Account \\#{} *{}* \\[{}\\]\n\
                    Balance: `${:.2}` \\| Equity: `${:.2}`\n\
                    Float: `{:+.2}` \\| Daily P&L: `{:+.2}`\n",
                    id, escape_md(&acc.name), atype,
                    acc.balance, acc.equity, acc.float_profit, acc.daily_pnl
                ));
            }
        }
        None => {
            let total_balance: f64 = accounts.values().filter(|a| a.is_real()).map(|a| a.balance).sum();
            let total_pnl: f64 = accounts.values().map(|a| a.daily_pnl).sum();
            let total_float: f64 = accounts.values().filter(|a| a.is_real()).map(|a| a.float_profit).sum();
            content.push_str(&format!(
                "Tổng Real Balance: `${:.2}`\nTotal Daily P&L: `{:+.2}`\nFloat P&L: `{:+.2}`\n\n",
                total_balance, total_pnl, total_float
            ));
            content.push_str("📈 *Theo Bots:*\n");
            for b in &bots {
                let st = if b.enabled { "✅" } else { "⏸️" };
                content.push_str(&format!(
                    "{} `{}` {} — P&L: `{:+.2}` \\| Trades: {}\n",
                    st, escape_md(&b.id), escape_md(&b.symbol), b.daily_pnl, b.trade_count_today
                ));
            }
        }
    }

    drop(accounts);

    let inline_kb = keyboards::report_inline_keyboard(&bots, acc_id);
    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), content);
    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

/// Autotrade toggle
async fn handle_autotrade(
    bot: &Bot, chat_id: ChatId, user_id: i64,
    session: &UserSession, state: &Arc<AppState>, enable: bool
) -> Result<()> {
    let icon = if enable { "🟢" } else { "🔴" };
    let verb = if enable { "BẬT" } else { "TẮT" };

    let req = make_req(session, "system".to_string(), if enable { OrderAction::EnableAutotrade } else { OrderAction::DisableAutotrade });
    match dispatch(state.clone(), req).await {
        Ok(_) => {
            let text = format!(
                "{}\n━━━━━━━━━━━━━━━━━━━━\n{} Autotrade {} \\!",
                session.scope_badge(), icon, verb
            );
            bot.send_message(chat_id, text)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
        Err(e) => { send_error(bot, chat_id, &format!("❌ {}", e), session).await?; }
    }
    Ok(())
}

/// Close all positions
async fn handle_close_all(
    bot: &Bot, chat_id: ChatId, user_id: i64,
    session: &UserSession, state: &Arc<AppState>, action: &str
) -> Result<()> {
    let req = make_req(session, "system".to_string(), OrderAction::CloseAll);
    match dispatch(state.clone(), req).await {
        Ok(result) => {
            let text = format!(
                "{}\n━━━━━━━━━━━━━━━━━━━━\n🔴 Đã đóng {} lệnh\\. Autotrade TẮT\\.",
                session.scope_badge(), result.success_count
            );
            bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2).await?;
            // Trade events → notify group (handled in engine)
        }
        Err(e) => { send_error(bot, chat_id, &format!("❌ {}", e), session).await?; }
    }
    Ok(())
}

// ── Position Actions (from inline keyboard) ───────────────────────────────────

async fn handle_position_action(
    user_id: i64, acc_id: Option<i64>,
    action: &str, session: &UserSession, state: &Arc<AppState>
) -> String {
    if action == "ref" { return "🔄 Refreshing...".to_string(); }

    let positions = state.get_open_positions().await;
    let target_positions: Vec<_> = match acc_id {
        Some(id) => positions.into_iter().filter(|p| p.account_id == id).collect(),
        None => positions,
    };

    let to_close: Vec<_> = match action {
        "cp" => target_positions.iter().filter(|p| p.pnl > 0.0).collect(),  // close profit
        "cl" => target_positions.iter().filter(|p| p.pnl < 0.0).collect(),  // close loss
        "cb" => target_positions.iter().filter(|p| p.side.to_uppercase() == "BUY").collect(),
        "cs" => target_positions.iter().filter(|p| p.side.to_uppercase() == "SELL").collect(),
        "ca" => target_positions.iter().collect(),  // close all
        _ => return "❌ Action không xác định".to_string(),
    };

    if to_close.is_empty() {
        return "📭 Không có lệnh phù hợp để đóng.".to_string();
    }

    // Execute close via cTrader
    let acc_ids: Vec<i64> = to_close.iter().map(|p| p.account_id).collect::<std::collections::HashSet<_>>().into_iter().collect();
    let count = to_close.len();

    // Close in DB
    for pos in &to_close {
        let _ = crate::storage::close_positions_by_bot(&state.db, &pos.bot_id).await;
    }

    // Update memory
    {
        let mut positions_lock = state.positions.write().await;
        for pos in positions_lock.iter_mut() {
            if to_close.iter().any(|p| p.id == pos.id) {
                pos.status = crate::models::PositionStatus::Closed;
                pos.closed_at = Some(Utc::now());
            }
        }
    }

    format!("✅ Đã đóng {} lệnh.", count)
}

async fn handle_pending_action(
    user_id: i64, acc_id: i64, action: &str,
    session: &UserSession, state: &Arc<AppState>
) -> String {
    if action == "ref" { return "🔄 Refreshing...".to_string(); }
    format!("🧪 Mock: Hủy pending [{}] cho account #{}", action, acc_id)
}

// ── Key management ────────────────────────────────────────────────────────────

async fn handle_key_command(
    bot: &Bot, chat_id: ChatId, parts: &[&str],
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    if parts.len() < 2 {
        let clients = state.list_api_clients().await;
        let mut content = format!("🔑 *API Keys* \\({}\\)\n━━━━━━━━━━━━━━━━━━━━\n", clients.len());
        for c in &clients {
            let st = if c.enabled { "🟢" } else { "🔴" };
            content.push_str(&format!(
                "{} `{}` — *{}* \\[{}\\]\n   Key: `{}…` \\| {} reqs\n",
                st, &c.id[..8], escape_md(&c.name), c.source,
                &c.api_key[..16], c.request_count
            ));
        }
        content.push_str("\n`/key add <name> <source>` \\| `/key off/del <id>`");
        let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), content);
        bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2).await?;
        return Ok(());
    }

    let msg = match parts[1].to_lowercase().as_str() {
        "add" => {
            if parts.len() < 3 { "❌ Dùng: `/key add <tên> [source]`".to_string() }
            else {
                let name = parts[2].to_string();
                let source = parts.get(3).map(|s| s.to_uppercase()).unwrap_or("API".to_string());
                let client = ApiClient::new(name.clone(), source.clone());
                let key = client.api_key.clone();
                let id = client.id[..8].to_string();
                match state.add_api_client(client).await {
                    Ok(_) => format!("✅ Key mới\\!\n📛 {}\n📡 {}\n🆔 `{}`\n🔑 `{}`\n⚠️ Lưu key lại\\!", escape_md(&name), source, id, key),
                    Err(e) => format!("❌ {}", e),
                }
            }
        }
        "del" => {
            if parts.len() < 3 { "❌ `/key del <id>`".to_string() }
            else {
                match state.delete_api_client(parts[2]).await {
                    Ok(true) => format!("✅ Đã xóa `{}`", &parts[2][..8.min(parts[2].len())]),
                    Ok(false) => "❌ Không tìm thấy key".to_string(),
                    Err(e) => format!("❌ {}", e),
                }
            }
        }
        "on" => {
            if parts.len() < 3 { "❌ `/key on <id>`".to_string() }
            else {
                match state.set_client_enabled(parts[2], true).await {
                    Ok(true) => format!("🟢 Key `{}…` BẬT", &parts[2][..8.min(parts[2].len())]),
                    Ok(false) => "❌ Không tìm thấy key".to_string(),
                    Err(e) => format!("❌ {}", e),
                }
            }
        }
        "off" => {
            if parts.len() < 3 { "❌ `/key off <id>`".to_string() }
            else {
                match state.set_client_enabled(parts[2], false).await {
                    Ok(true) => format!("🔴 Key `{}…` TẮT", &parts[2][..8.min(parts[2].len())]),
                    Ok(false) => "❌ Không tìm thấy key".to_string(),
                    Err(e) => format!("❌ {}", e),
                }
            }
        }
        _ => "❌ Dùng: `/key list|add|del|on|off`".to_string(),
    };

    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), escape_md(&msg));
    bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}

// ── Trade helpers ─────────────────────────────────────────────────────────────

async fn parse_and_trade(
    _user_id: i64, session: &UserSession, side: TradeSide,
    parts: &[&str], state: &Arc<AppState>
) -> String {
    if parts.len() < 4 {
        let cmd = if side == TradeSide::Buy { "buy" } else { "sell" };
        return format!("❌ Format:\n`/{} SYMBOL VOLUME BOT_ID [sl=X] [tp=X]`", cmd);
    }
    let symbol = parts[1].to_uppercase();
    let volume: f64 = match parts[2].parse() { Ok(v) => v, Err(_) => return "❌ Volume không hợp lệ".to_string() };
    let bot_id = parts[3].to_string();
    let mut sl: Option<f64> = None;
    let mut tp: Option<f64> = None;
    for &part in &parts[4..] {
        if let Some(v) = part.strip_prefix("sl=") { sl = v.parse().ok(); }
        else if let Some(v) = part.strip_prefix("tp=") { tp = v.parse().ok(); }
    }
    let req = OrderRequest::market_order(TradeSource::Telegram, bot_id, symbol, side, volume, sl, tp);
    match dispatch(state.clone(), req).await {
        Ok(result) => result.to_telegram_msg(),
        Err(e) => format!("❌ {}", e),
    }
}

async fn send_trade_reply(bot: &Bot, chat_id: ChatId, session: &UserSession, msg: &str) -> Result<()> {
    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), escape_md(msg));
    bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}

// ── Public: Notify group (trade events only) ──────────────────────────────────

pub async fn send_trade_event_to_group(bot: &teloxide::Bot, chat_id: &str, event_type: &str, pos: &Position) -> Result<()> {
    if chat_id.is_empty() { return Ok(()); }

    let icon = match event_type { "OPEN" => "🟢", "CLOSE" => "🔴", "MODIFY" => "✏️", _ => "⚪" };
    let src_emoji = crate::models::TradeSource::from_str(&pos.source).emoji();
    let text = format!(
        "{} *{}* {} `{}`\n\
        💼 Acc\\#{} \\| Bot: `{}`\n\
        📈 {} {} @ `{:.5}`\n\
        💰 Vol: `{:.2}` \\| P&L: `{:+.2}`",
        icon, event_type, src_emoji, escape_md(&pos.source),
        pos.account_id, escape_md(&pos.bot_id),
        pos.side.to_uppercase(), escape_md(&pos.symbol), pos.open_price,
        pos.volume, pos.pnl
    );

    send_notify(bot, chat_id, &text).await
}

pub async fn send_notify(bot: &teloxide::Bot, chat_id: &str, text: &str) -> Result<()> {
    use teloxide::types::ChatId;
    if chat_id.is_empty() { return Ok(()); }
    let result = if let Ok(id) = chat_id.trim_start_matches('@').parse::<i64>() {
        bot.send_message(ChatId(id), text).parse_mode(ParseMode::MarkdownV2).await
    } else {
        bot.send_message(chat_id.to_string(), text).parse_mode(ParseMode::MarkdownV2).await
    };
    if let Err(e) = result { warn!("📢 Notify failed: {}", e); }
    Ok(())
}

// ── Status ────────────────────────────────────────────────────────────────────

async fn format_status(state: &Arc<AppState>) -> String {
    let s = state.get_system_status().await;
    let uptime = AppState::format_uptime(s.uptime_secs);
    let mode = if state.config.is_mock() { "🧪 Mock" } else { "🔴 Live" };
    let float_icon = if s.total_float_profit >= 0.0 { "🟢" } else { "🔴" };
    let daily_icon = if s.total_daily_pnl >= 0.0 { "📈" } else { "📉" };

    format!(
        "🔌 {} \\| ⏱ {}\n\n\
        💼 Tài khoản: {} \\(🔴{}R 🔵{}D\\) Online: {}/{}\n\
        🟢 Auto: {}/{}\n\n\
        💰 Balance Real: `${:.0}`\n\
        📊 Equity: `${:.0}` \\| {} Float: `{:+.0}`\n\
        {} Daily P&L: `{:+.0}`\n\n\
        🤖 Bots: {}/{} \\| 📈 Positions: {}\n\
        🔑 Keys: {}/{}",
        mode, uptime,
        s.total_accounts, s.real_accounts, s.demo_accounts,
        s.connected_accounts, s.total_accounts,
        s.active_accounts, s.total_accounts,
        s.total_real_balance, s.total_real_equity,
        float_icon, s.total_float_profit,
        daily_icon, s.total_daily_pnl,
        s.active_bots, s.total_bots, s.open_positions,
        s.active_api_clients, s.total_api_clients,
    )
}

// ── Utility helpers ───────────────────────────────────────────────────────────

/// Tạo OrderRequest từ session (scope-aware)
fn make_req(session: &UserSession, bot_id: String, action: OrderAction) -> OrderRequest {
    let mut req = OrderRequest::new(TradeSource::Telegram, bot_id, action);
    if let Some(acc_id) = session.account_id {
        req.account_scope = AccountScope::Single;
        req.account_ids = vec![acc_id];
    }
    req
}

async fn run_and_reply(
    bot: &Bot, chat_id: ChatId, user_id: i64,
    session: &UserSession, req: OrderRequest, state: &Arc<AppState>
) -> Result<()> {
    let result_msg = match dispatch(state.clone(), req).await {
        Ok(r) => r.to_telegram_msg(),
        Err(e) => format!("❌ {}", e),
    };
    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), escape_md(&result_msg));
    bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}

async fn send_msg_with_keyboard(
    bot: &Bot, chat_id: ChatId, scope_header: &str, content: &str,
    session: &UserSession, inline_kb: Option<InlineKeyboardMarkup>
) -> Result<()> {
    let text = if scope_header.is_empty() {
        content.to_string()
    } else {
        format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), content)
    };

    let mut builder = bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2);
    if let Some(kb) = inline_kb {
        builder = builder.reply_markup(ReplyMarkup::InlineKeyboard(kb));
    }
    builder.await?;
    Ok(())
}

async fn send_error(bot: &Bot, chat_id: ChatId, msg: &str, session: &UserSession) -> Result<()> {
    let text = format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), msg);
    bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}

/// Edit message from callback query, or send new if can't edit
async fn edit_or_send(bot: &Bot, q: &CallbackQuery, text: &str) -> Result<()> {
    use teloxide::types::ChatId;
    if let Some(msg) = &q.message {
        let _ = bot.edit_message_text(msg.chat().id, msg.id(), text)
            .parse_mode(ParseMode::MarkdownV2)
            .await;
    }
    Ok(())
}

async fn ensure_account_scope(session: &mut UserSession, acc_id: i64, state: &Arc<AppState>) {
    if session.account_id != Some(acc_id) {
        let accounts = state.accounts.read().await;
        let name = accounts.get(&acc_id).map(|a| a.name.clone()).unwrap_or_default();
        drop(accounts);
        session.set_account_scope(acc_id, name);
    }
}

fn escape_md(s: &str) -> String {
    s.chars().flat_map(|c| {
        if "_*[]()~`>#+-=|{}.!".contains(c) { vec!['\\', c] } else { vec![c] }
    }).collect()
}

fn help_text() -> String {
    r#"🤖 *iZFx\.Trade v2\.1*

*Navigation:*
`/list` — Danh sách tài khoản
`/account <id>` — Vào quản lý account
`/top` — Về App scope

*System:*
`/status` — Trạng thái hệ thống
📊 Status \\| 💼 Tài khoản \\| 🤖 Bots

*Autotrade:*
`/a` \\| 🟢 Auto ON — Bật
`/d` \\| 🔴 Auto OFF — Tắt

*Trade:*
`/buy XAUUSD 0\.1 bot_id sl=3280 tp=3320`
`/sell BTCUSD 0\.01 bot_id`

*Close:*
`/c` — Đóng tất cả
Hoặc dùng bàn phím 📈 Positions

*API Keys:*
`/key` — Xem keys
`/key add <name> <source>` — Thêm"#.to_string()
}
