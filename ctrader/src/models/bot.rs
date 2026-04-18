use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    pub id: String,         // Unique id, ex: "gold_scalper"
    pub name: String,       // Display name
    pub account_id: i64,    // Owner account
    pub enabled: bool,
    pub symbol: String,     // ex: "XAUUSD"
    pub timeframe: String,  // ex: "M15"
    pub daily_target_profit: f64,
    pub daily_max_loss: f64,
    pub daily_pnl: f64,
    pub trade_count_today: i32,
    pub created_at: DateTime<Utc>,
}

impl Bot {
    pub fn new(id: String, name: String, account_id: i64, symbol: String) -> Self {
        Self {
            id,
            name,
            account_id,
            enabled: true,
            symbol,
            timeframe: "M15".to_string(),
            daily_target_profit: 0.0,
            daily_max_loss: 0.0,
            daily_pnl: 0.0,
            trade_count_today: 0,
            created_at: Utc::now(),
        }
    }

    pub fn should_halt(&self) -> Option<String> {
        if self.daily_target_profit > 0.0 && self.daily_pnl >= self.daily_target_profit {
            return Some(format!(
                "Bot {} đạt target profit: {:.2}",
                self.id, self.daily_pnl
            ));
        }
        if self.daily_max_loss > 0.0 && self.daily_pnl <= -self.daily_max_loss {
            return Some(format!(
                "Bot {} vượt max loss: {:.2}",
                self.id, self.daily_pnl
            ));
        }
        None
    }
}
