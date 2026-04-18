use std::sync::Arc;
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::types::{
    Message, CallbackQuery, ParseMode, ReplyMarkup,
    KeyboardMarkup, InlineKeyboardMarkup, BotCommand,
};
use tracing::{info, warn};
use chrono::Utc;

use crate::state::AppState;
use crate::models::{
    Account, Bot as TradingBot, Position, ApiClient,
    OrderRequest, OrderAction, TradeSource, TradeSide, AccountScope,
};
use crate::engine::{dispatch, IzParser};
use super::keyboards;
use super::session::{UserSession, CurrentView};

// ── Bot Entry Point ───────────────────────────────────────────────────────────

pub async fn run_telegram_bot(state: Arc<AppState>) {
    let token = &state.config.telegram_bot_token;
    
    // Check token config
    if token.is_empty() || token == "your_bot_token_here" {
        warn!("TELEGRAM_BOT_TOKEN not configured. Skipping bot start.");
        return;
    }

    let bot = Bot::new(token);
    
    info!("Starting Telegram Bot...");
    match bot.get_me().await {
        Ok(me) => info!("Telegram Bot authenticated: @{}", me.user.username.unwrap_or_default()),
        Err(e) => {
            warn!("Failed to authenticate Telegram Bot: {}. Bot will not run.", e);
            return;
        }
    }

    // Set Menu Commands (No Icons - Pro)
    let commands = vec![
        BotCommand::new("start", "Home / Menu"),
        BotCommand::new("list", "Account List"),
        BotCommand::new("status", "System Status"),
        BotCommand::new("bots", "Bots Management"),
        BotCommand::new("positions", "Positions"),
        BotCommand::new("pending", "Pending Orders"),
        BotCommand::new("report", "Trading Report"),
        BotCommand::new("a", "AutoTrade ON"),
        BotCommand::new("d", "AutoTrade OFF"),
        BotCommand::new("on", "Enable Bot [id]"),
        BotCommand::new("off", "Disable Bot [id]"),
        BotCommand::new("key", "API Keys"),
        BotCommand::new("top", "App Scope"),
        BotCommand::new("help", "User Guide"),
    ];
    let _ = bot.set_my_commands(commands).await;

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
        bot.send_message(msg.chat.id, "Access Denied.").await?;
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
    // ── 1. Check if it's a # trading command ────────────────────────────
    if text.starts_with('#') {
        if let Some(mut req) = IzParser::parse(text, TradeSource::Telegram) {
            // Apply current session scope
            if !session.is_app_scope() {
                req.account_scope = crate::models::AccountScope::Single;
                req.account_ids = session.account_id.into_iter().collect();
            }
            
            let res = crate::engine::dispatch(state.clone(), req).await?;
            send_trade_reply(bot, msg.chat.id, &session, &res.messages.join("\n")).await?;
            return Ok(());
        }
    }

    let badge_text = session.scope_badge().replace('*', "").replace('\\', "");
    let text_to_match = if text.contains(&badge_text) { 
        "🔄 refresh" 
    } else { 
        text 
    };

    // ── 2. Navigation & Direct Commands ──────────────────────────────────
    match text_to_match {
        "/start" | "Top" | "APP Scope" => {
            session.set_app_scope();
            state.set_user_session(user_id, session.clone()).await;
            send_app_home(bot, msg.chat.id, &session, state.clone()).await?;
            return Ok(());
        }
        "Back" => {
            if !session.is_app_scope() {
                session.set_app_scope();
                state.set_user_session(user_id, session.clone()).await;
                send_app_home(bot, msg.chat.id, &session, state.clone()).await?;
            }
            return Ok(());
        }
        "/help" | "Help" => {
            send_trade_reply(bot, msg.chat.id, &session, &help_text()).await?;
            return Ok(());
        }
        _ => {}
    }

    // ── 3. Action commands ──────────────────────────────────────────
    let parts: Vec<&str> = text_to_match.split_whitespace().collect();
    let cmd = parts[0].to_lowercase();
    match cmd.as_str() {
        "Refresh" | "🔄 refresh" => {
            match session.current_view {
                CurrentView::Positions => handle_positions(bot, msg.chat.id, user_id, &session, &state).await?,
                CurrentView::Bots => handle_bots(bot, msg.chat.id, &session, &state).await?,
                CurrentView::Report => handle_report_display(bot, msg.chat.id, session.account_id, None, &session, &state).await?,
                CurrentView::Pending => handle_pending(bot, msg.chat.id, &session, &state).await?,
                _ => {
                    if session.is_app_scope() {
                        send_app_home(bot, msg.chat.id, &session, state.clone()).await?;
                    } else {
                        handle_account_info(bot, msg.chat.id, &session, &state).await?;
                    }
                }
            }
        }

        // Mapping cho bàn phím 3x3
        "Top" | "APP Scope" => {
            session.set_app_scope();
            state.set_user_session(user_id, session.clone()).await;
            send_app_home(bot, msg.chat.id, &session, state.clone()).await?;
        }
        "Back" => {
            if !session.is_app_scope() {
                session.set_app_scope();
                state.set_user_session(user_id, session.clone()).await;
                send_app_home(bot, msg.chat.id, &session, state.clone()).await?;
            }
        }
        "Positions" => {
            handle_positions(bot, msg.chat.id, user_id, &session, &state).await?;
        }
        "Bots" => {
            handle_bots(bot, msg.chat.id, &session, &state).await?;
        }
        "Pending" => {
            handle_pending(bot, msg.chat.id, &session, &state).await?;
        }
        "Report" => {
            handle_report_display(bot, msg.chat.id, session.account_id, None, &session, &state).await?;
        }
        "ON Autotrade" => {
            handle_autotrade(bot, msg.chat.id, user_id, &session, &state, true).await?;
        }
        "OFF Autotrade" => {
            handle_autotrade(bot, msg.chat.id, user_id, &session, &state, false).await?;
        }
        "Accounts" | "App Info" => {
            handle_list_accounts(bot, msg.chat.id, &session, &state).await?;
        }

        _ if text_to_match.starts_with("Acc #") && text_to_match.ends_with(" Info") => {
            handle_account_info(bot, msg.chat.id, &session, &state).await?;
        }

        _ => {
            send_error(bot, msg.chat.id, &format!("Unknown command: `{}`\nUse /help for list.", escape_md(&cmd)), &session).await?;
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
        Some(m) => m.chat.id,
        None => return Ok(()),
    };

    let mut session = state.get_user_session(user_id).await;

    // Parse callback data
    let parts: Vec<&str> = data.splitn(4, ':').collect();
    match parts.as_slice() {
        // ── Navigation ─────────────────────────────────────────────────
        ["top"] | ["app", "home"] => {
            session.set_app_scope();
            state.set_user_session(user_id, session.clone()).await;
            send_app_home(&bot, chat_id, &session, state.clone()).await?;
            return Ok(());
        }
        ["app", "list"] => {
            handle_list_accounts(&bot, chat_id, &session, &state).await?;
            return Ok(());
        }
        ["cancel"] => {
            edit_or_send(&bot, &q, "Đã hủy thao tác.").await?;
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

        // ── Autotrade (từ Info) ────────────────────────────────────────
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

        // ── Positions ───────────────────────────────────────────────────
        [scope, "pos", "sym", sym] => {
            if *scope == "acc" {
                if let Some(id) = session.account_id {
                    ensure_account_scope(&mut session, id, &state).await;
                }
            } else {
                session.set_app_scope();
            }
            handle_positions_tier2(&bot, chat_id, sym, &session, &state).await?;
            return Ok(());
        }
        [scope, "pos", "act", action, sym] => {
            if *scope == "acc" {
                if let Some(id) = session.account_id {
                    ensure_account_scope(&mut session, id, &state).await;
                }
            } else {
                session.set_app_scope();
            }
            let msg = handle_position_action(user_id, session.account_id, action, sym, &session, &state).await;
            edit_or_send(&bot, &q, &msg).await?;
            handle_positions_tier2(&bot, chat_id, sym, &session, &state).await?;
            return Ok(());
        }
        [scope, "pos", "lst"] | [scope, "pos", "ref"] => {
            if *scope == "acc" {
                if let Some(id) = session.account_id {
                    ensure_account_scope(&mut session, id, &state).await;
                }
            } else {
                session.set_app_scope();
            }
            handle_positions(&bot, chat_id, user_id, &session, &state).await?;
            return Ok(());
        }

        // ── Pending ────────────────────────────────────────────────────
        [scope, "pnd", "act", action] => {
            if *scope == "acc" {
                if let Some(id) = session.account_id {
                    ensure_account_scope(&mut session, id, &state).await;
                }
            } else {
                session.set_app_scope();
            }
            let msg = handle_pending_action(user_id, session.account_id, action, &session, &state).await;
            edit_or_send(&bot, &q, &msg).await?;
            handle_pending(&bot, chat_id, &session, &state).await?;
            return Ok(());
        }
        [scope, "pnd", "ref"] => {
            if *scope == "acc" {
                if let Some(id) = session.account_id {
                    ensure_account_scope(&mut session, id, &state).await;
                }
            } else {
                session.set_app_scope();
            }
            handle_pending(&bot, chat_id, &session, &state).await?;
            return Ok(());
        }

        // ── Bots ───────────────────────────────────────────────────────
        [scope, "bot", "sel", short_id] => {
            if *scope == "acc" {
                if let Some(id) = session.account_id {
                    ensure_account_scope(&mut session, id, &state).await;
                }
            } else {
                session.set_app_scope();
            }
            handle_bots_tier2(&bot, chat_id, short_id, &session, &state).await?;
            return Ok(());
        }
        [scope, "bot", "act", action, short_id] => {
            if *scope == "acc" {
                if let Some(id) = session.account_id {
                    ensure_account_scope(&mut session, id, &state).await;
                }
            } else {
                session.set_app_scope();
            }
            let msg = handle_bot_action(user_id, session.account_id, action, short_id, &session, &state).await;
            edit_or_send(&bot, &q, &msg).await?;
            handle_bots_tier2(&bot, chat_id, short_id, &session, &state).await?;
            return Ok(());
        }
        [scope, "bot", "lst"] | [scope, "bot", "ref"] => {
            if *scope == "acc" {
                if let Some(id) = session.account_id {
                    ensure_account_scope(&mut session, id, &state).await;
                }
            } else {
                session.set_app_scope();
            }
            handle_bots(&bot, chat_id, &session, &state).await?;
            return Ok(());
        }

        // ── Report ─────────────────────────────────────────────────────
        [scope, "rpt", target, mode] => {
            let acc_id = if *scope == "acc" {
                session.account_id
            } else {
                target.parse::<i64>().ok().filter(|&id| id != 0)
            };
            handle_report_display(&bot, chat_id, acc_id, Some((*mode).to_string()), &session, &state).await?;
            return Ok(());
        }
        ["app", "rpt", "sum"] | ["app", "rpt", "det"] => {
            let mode = parts[2];
            handle_report_display(&bot, chat_id, None, Some(mode.to_string()), &session, &state).await?;
            return Ok(());
        }

        _ => {
            edit_or_send(&bot, &q, &format!("Mục không xác định: `{}`", data)).await?;
            Ok(())
        }
    }
}

// ── View Handlers ─────────────────────────────────────────────────────────────

/// App home screen
async fn send_app_home(bot: &Bot, chat_id: ChatId, session: &UserSession, state: Arc<AppState>) -> Result<()> {
    let status = format_status(&state).await;
    let text = escape_md(&format!("{}\n━━━━━━━━━━━━━━━━━━━━\n{}", session.scope_badge(), status));
    
    let sent = bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboards::main_reply_keyboard(session.account_id))
        .await?;

    // Xóa pin cũ và ghim tin nhắn App Scope để người dùng luôn thấy
    let _ = bot.unpin_all_chat_messages(chat_id).await;
    let _ = bot.pin_chat_message(chat_id, sent.id).await;

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
    let mut content = "*Accounts List*\n━━━━━━━━━━━━━━━━━━━━\n".to_string();

    if acc_list.is_empty() {
        content.push_str("No accounts found.\nUse `/account add` to add one.");
    } else {
        for acc in &acc_list {
            let atype = if acc.is_real() { "REAL" } else { "DEMO" };
            let auto = if acc.autotrade { "[ON]" } else { "[OFF]" };
            content.push_str(&format!(
                "{} #{} {} [{}]\n   Bal: ${:.2} | Eq: ${:.2} | P&L: {:+.2}\n",
                auto, acc.id, escape_md(&acc.name), atype,
                acc.balance, acc.equity, acc.daily_pnl
            ));
        }
        content.push_str("\nSelect an account to manage:");
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
            bot.send_message(chat_id, format!("Không tìm thấy account \\#{}\\.", acc_id))
                .parse_mode(ParseMode::MarkdownV2).await?;
            return Ok(());
        }
    };
    drop(accounts);

    session.set_account_scope(acc_id, acc.name.clone());
    state.set_user_session(user_id, session.clone()).await;

    let atype = if acc.is_real() { "REAL" } else { "DEMO" };
    let auto = if acc.autotrade { "Autotrade: [ON]" } else { "Autotrade: [OFF]" };

    let text = format!(
        "#{} {} | {}\n------------------------------\n\
        Balance: ${:.2}\n\
        Equity: ${:.2} | Float: {:+.2}\n\
        Daily P&L: {:+.2}\n\
        {}\n------------------------------\n\
        Managing this account. Use keyboard below.",
        acc_id, escape_md(&acc.name), atype,
        acc.balance, acc.equity, acc.float_profit,
        acc.daily_pnl, auto
    );

    let inline_kb = keyboards::report_inline_keyboard(Some(acc_id));

    let sent_info = bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    
    // Xóa pin cũ và ghim tin nhắn Account Scope
    let _ = bot.unpin_all_chat_messages(chat_id).await;
    let _ = bot.pin_chat_message(chat_id, sent_info.id).await;

    // Send account reply keyboard (ẩn) - chỉ để cập nhật keyboard dưới màn hình
    let _ = bot.send_message(chat_id, escape_md(&format!("Đã chuyển sang {:?}", session.scope_badge().replace('*', "").replace('\\', ""))))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboards::main_reply_keyboard(Some(acc_id)))
        .await;
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
        return send_error(bot, chat_id, "Không tìm thấy account\\.", session).await;
    };
    drop(accounts);

    let positions = state.get_open_positions().await;
    let acc_positions: Vec<_> = positions.iter().filter(|p| p.account_id == acc_id).collect();
    let atype = if acc.is_real() { "REAL" } else { "DEMO" };
    let auto = if acc.autotrade { "[ON]" } else { "[OFF]" };

    let text = escape_md(&format!(
        "{}\n\
        Account Information\n\n\
        ID: {} | {} | Auto: {}\n\
        Balance: ${:.2}\n\
        Equity: ${:.2}\n\
        Float P/L: {:+.2}\n\
        Daily P/L: {:+.2}\n\
        Open Positions: {}\n\
        Daily target: ${:.2} | Max loss: ${:.2}",
        session.scope_badge().replace('*', "").replace('\\', ""), acc_id, atype, auto,
        acc.balance, acc.equity, acc.float_profit, acc.daily_pnl,
        acc_positions.len(), acc.daily_target_profit, acc.daily_max_loss
    ));

    send_trade_reply(bot, chat_id, session, &text).await
}

