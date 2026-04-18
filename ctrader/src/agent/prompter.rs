use anyhow::Result;
use std::sync::Arc;
use crate::state::AppState;
use crate::models::{OrderRequest, OrderAction, TradeSource, TradeSide};
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResponse {
    pub action: String,
    pub params: Option<serde_json::Value>,
}

pub async fn prepare_context(state: &Arc<AppState>) -> String {
    let status = state.get_system_status().await;
    let positions = state.get_open_positions().await;
    
    let mut ctx = format!(
        "SYSTEM CONTEXT:\n\
        - User: Hưng (Owner)\n\
        - Total Balance: ${:.2}\n\
        - Total Equity: ${:.2}\n\
        - Float Profit: ${:+.2}\n\
        - Open Positions Count: {}\n",
        status.total_real_balance, 
        status.total_real_equity, 
        status.total_float_profit,
        status.open_positions
    );

    if !positions.is_empty() {
        ctx.push_str("- Current Positions:\n");
        for p in positions.iter().take(10) {
            ctx.push_str(&format!("  - Acc:{} | {} {} {} @ {}\n", p.account_id, p.side, p.symbol, p.volume, p.open_price));
        }
    }

    ctx.push_str("\nINSTRUCTIONS:\n\
        Bạn là bộ não điều hành của module cTrader trong hệ sinh thái ZeroClaw.\n\
        Nhiệm vụ của bạn là nhận tin nhắn từ chủ thể (Hưng), phân tích ý định và trả về DUY NHẤT một mã JSON.\n\
        Các hành động cho phép:\n\
        - OPEN_ORDER: params: {symbol, side, volume}\n\
        - CLOSE_ORDER: params: {symbol, bot_id}\n\
        - GET_STATUS: params: {}\n\
        - CANCEL_ALL: params: {}\n\
        - TASK_PLAN: params: {steps: Array<String>} (Dùng khi cần kiểm tra thông tin hoặc báo cáo trước khi thực thi)\n\n\
        KHÔNG giải thích, KHÔNG chào hỏi, chỉ trả về JSON.");

    ctx
}

pub async fn parse_agent_json(raw: &str, _state: &Arc<AppState>) -> Result<Option<OrderRequest>> {
    // LLM sometimes wraps JSON in code blocks
    let clean_json = raw.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let resp: AgentResponse = match serde_json::from_str(clean_json) {
        Ok(r) => r,
        Err(e) => {
            warn!("AI returned invalid JSON: {}. Raw: {}", e, raw);
            return Ok(None);
        }
    };

    let source = TradeSource::Telegram; // Default for AI for now
    let action = match resp.action.as_str() {
        "OPEN_ORDER" => OrderAction::Open,
        "CLOSE_ORDER" => OrderAction::Close,
        "GET_STATUS" => OrderAction::SystemStatus,
        "CANCEL_ALL" => OrderAction::CloseAll,
        "TASK_PLAN" => OrderAction::ListAccounts, // Map TASK_PLAN to a broad report for now
        _ => return Ok(None),
    };

    let mut req = OrderRequest::new(source, "ai_agent".to_string(), action);
    
    if let Some(params) = resp.params {
        if let Some(symbol) = params.get("symbol").and_then(|v| v.as_str()) {
            req.symbol = Some(symbol.to_uppercase());
        }
        if let Some(side) = params.get("side").and_then(|v| v.as_str()) {
            req.side = Some(if side.to_uppercase() == "BUY" { TradeSide::Buy } else { TradeSide::Sell });
        }
        if let Some(vol) = params.get("volume") {
            req.volume = vol.as_f64();
        }
        if let Some(bot_id) = params.get("bot_id").and_then(|v| v.as_str()) {
            req.bot_id = bot_id.to_string();
        }
        
        // Handle Task Plan steps as messages
        if let Some(steps) = params.get("steps").and_then(|v| v.as_array()) {
            let step_list: Vec<String> = steps.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
            req.target_id = step_list.join(" | ");
        }
    }

    Ok(Some(req))
}
