use crate::models::{OrderRequest, OrderAction, TradeSource, TradeSide};
use regex::Regex;
use once_cell::sync::Lazy;

static BOT_CMD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^/([adc])(\d+)$").unwrap());

pub struct IzParser;

impl IzParser {
    pub fn parse(text: &str, source: TradeSource) -> Option<OrderRequest> {
        let text = text.trim();
        if text.is_empty() { return None; }

        // 0. Shorthand Bot Commands (/a1, /d1, /c1)
        if let Some(caps) = BOT_CMD_RE.captures(text) {
            let action_char = caps.get(1).map(|m| m.as_str()).unwrap();
            let index = caps.get(2).map(|m| m.as_str()).unwrap();
            
            let action = match action_char {
                "a" => OrderAction::EnableBot,
                "d" => OrderAction::DisableBot,
                "c" => OrderAction::CloseBotPositions,
                _ => return None,
            };

            let mut req = OrderRequest::new(source, "shorthand".to_string(), action);
            req.target_id = index.to_string(); // We'll resolve the index in core.rs
            return Some(req);
        }

        let parts: Vec<&str> = text.split_whitespace().collect();
        let first_word = parts[0].to_lowercase();
        
        // 1. Slash Commands (/a, /list, etc.)
        if text.starts_with('/') {
            let cmd = &first_word[1..];
            let action = match cmd {
                "a" | "autoon"  => Some(OrderAction::EnableAutotrade),
                "d" | "autooff" => Some(OrderAction::DisableAutotrade),
                "on"   => Some(OrderAction::EnableBot),
                "off"  => Some(OrderAction::DisableBot),
                "l" | "list" | "acc" => Some(OrderAction::ListAccounts),
                "p" | "pos" | "positions" => Some(OrderAction::ListPositions),
                "o" | "pending" => Some(OrderAction::ListPending),
                "c" | "closeall" => Some(OrderAction::CloseAll),
                "r" | "report" => Some(OrderAction::ListGrouped),
                "rb" => Some(OrderAction::BotReport),
                "status" => Some(OrderAction::SystemStatus),
                "agent" => {
                    match parts.get(1).map(|&s| s.to_lowercase()).as_deref() {
                        Some("on") => Some(OrderAction::AgentOn),
                        Some("off") => Some(OrderAction::AgentOff),
                        _ => Some(OrderAction::AgentOn),
                    }
                }
                "key" | "api" => Some(OrderAction::ListApiClients),
                "sys" => {
                    match parts.get(1).map(|&s| s.to_lowercase()).as_deref() {
                        Some("cleanup") => Some(OrderAction::SystemCleanup),
                        Some("status") => Some(OrderAction::SystemStatus),
                        Some("nuclear") => Some(OrderAction::SystemNuclearWipe),
                        _ => Some(OrderAction::SystemStatus),
                    }
                }
                _ => None,
            };

            if let Some(act) = action {
                let mut req = OrderRequest::new(source, "slash".to_string(), act.clone());
                if parts.len() > 1 && !matches!(act, OrderAction::SystemCleanup | OrderAction::SystemNuclearWipe) { 
                    req.target_id = parts[1].to_string(); 
                }
                return Some(req);
            }
        }

        // 2. Trading Commands (#BUY, #SELL...)
        if text.starts_with('#') {
            let cmd = first_word.to_uppercase();
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

            return Some(req);
        }

        // 3. Plain Text Commands (list, pos, status, acc, api, sys)
        let action = match first_word.as_str() {
            "l" | "list" => Some(OrderAction::ListAccounts),
            "p" | "pos" => Some(OrderAction::ListPositions),
            "r" | "report" | "status" | "st" => Some(OrderAction::SystemStatus),
            "a" | "on" => Some(OrderAction::EnableAutotrade),
            "d" | "off" => Some(OrderAction::DisableAutotrade),
            "c" | "close" => Some(OrderAction::CloseAll),
            "acc" => {
                match parts.get(1).map(|&s| s.to_lowercase()).as_deref() {
                    Some("list") | Some("l") => Some(OrderAction::ListAccounts),
                    Some("status") | Some("st") | Some("r") => Some(OrderAction::AccountReport),
                    Some("add") => Some(OrderAction::AddAccount),
                    Some("del") | Some("rm") => Some(OrderAction::DeleteAccount),
                    _ => Some(OrderAction::ListAccounts),
                }
            }
            "api" => {
                match parts.get(1).map(|&s| s.to_lowercase()).as_deref() {
                    Some("list") | Some("l") => Some(OrderAction::ListApiClients),
                    Some("add") => Some(OrderAction::AddApiClient),
                    Some("del") | Some("rm") => Some(OrderAction::DeleteApiClient),
                    _ => Some(OrderAction::ListApiClients),
                }
            }
            "sys" => {
                match parts.get(1).map(|&s| s.to_lowercase()).as_deref() {
                    Some("cleanup") => Some(OrderAction::SystemCleanup),
                    Some("status") => Some(OrderAction::SystemStatus),
                    Some("nuclear") => Some(OrderAction::SystemNuclearWipe),
                    _ => Some(OrderAction::SystemStatus),
                }
            }
            _ => None,
        };

        if let Some(act) = action {
            let mut req = OrderRequest::new(source, "manual".to_string(), act);
            // Handling for subcommands that need target_id
            if parts.len() > 2 {
                req.target_id = parts[2].to_string();
            } else if parts.len() > 1 && !matches!(first_word.as_str(), "acc" | "api" | "sys") {
                req.target_id = parts[1].to_string();
            }
            return Some(req);
        }

        None
    }
}
