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
                "✅ Lệnh thành công\n📦 Đã thực thi: {} account(s)\n🆔 `{}`",
                self.success_count,
                &self.request_id[..8],
            )
        } else {
            format!(
                "⚠️ Kết quả: {} OK / {} lỗi\n🆔 `{}`",
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
            result.messages.push("✅ Autotrade đã BẬT cho tất cả account".to_string());
            result.success_count = 1;
        }
        OrderAction::DisableAutotrade => {
            state.set_all_autotrade(false).await?;
            result.messages.push("🔴 Autotrade đã TẮT cho tất cả account".to_string());
            result.success_count = 1;
        }

        // ── Bot Control ──────────────────────────────────────────────
        OrderAction::EnableBot => {
            let found = state.set_bot_enabled(&req.bot_id, true).await?;
            if found {
                result.messages.push(format!("✅ Bot `{}` đã BẬT", req.bot_id));
                result.success_count = 1;
            } else {
                result.messages.push(format!("❌ Không tìm thấy bot `{}`", req.bot_id));
                result.error_count = 1;
            }
        }
        OrderAction::DisableBot => {
            let found = state.set_bot_enabled(&req.bot_id, false).await?;
            if found {
                result.messages.push(format!("⏸️ Bot `{}` đã TẮT", req.bot_id));
                result.success_count = 1;
            } else {
                result.messages.push(format!("❌ Không tìm thấy bot `{}`", req.bot_id));
                result.error_count = 1;
            }
        }

        // ── Close All ────────────────────────────────────────────────
        OrderAction::CloseAll => {
            // Đóng trong cTrader
            for &acc_id in &account_ids {
                let _ = state.ctrader.close_all_by_bot(acc_id, &req.bot_id).await;
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


        // ── Close by Bot ─────────────────────────────────────────────
        OrderAction::Close => {
            for &acc_id in &account_ids {
                let _ = state.ctrader.close_all_by_bot(acc_id, &req.bot_id).await;
            }
            let closed = crate::storage::close_positions_by_bot(&state.db, &req.bot_id).await?;
            {
                let mut positions = state.positions.write().await;
                for pos in positions.iter_mut().filter(|p| p.bot_id == req.bot_id) {
                    pos.status = PositionStatus::Closed;
                    pos.closed_at = Some(chrono::Utc::now());
                }
            }
            result.messages.push(format!("✅ Đã đóng {} lệnh của bot `{}`", closed, req.bot_id));
            result.success_count = closed as usize;

            // 📢 Notify group: CLOSE event
            if !state.config.telegram_notify_chat_id.is_empty() && closed > 0 {
                if let Some(bot) = &state.telegram_bot {
                    let msg = format!(
                        "🔴 *CLOSE* | Source: {} | Bot: `{}`\n\
                        Đã đóng {} lệnh",
                        req.source, req.bot_id, closed
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

                match state.ctrader.execute(&req, acc_id).await {
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
