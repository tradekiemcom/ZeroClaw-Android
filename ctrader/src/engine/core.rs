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
        OrderAction::EnableBot => {
            let found = state.set_bot_enabled(&req.bot_id, true).await?;
            if found {
                result.messages.push(format!("[ACTIVE] Bot `{}` is now ENABLED", req.bot_id));
                result.success_count = 1;
            } else {
                result.messages.push(format!("[ERROR] Bot `{}` not found", req.bot_id));
                result.error_count = 1;
            }
        }
        OrderAction::DisableBot => {
            let found = state.set_bot_enabled(&req.bot_id, false).await?;
            if found {
                result.messages.push(format!("[DISABLED] Bot `{}` is now OFF", req.bot_id));
                result.success_count = 1;
            } else {
                result.messages.push(format!("[ERROR] Bot `{}` not found", req.bot_id));
                result.error_count = 1;
            }
        }

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
            result.messages.push(format!("🔴 Đã đóng {} lệnh + tắt autotrade", closed));
            result.success_count = closed as usize;

            // 📢 Notify group: CLOSE ALL event
            if !state.config.telegram_notify_chat_id.is_empty() {
                if let Some(bot) = &state.telegram_bot {
                    let msg = format!(
                        "🔴 *CLOSE ALL* \n\
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

            result.messages.push(format!("✅ {:?} bot: `{}` | Symbol: {} | Closed: {}", req.action, req.bot_id, symbol, closed));
            result.success_count = closed as usize;

            // 📢 Notify group
            if !state.config.telegram_notify_chat_id.is_empty() && closed > 0 {
                if let Some(bot) = &state.telegram_bot {
                    let msg = format!(
                        "🔴 *{}* | Bot: `{}`\nSbl: {} | Closed: {} orders",
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
                        result.messages.push(format!("❌ Bot `{}` đang bị tắt", req.bot_id));
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
                            result.messages.push(format!("🛑 Account {}: {}", acc_id, reason));
                            continue;
                        }
                    }
                }

                match state.pool.execute_order(acc_id, req.clone()).await {
                    Ok(exec_result) if exec_result.success => {
                        if let Some(pos) = exec_result.position {
                            info!("✅ Opened | Account: {} | {} {} {}@{}",
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
                        error!("❌ Execute failed: {}", exec_result.message);
                        result.messages.push(exec_result.message);
                        result.error_count += 1;
                    }
                    Err(e) => {
                        error!("❌ cTrader error: {}", e);
                        result.messages.push(e.to_string());
                        result.error_count += 1;
                    }
                }
            }
        }

        _ => {
            result.messages.push(format!("⚠️ Action {:?} chưa được implement", req.action));
            result.error_count = 1;
        }
    }

    Ok(result)
}
