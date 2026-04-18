use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub order_id: String,
    pub account_id: i64,
    pub bot_id: String,
    pub source: String,
    pub symbol: String,
    pub side: String,
    pub volume: f64,
    pub open_price: f64,
    pub sl: Option<f64>,
    pub tp: Option<f64>,
    pub pnl: f64,
    pub status: PositionStatus,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PositionStatus {
    Open,
    Closed,
    Cancelled,
}

impl std::fmt::Display for PositionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionStatus::Open => write!(f, "open"),
            PositionStatus::Closed => write!(f, "closed"),
            PositionStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl Position {
    pub fn format_telegram(&self) -> String {
        format!(
            "*{}* | {}\n\
            Bot: `{}`\n\
            Action: {} {} @ {:.5}\n\
            Volume: {:.2}\n\
            SL: {} | TP: {}\n\
            Time: {}",
            self.symbol,
            self.source,
            self.bot_id,
            self.side.to_uppercase(),
            self.symbol,
            self.open_price,
            self.volume,
            self.sl.map(|v| format!("{:.5}", v)).unwrap_or("—".to_string()),
            self.tp.map(|v| format!("{:.5}", v)).unwrap_or("—".to_string()),
            self.opened_at.format("%Y-%m-%d %H:%M:%S UTC"),
        )
    }
}