/// View Handlers ─────────────────────────────────────────────────────────────

async fn handle_positions(
    bot: &Bot, chat_id: ChatId, user_id: i64,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    // Update session view
    let mut s = session.clone();
    s.current_view = CurrentView::Positions;
    state.set_user_session(user_id, s.clone()).await;

    let all_positions = state.get_open_positions().await;
    
    // Group positions by account_id
    let mut grouped: std::collections::HashMap<i64, Vec<crate::models::Position>> = std::collections::HashMap::new();
    for pos in all_positions {
        if let Some(sid) = session.account_id {
            if pos.account_id != sid { continue; }
        }
        grouped.entry(pos.account_id).or_default().push(pos);
    }

    let mut content = String::new();
    let mut all_symbols = std::collections::HashSet::new();
    let mut total_float = 0.0;
    let mut total_orders = 0;

    let mut sorted_accs: Vec<_> = grouped.keys().cloned().collect();
    sorted_accs.sort();

    if sorted_accs.is_empty() {
        content.push_str("No open positions.\n");
    } else {
        for acc_id in sorted_accs {
            let acc_positions = grouped.get(&acc_id).unwrap();
            let acc_pnl: f64 = acc_positions.iter().map(|p| p.pnl).sum();
            total_float += acc_pnl;
            total_orders += acc_positions.len();

            content.push_str(&format!("Acc: #{} | Orders: {} | P&L: {:+.2}\n", acc_id, acc_positions.len(), acc_pnl));
            content.push_str("------------------------------\n");

            for pos in acc_positions {
                all_symbols.insert(pos.symbol.clone());
                let side_label = if pos.side.to_uppercase() == "BUY" { "[BUY]" } else { "[SELL]" };
                let pnl_label = if pos.pnl >= 0.0 { "[PROFIT]" } else { "[LOSS]" };
                
                content.push_str(&format!(
                    "{} {} {:.2}L @ {:.5}\nPNL: {:+.2} {}\n\n",
                    pos.bot_id, side_label, pos.volume, pos.open_price,
                    pos.pnl, pnl_label
                ));
            }
            content.push('\n');
        }
        content.push_str("==============================\n");
        content.push_str(&format!("TOTAL ORDERS: {}\nTOTAL FLOAT : {:+.2} USD", total_orders, total_float));
    }

    let mut symbols: Vec<String> = all_symbols.into_iter().collect();
    symbols.sort();

    let badge = session.scope_badge().replace('*', "").replace('\\', "");
    let text = format!("{}\n\n{}", badge, escape_md(&content));
    let inline_kb = keyboards::positions_tier1_keyboard(&symbols, session.account_id);

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

async fn handle_pending(
    bot: &Bot, chat_id: ChatId,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let positions = state.get_open_positions().await;
    
    // Group by account
    let mut grouped: std::collections::HashMap<i64, Vec<crate::models::Position>> = std::collections::HashMap::new();
    for ord in positions {
        if let Some(sid) = session.account_id {
            if ord.account_id != sid { continue; }
        }
        grouped.entry(ord.account_id).or_default().push(ord);
    }

    let mut content = String::new();
    let mut sorted_accs: Vec<_> = grouped.keys().cloned().collect();
    sorted_accs.sort();

    if sorted_accs.is_empty() {
        content.push_str("No pending orders.\n");
    } else {
        for acc_id in sorted_accs {
            let acc_orders = grouped.get(&acc_id).unwrap();
            content.push_str(&format!("Acc: #{} | Pending: {}\n", acc_id, acc_orders.len()));
            content.push_str("------------------------------\n");

            for ord in acc_orders {
                let side_label = if ord.side.to_uppercase() == "BUY" { "[BUY]" } else { "[SELL]" };
                content.push_str(&format!(
                    "{} {} {} @ {:.5}\nVol: {:.2}\n\n",
                    ord.bot_id, side_label, ord.symbol, ord.open_price, ord.volume
                ));
            }
            content.push('\n');
        }
    }

    let badge = session.scope_badge().replace('*', "").replace('\\', "");
    let text = format!("{}\n\n{}", badge, escape_md(&content));
    let inline_kb = keyboards::pending_inline_keyboard(session.account_id);

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

async fn handle_bots(
    bot: &Bot, chat_id: ChatId,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let bots = state.all_bots().await;
    
    // Group by account
    let mut grouped: std::collections::HashMap<i64, Vec<TradingBot>> = std::collections::HashMap::new();
    for b in bots {
        if let Some(sid) = session.account_id {
            if b.account_id != sid { continue; }
        }
        grouped.entry(b.account_id).or_default().push(b);
    }

    let mut content = String::new();
    let mut sorted_accs: Vec<_> = grouped.keys().cloned().collect();
    sorted_accs.sort();

    if sorted_accs.is_empty() {
        content.push_str("No bots configured.\n");
    } else {
        for acc_id in sorted_accs {
            let acc_bots = grouped.get(&acc_id).unwrap();
            content.push_str(&format!("Acc: #{} | Bots: {}\n", acc_id, acc_bots.len()));
            content.push_str("------------------------------\n");

            for b in acc_bots {
                let status = if b.enabled { "[ACTIVE]" } else { "[DISABLED]" };
                content.push_str(&format!(
                    "{} | {} | TF: {}\nDaily P&L: {:+.2}\n",
                    b.id, status, b.timeframe, b.daily_pnl
                ));
            }
            content.push('\n');
        }
    }

    let badge = session.scope_badge().replace('*', "").replace('\\', "");
    let text = format!("{}\n\n{}", badge, escape_md(&content));
    
    // Convert to Bot for keyboard
    let configs: Vec<_> = state.all_bots().await;
    let inline_kb = keyboards::bots_tier1_keyboard(&configs, session.account_id);

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

async fn handle_positions_tier2(
    bot: &Bot, chat_id: ChatId, symbol: &str,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let positions = state.get_open_positions().await;
    let sym_positions: Vec<_> = positions.iter().filter(|p| {
        p.symbol == symbol && (session.account_id.is_none() || session.account_id == Some(p.account_id))
    }).collect();

    if sym_positions.is_empty() {
        return send_error(bot, chat_id, &format!("No {} positions found.", symbol), session).await;
    }

    let total_pnl: f64 = sym_positions.iter().map(|p| p.pnl).sum();
    let buy_vol: f64 = sym_positions.iter().filter(|p| p.side.to_uppercase() == "BUY").map(|p| p.volume).sum();
    let sell_vol: f64 = sym_positions.iter().filter(|p| p.side.to_uppercase() == "SELL").map(|p| p.volume).sum();

    let mut content = format!(
        "Analysis: {}\n------------------------------\n\
        Orders: {}\n\
        Buy Vol: {:.2}L | Sell Vol: {:.2}L\n\
        Net P&L: {:+.2} USD\n\n\
        Select action:",
        symbol, sym_positions.len(), buy_vol, sell_vol, total_pnl
    );

    let badge = session.scope_badge().replace('*', "").replace('\\', "");
    let text = format!("{}\n\n{}", badge, escape_md(&content));
    let inline_kb = keyboards::positions_actions_keyboard(symbol, session.account_id);

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

async fn handle_bots_tier2(
    bot: &Bot, chat_id: ChatId, bot_id: &str,
    session: &UserSession, state: &Arc<AppState>
) -> Result<()> {
    let bots = state.bots.read().await;
    let Some(bot_cfg) = bots.get(bot_id).cloned() else {
        drop(bots);
        return send_error(bot, chat_id, "Bot not found.", session).await;
    };
    drop(bots);

    let status_label = if bot_cfg.enabled { "[ACTIVE]" } else { "[DISABLED]" };
    let content = format!(
        "Bot: {}\n------------------------------\n\
        Status: {}\n\
        Symbol: {}\n\
        TF: {}\n\
        Daily P&L: {:+.2}\n\n\
        Select action:",
        bot_cfg.id, status_label, bot_cfg.symbol, bot_cfg.timeframe, bot_cfg.daily_pnl
    );

    let badge = session.scope_badge().replace('*', "").replace('\\', "");
    let text = format!("{}\n\n{}", badge, escape_md(&content));
    let inline_kb = keyboards::bots_tier2_keyboard(&bot_cfg.id, Some(bot_cfg.enabled), session.account_id);

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb))
        .await?;
    Ok(())
}

async fn handle_autotrade(
    bot: &Bot, chat_id: ChatId, _user_id: i64,
    session: &UserSession, state: &Arc<AppState>, enable: bool
) -> Result<()> {
    match session.account_id {
        Some(acc_id) => {
            state.update_account_autotrade(acc_id, enable).await?;
            let status = if enable { "[ON]" } else { "[OFF]" };
            send_trade_reply(bot, chat_id, session, &format!("Autotrade is now {} for account #{}", status, acc_id)).await?;
        }
        None => {
            state.set_all_autotrade(enable).await?;
            let status = if enable { "[ON]" } else { "[OFF]" };
            send_trade_reply(bot, chat_id, session, &format!("Global Autotrade is now {}", status)).await?;
        }
    }
    Ok(())
}

async fn handle_report_display(
    bot: &Bot, chat_id: ChatId,
    _acc_id: Option<i64>, _bot_id: Option<String>,
    session: &UserSession, _state: &Arc<AppState>
) -> Result<()> {
    // Professional text-only report
    send_trade_reply(bot, chat_id, session, "Professional Report system: Active").await
}

// ── Action Handlers ────────────────────────────────────────────────────────────

async fn handle_position_action(
    _user_id: i64, _acc_id: Option<i64>, action: &str, target: &str,
    session: &UserSession, state: &Arc<AppState>
) -> String {
    let order_action = match action {
        "c" | "close" => OrderAction::Close,
        "cp" | "profit" => OrderAction::CloseProfit,
        "cl" | "loss" => OrderAction::CloseLoss,
        "cb" | "buy" => OrderAction::CloseBuy,
        "cs" | "sell" => OrderAction::CloseSell,
        _ => return "Action invalid".to_string(),
    };
    
    let mut req = make_req(session, "bot".to_string(), order_action);
    req.target_id = target.to_string();
    if target != "all" { req.symbol = Some(target.to_string()); }

    match crate::engine::dispatch(state.clone(), req).await {
        Ok(res) => res.messages.join("\n"),
        Err(e) => format!("Error: {}", e),
    }
}

async fn handle_bot_action(
    _user_id: i64, _acc_id: Option<i64>, action: &str, bot_id: &str,
    session: &UserSession, state: &Arc<AppState>
) -> String {
    let order_action = match action {
        "on" | "enable" => OrderAction::EnableBot,
        "off" | "disable" => OrderAction::DisableBot,
        _ => return "Bot action invalid".to_string(),
    };

    let mut req = make_req(session, bot_id.to_string(), order_action);
    req.target_id = bot_id.to_string();

    match crate::engine::dispatch(state.clone(), req).await {
        Ok(res) => res.messages.join("\n"),
        Err(e) => format!("Error: {}", e),
    }
}

async fn handle_pending_action(
    _user_id: i64, _acc_id: Option<i64>, action: &str,
    session: &UserSession, state: &Arc<AppState>
) -> String {
    let order_action = match action {
        "del" | "delete" => OrderAction::Delete,
        _ => return "Pending action invalid".to_string(),
    };

    let req = make_req(session, "bot".to_string(), order_action);
    match crate::engine::dispatch(state.clone(), req).await {
        Ok(res) => res.messages.join("\n"),
        Err(e) => format!("Error: {}", e),
    }
}

// ── Notify & Status ───────────────────────────────────────────────────────────

pub async fn send_trade_event_to_group(bot: &teloxide::Bot, chat_id: &str, event_type: &str, pos: &Position) -> Result<()> {
    if chat_id.is_empty() { return Ok(()); }
    let text = format!(
        "[{}] {}\nAcc #{} | Bot: {}\n{} {} @ {}\nVol: {} | P&L: {:+}",
        event_type, event_type, pos.account_id, escape_md(&pos.bot_id),
        pos.side.to_uppercase(), escape_md(&pos.symbol), pos.open_price,
        pos.volume, pos.pnl
    );
    send_notify(bot, chat_id, &text).await
}

pub async fn send_notify(bot: &teloxide::Bot, chat_id: &str, text: &str) -> Result<()> {
    use teloxide::types::ChatId;
    if chat_id.is_empty() { return Ok(()); }
    let msg = escape_md(text);
    let result = if let Ok(id) = chat_id.trim_start_matches('@').parse::<i64>() {
        bot.send_message(ChatId(id), msg).parse_mode(ParseMode::MarkdownV2).await
    } else {
        bot.send_message(chat_id.to_string(), msg).parse_mode(ParseMode::MarkdownV2).await
    };
    if let Err(e) = result { warn!("Notify failed: {}", e); }
    Ok(())
}

async fn format_status(state: &Arc<AppState>) -> String {
    let s = state.get_system_status().await;
    let mode = if state.config.is_mock() { "Mode: Mock" } else { "Mode: Live" };
    format!(
        "{} | Uptime: {}\n\n\
        Accounts: {} | Online: {}/{}\n\
        Auto: {}/{}\n\n\
        Real Balance: ${:.0}\n\
        Equity: ${:.0} | Float: {:+.0}\n\
        Daily P/L: {:+.0}\n\n\
        Bots: {}/{} | Positions: {}",
        mode, AppState::format_uptime(s.uptime_secs),
        s.total_accounts, s.connected_accounts, s.total_accounts,
        s.active_accounts, s.total_accounts,
        s.total_real_balance, s.total_real_equity, s.total_float_profit,
        s.total_daily_pnl, s.active_bots, s.total_bots, s.open_positions
    )
}

// ── Utilities ─────────────────────────────────────────────────────────────────

fn make_req(session: &UserSession, bot_id: String, action: OrderAction) -> OrderRequest {
    let mut req = OrderRequest::new(TradeSource::Telegram, bot_id, action);
    if let Some(acc_id) = session.account_id {
        req.account_scope = AccountScope::Single;
        req.account_ids = vec![acc_id];
    }
    req
}

async fn send_trade_reply(bot: &Bot, chat_id: ChatId, session: &UserSession, msg: &str) -> Result<()> {
    let badge = session.scope_badge().replace('*', "").replace('\\', "");
    let text = format!("{}\n------------------------------\n{}", badge, escape_md(msg));
    bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}

async fn send_error(bot: &Bot, chat_id: ChatId, msg: &str, session: &UserSession) -> Result<()> {
    let badge = session.scope_badge().replace('*', "").replace('\\', "");
    let text = format!("{}\nERROR: {}", badge, escape_md(msg));
    bot.send_message(chat_id, text).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}

async fn edit_or_send(bot: &Bot, q: &CallbackQuery, text: &str) -> Result<()> {
    if let Some(msg) = &q.message {
        let _ = bot.edit_message_text(msg.chat.id, msg.id, text).parse_mode(ParseMode::MarkdownV2).await;
    }
    Ok(())
}

async fn ensure_account_scope(session: &mut UserSession, acc_id: i64, state: &Arc<AppState>) {
    if session.account_id != Some(acc_id) {
        let accounts = state.accounts.read().await;
        if let Some(acc) = accounts.get(&acc_id) {
            session.set_account_scope(acc_id, acc.name.clone());
        }
    }
}

fn escape_md(s: &str) -> String {
    s.chars().flat_map(|c| {
        if "_*[]()~`>#+-=|{}.!".contains(c) { vec!['\\', c] } else { vec![c] }
    }).collect()
}

fn help_text() -> String {
    "iZFx.Trade v2.1\n\n\
    /list - Accounts\n\
    /account <id> - Manage Acc\n\
    /top - App Scope\n\n\
    /status - System Status\n\
    /key - API Key Management".to_string()
}
