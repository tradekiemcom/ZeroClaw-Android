use crate::models::{OrderRequest, OrderAction, TradeSource, TradeSide};
use crate::models::order::AccountScope;

pub struct IzParser;

impl IzParser {
    pub fn parse(text: &str, source: TradeSource) -> Option<OrderRequest> {
        let text = text.trim();
        if text.is_empty() { return None; }

        // 1. Slash Commands
        if text.starts_with('/') {
            let parts: Vec<&str> = text.split_whitespace().collect();
            let cmd = parts[0].to_lowercase();
            
            let action = match cmd.as_str() {
                "/a" | "/autoon"  => Some(OrderAction::EnableAutotrade),
                "/d" | "/autooff" => Some(OrderAction::DisableAutotrade),
                "/on"  => Some(OrderAction::EnableBot),
                "/off" => Some(OrderAction::DisableBot),
                "/l" | "/list"      => Some(OrderAction::ListAccounts),
                "/p" | "/positions" => Some(OrderAction::ListPositions),
                "/o" | "/pending"   => Some(OrderAction::ListPending),
                "/c" | "/closeall"  => Some(OrderAction::CloseAll),
                _ => None,
            };

            if let Some(act) = action {
                let mut req = OrderRequest::new(source, "slash".to_string(), act);
                if parts.len() > 1 { req.target_id = parts[1].to_string(); }
                return Some(req);
            }
            return None;
        }

        // 2. Trading Commands (#BUY, #SELL...)
        if !text.starts_with('#') { return None; }
        let parts: Vec<&str> = text.split_whitespace().collect();
        let cmd = parts[0].to_uppercase();
        
        let action = match cmd.as_str() {
            "#BUY" => OrderAction::Open,
            "#SELL" => OrderAction::Open,
            "#CLOSE" => OrderAction::Close,
            _ => return None,
        };

        let mut req = OrderRequest::new(source, "manual".to_string(), action);
        if cmd.contains("BUY") { req.side = Some(TradeSide::Buy); }
        if cmd.contains("SELL") { req.side = Some(TradeSide::Sell); }
        
        if parts.len() > 1 { req.symbol = Some(parts[1].to_uppercase()); }
        if parts.len() > 2 { req.volume = parts[2].parse().ok(); }

        Some(req)
    }
}
