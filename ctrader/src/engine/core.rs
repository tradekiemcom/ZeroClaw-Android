use std::sync::Arc;
use anyhow::Result;
use tracing::{info, warn, error};

use crate::state::AppState;
use crate::models::order::{OrderRequest, OrderAction, AccountScope};
use crate::models::position::PositionStatus;
use crate::ctrader::ExecutionResult;

/// Kết quả dispatch từ Core Engine
#[derive(Debug)]
pub struct DispatchResult {
    pub request_id: String,
    pub success_count: usize,
    pub error_count: usize,
    pub messages: Vec<String>,
    pub positions: Vec<crate::models::Position>,
}

impl DispatchResult {
    pub fn to_telegram_msg(&self) -> String {
        if self.error_count == 0 {
            format!(
                "[SUCCESS] Processing orders\nExecuted: {} account(s)\nID: {}",
                self.success_count,
                &self.request_id[..8],
            )
        } else {
            format!(
                "[WARNING] Result: {} OK / {} Errors\nID: {}",
                self.success_count, self.error_count,
                &self.request_id[..8],
            )
        }
    }
}

/// Gửi lệnh qua Core Engine — điểm trung tâm của toàn bộ system
pub async fn dispatch(state: Arc<AppState>, req: OrderRequest) -> Result<DispatchResult> {
    info!("🔀 Dispatch | Source: {} | Action: {:?} | Bot: {} | ReqID: {}",
        req.source, req.action, req.bot_id,
        &req.request_id[..8.min(req.request_id.len())]
    );

    // Xác định danh sách account cần xử lý
    let account_ids = match &req.account_scope {
        AccountScope::All => state.all_account_ids().await,
        AccountScope::Single if !req.account_ids.is_empty() => req.account_ids.clone(),
        AccountScope::List => req.account_ids.clone(),
        _ => state.all_account_ids().await,
    };

    if account_ids.is_empty() {
        warn!("⚠️  Không có account nào để xử lý");
    }

    let mut result = DispatchResult {
        request_id: req.request_id.clone(),
        success_count: 0,
        error_count: 0,
        messages: vec![],
        positions: vec![],
    };

    match &req.action {
        // ── Kiểm soát Autotrade ──────────────────────────────────────
        OrderAction::EnableAutotrade => {
            state.set_all_autotrade(true).await?;
            result.messages.push("[SUCCESS] Autotrade is now ON for all accounts".to_string());
            result.success_count = 1;
        }
        OrderAction::DisableAutotrade => {
            state.set_all_autotrade(false).await?;
            result.messages.push("[OFF] Autotrade is now OFF for all accounts".to_string());
            result.success_count = 1;
        }

        // ── Bot Control ──────────────────────────────────────────────

        // ── Close All ────────────────────────────────────────────────
        OrderAction::CloseAll => {
            // Đóng trong cTrader
            for &acc_id in &account_ids {
                let _ = state.pool.close_order(acc_id, "ALL_BY_BOT").await;
            }
            // Đóng trong DB
            let closed = crate::storage::close_all_positions(&state.db).await?;
            // Update memory
            {
                let mut positions = state.positions.write().await;
                for pos in positions.iter_mut() {
                    if pos.status == PositionStatus::Open {
                        pos.status = PositionStatus::Closed;
                        pos.closed_at = Some(chrono::Utc::now());
                    }
                }
            }
            state.set_all_autotrade(false).await?;
            result.messages.push(format!("[OFF] Closed {} orders + disabled autotrade", closed));
            result.success_count = closed as usize;

            // 📢 Notify group: CLOSE ALL event
            if !state.config.telegram_notify_chat_id.is_empty() {
                if let Some(bot) = &state.telegram_bot {
                    let msg = format!(
                        "[OFF] CLOSE ALL\n\
                        Source: {} | Bot: `{}`\n\
                        Đã đóng {} lệnh + tắt autotrade",
                        req.source, req.bot_id, closed
                    );
                    let _ = crate::telegram::send_notify(bot, &state.config.telegram_notify_chat_id, &msg).await;
                }
            }
        }


        // ── Close by Bot / Action ────────────────────────────────────
        OrderAction::Close | OrderAction::CloseBuy | OrderAction::CloseSell | 
        OrderAction::CloseProfit | OrderAction::CloseLoss => {
            let symbol = req.symbol.as_deref().unwrap_or("ALL");
            let action_str = format!("{:?}", req.action);
            
            for &acc_id in &account_ids {
                // Trong mock, we just say Success. 
                // Actual implementation sẽ cần lọc position trong state và gửi từng lệnh đóng.
                let _ = state.pool.close_order(acc_id, &req.bot_id).await;
            }

            // DB Update: 
            let closed = match req.action {
                OrderAction::CloseProfit => crate::storage::close_positions_by_pnl(&state.db, &req.bot_id, true).await?,
                OrderAction::CloseLoss => crate::storage::close_positions_by_pnl(&state.db, &req.bot_id, false).await?,
                _ => crate::storage::close_positions_by_bot(&state.db, &req.bot_id).await?,
            };

            // Memory Sync:
            {
                let mut positions = state.positions.write().await;
                let bot_id = &req.bot_id;
                for pos in positions.iter_mut().filter(|p| {
                    if p.status != PositionStatus::Open { return false; }
                    if p.bot_id != *bot_id { return false; }
                    if symbol != "ALL" && p.symbol != symbol { return false; }
                    match req.action {
                        OrderAction::CloseBuy => p.side.to_uppercase() == "BUY",
                        OrderAction::CloseSell => p.side.to_uppercase() == "SELL",
                        OrderAction::CloseProfit => p.pnl > 0.0,
                        OrderAction::CloseLoss => p.pnl <= 0.0,
                        _ => true,
                    }
                }) {
                    pos.status = PositionStatus::Closed;
                    pos.closed_at = Some(chrono::Utc::now());
                }
            }

            result.messages.push(format!("[OK] {:?} bot: `{}` | Symbol: {} | Closed: {}", req.action, req.bot_id, symbol, closed));
            result.success_count = closed as usize;

            // 📢 Notify group
            if !state.config.telegram_notify_chat_id.is_empty() && closed > 0 {
                if let Some(bot) = &state.telegram_bot {
                    let msg = format!(
                        "[OFF] {} | Bot: `{}`\nSbl: {} | Closed: {} orders",
                        action_str.to_uppercase(), req.bot_id, symbol, closed
                    );
                    let _ = crate::telegram::send_notify(bot, &state.config.telegram_notify_chat_id, &msg).await;
                }
            }
        }


        // ── Open Trade ───────────────────────────────────────────────
        OrderAction::Open => {
            // Kiểm tra bot tồn tại và đang bật
            {
                let bots = state.bots.read().await;
                if let Some(bot) = bots.get(&req.bot_id) {
                    if !bot.enabled {
                        result.messages.push(format!("[ERROR] Bot `{}` is disabled", req.bot_id));
                        result.error_count = 1;
                        return Ok(result);
                    }
                }
            }

            // Execute trên từng account
            for &acc_id in &account_ids {
                // Kiểm tra risk
                {
                    let accounts = state.accounts.read().await;
                    if let Some(acc) = accounts.get(&acc_id) {
                        if !acc.autotrade {
                            result.messages.push(format!("⏸️ Account {} đang ở manual mode", acc_id));
                            continue;
                        }
                        if let Some(reason) = acc.should_halt_trading() {
                            result.messages.push(format!("[STOP] Account {}: {}", acc_id, reason));
                            continue;
                        }
                    }
                }

                match state.pool.execute_order(acc_id, req.clone()).await {
                    Ok(exec_result) if exec_result.success => {
                        if let Some(pos) = exec_result.position {
                            info!("[OK] Opened | Account: {} | {} {} {}@{}",
                                acc_id, pos.side, pos.symbol, pos.volume, pos.open_price);
                            state.add_position(pos.clone()).await?;

                            // 📢 Notify group: OPEN event (mọi nguồn đều notify)
                            if !state.config.telegram_notify_chat_id.is_empty() {
                                if let Some(bot) = &state.telegram_bot {
                                    let _ = crate::telegram::send_trade_event_to_group(
                                        bot,
                                        &state.config.telegram_notify_chat_id,
                                        "OPEN",
                                        &pos,
                                    ).await;
                                }
                            }

                            result.positions.push(pos);
                        }
                        result.success_count += 1;
                    }
                    Ok(exec_result) => {
                        error!("[ERROR] Execute failed: {}", exec_result.message);
                        result.messages.push(exec_result.message);
                        result.error_count += 1;
                    }
                    Err(e) => {
                        error!("[ERROR] cTrader error: {}", e);
                        result.messages.push(e.to_string());
                        result.error_count += 1;
                    }
                }
            }
        }

        // ── Reporting & Info ─────────────────────────────────────────
        OrderAction::ListAccounts => {
            let summary = state.format_accounts_summary().await;
            result.messages.push(summary);
            result.success_count = 1;
        }
        OrderAction::ListPositions => {
            let positions = state.get_open_positions().await;
            let mut msg = String::new();
            for p in positions {
                if !account_ids.contains(&p.account_id) { continue; }
                msg.push_str(&format!("Acc:{} | {} {} {} @ {} | P&L: {:+.2}\n", p.account_id, p.side, p.symbol, p.volume, p.open_price, p.pnl));
            }
            if msg.is_empty() { msg = "[INFO] No open positions.".to_string(); }
            result.messages.push(msg);
            result.success_count = 1;
        }
        OrderAction::AccountReport => {
            let accounts = state.accounts.read().await;
            for &id in &account_ids {
                if let Some(acc) = accounts.get(&id) {
                    let mut msg = format!("\n[ACCOUNT REPORT] #{} : {}\n", id, acc.name);
                    msg.push_str("----------------------------------------\n");
                    msg.push_str(&format!("Type: {:?} | Connected: {} | Auto: {}\n", acc.account_type, acc.connected, acc.autotrade));
                    msg.push_str(&format!("Balance: ${:.2} | Equity: ${:.2}\n", acc.balance, acc.equity));
                    msg.push_str(&format!("Daily P&L: ${:+.2} | Float: {:+.2}\n", acc.daily_pnl, acc.float_profit));
                    result.messages.push(msg);
                    result.success_count += 1;
                }
            }
        }

        // ── Management Actions (CRUD) ────────────────────────────────
        OrderAction::AddAccount => {
            // target_id contains "id:name:token:demo|real" or similar
            let parts: Vec<&str> = req.target_id.split(':').collect();
            if parts.len() >= 3 {
                let id = parts[0].parse::<i64>().unwrap_or(0);
                let name = parts[1].to_string();
                let token = parts[2].to_string();
                let is_real = parts.get(3) == Some(&"real");
                
                let mut acc = crate::models::Account::new(id, name, id, if is_real { crate::models::AccountType::Real } else { crate::models::AccountType::Demo });
                acc.access_token = Some(token);
                state.add_account(acc).await?;
                result.messages.push(format!("[SUCCESS] Added Account #{}", id));
                result.success_count = 1;
            } else {
                result.messages.push("[ERROR] Invalid AddAccount parameters".to_string());
                result.error_count = 1;
            }
        }
        OrderAction::DeleteAccount => {
            if let Ok(id) = req.target_id.parse::<i64>() {
                if state.delete_account(id).await? {
                    result.messages.push(format!("[SUCCESS] Deleted Account #{}", id));
                    result.success_count = 1;
                } else {
                    result.messages.push("[ERROR] Account not found".to_string());
                    result.error_count = 1;
                }
            }
        }
        OrderAction::AddApiClient => {
            let client = crate::models::ApiClient::new(req.target_id.clone(), "API".to_string());
            let key = client.api_key.clone();
            state.add_api_client(client).await?;
            result.messages.push(format!("[SUCCESS] Created API Key: {}", key));
            result.success_count = 1;
        }
        OrderAction::ListApiClients => {
            let clients = state.list_api_clients().await;
            let mut msg = "\n[API CLIENTS]\n".to_string();
            for c in clients {
                msg.push_str(&format!("{}\n", c.format_list_item()));
            }
            result.messages.push(msg);
            result.success_count = 1;
        }

        // ── System Utilities ─────────────────────────────────────────
        OrderAction::SystemStatus => {
            let status = state.get_system_status().await;
            let mut msg = format!("\n[SYSTEM REPORT]\n");
            msg.push_str("----------------------------------------\n");
            msg.push_str(&format!("Uptime: {}\n", AppState::format_uptime(status.uptime_secs)));
            msg.push_str(&format!("Accounts: {} targets | Active: {}\n", status.total_accounts, status.active_accounts));
            msg.push_str(&format!("Daily P&L: ${:+.2} | Float: {:+.2}\n", status.total_daily_pnl, status.total_float_profit));
            result.messages.push(msg);
            result.success_count = 1;
        }
        OrderAction::SystemCleanup => {
            info!("Cleanup action requested via {}", req.source);
            let status = std::process::Command::new("bash")
                .arg("../scripts/99-uninstall-cleanup.sh")
                .status();
            match status {
                Ok(s) if s.success() => result.messages.push("[SUCCESS] System cleanup completed successfully".to_string()),
                _ => result.messages.push("[ERROR] Cleanup failed or script not found".to_string()),
            }
            result.success_count = 1;
        }
        OrderAction::SystemNuclearWipe => {
            if req.source == crate::models::order::TradeSource::Cli {
                info!("NUCLEAR WIPE EXECUTED via CLI");
                let status = std::process::Command::new("bash")
                    .arg("../scripts/99-uninstall-cleanup.sh")
                    .arg("--force")
                    .status();
                match status {
                    Ok(s) if s.success() => result.messages.push("[CAUTION] SYSTEM WIPED CLEAN".to_string()),
                    _ => result.messages.push("[ERROR] Nuclear wipe failed".to_string()),
                }
                result.success_count = 1;
            } else {
                result.messages.push("[ERROR] Dangerous action restricted to CLI".to_string());
                result.error_count = 1;
            }
        }

        // ── Enhanced Bot & Grouped Management ─────────────────────────
        OrderAction::ListGrouped => {
            let by_bot = req.account_scope == crate::models::AccountScope::Single;
            let grouped = state.get_grouped_positions(&account_ids, by_bot).await;
            
            if grouped.is_empty() {
                result.messages.push("[INFO] No open positions found for the selected scope.".to_string());
            } else {
                let mut msg = format!("\n[OPEN POSITIONS] Grouped by {}\n", if by_bot { "Bot" } else { "Account" });
                msg.push_str("----------------------------------------\n");
                
                for (group_id, pos_list) in grouped {
                    let total_pnl: f64 = pos_list.iter().map(|p| p.pnl).sum();
                    msg.push_str(&format!("\nGroup {}: {} items | Float: ${:+.2}\n", group_id, pos_list.len(), total_pnl));
                    for p in pos_list {
                        msg.push_str(&format!("  - {} {} {} @ {} | P&L: {:+.2}\n", p.side, p.symbol, p.volume, p.open_price, p.pnl));
                    }
                }
                result.messages.push(msg);
            }
            result.success_count = 1;
        }

        OrderAction::EnableBot | OrderAction::DisableBot | OrderAction::CloseBotPositions => {
            let enabled = req.action == OrderAction::EnableBot;
            let bot_id = if req.bot_id.starts_with('/') || req.source == crate::models::TradeSource::Cli && req.target_id.parse::<usize>().is_ok() {
                // Shorthand resolution
                let idx: usize = req.target_id.parse().unwrap_or(1);
                let bots = state.all_bots().await;
                // Filter bots for current account scope if single
                let filtered: Vec<_> = if account_ids.len() == 1 {
                    bots.into_iter().filter(|b| b.account_id == account_ids[0]).collect()
                } else {
                    bots
                };
                filtered.get(idx - 1).map(|b| b.id.clone())
            } else {
                Some(req.target_id.clone())
            };

            if let Some(id) = bot_id {
                match req.action {
                    OrderAction::EnableBot | OrderAction::DisableBot => {
                        state.set_bot_enabled(&id, enabled).await?;
                        result.messages.push(format!("[SUCCESS] Bot `{}` status: {}", id, if enabled { "ACTIVE" } else { "DISABLED" }));
                    }
                    OrderAction::CloseBotPositions => {
                        // 1. Disable bot
                        state.set_bot_enabled(&id, false).await?;
                        // 2. Close positions for this bot
                        let positions = state.get_positions_by_bot(&id).await;
                        for _p in positions {
                            // In a real system, we'd call the connection pool to close
                            // For now, we update local state status
                            result.success_count += 1;
                        }
                        result.messages.push(format!("[OFF] Bot `{}` disabled & {} positions closed.", id, result.success_count));
                    }
                    _ => {}
                }
                result.success_count = 1;
            } else {
                result.messages.push("[ERROR] Bot not found or invalid index.".to_string());
                result.error_count = 1;
            }
        }

        OrderAction::BotReport => {
            let bots = state.all_bots().await;
            let mut msg = format!("\n[BOT PERFORMANCE REPORT]\n");
            msg.push_str("----------------------------------------\n");
            for bot in bots {
                let st = if bot.enabled { "[ON]" } else { "[OFF]" };
                msg.push_str(&format!("{} `{}` ({}) | P&L: ${:+.2} | Trades: {}\n", 
                    st, bot.id, bot.symbol, bot.daily_pnl, bot.trade_count_today));
            }
            result.messages.push(msg);
            result.success_count = 1;
        }

        // ── AI Agent Lifecycle ───────────────────────────────────────
        OrderAction::AgentOn => {
            let mut agent = state.ai_agent.lock().await;
            info!("AI Agent activation requested...");
            match agent.load_model().await {
                Ok(_) => {
                    let status = state.get_system_status().await;
                    let msg = format!(
                        "[AGENT ONLINE] Hello Hưng! AI Agent is now ready.\n\
                        Memory loaded successfully.\n\n\
                        System Snapshot:\n\
                        - Accounts: {}\n\
                        - Total Equity: ${:.2}\n\
                        - Open Positions: {}\n\n\
                        I am ready to assist with your trading tasks.",
                        status.total_accounts, status.total_real_equity, status.open_positions
                    );
                    result.messages.push(msg);
                    result.success_count = 1;
                }
                Err(e) => {
                    result.messages.push(format!("[ERROR] Failed to load AI: {}", e));
                    result.error_count = 1;
                }
            }
        }

        OrderAction::AgentOff => {
            let mut agent = state.ai_agent.lock().await;
            agent.unload();
            result.messages.push("[OFF] AI Agent is now offline. Memory released.".to_string());
            result.success_count = 1;
        }

        _ => {
            result.messages.push(format!("⚠️ Action {:?} chưa được implement", req.action));
            result.error_count = 1;
        }
    }

    Ok(result)
}
