use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub broker_account_id: i64,  // cTrader account ID
    pub access_token: Option<String>,
    pub connected: bool,
    pub autotrade: bool,
    pub daily_target_profit: f64,  // 0 = không giới hạn
    pub daily_max_loss: f64,        // 0 = không giới hạn
    pub daily_pnl: f64,
    pub created_at: DateTime<Utc>,
}

impl Account {
    pub fn new(id: i64, name: String, broker_account_id: i64) -> Self {
        Self {
            id,
            name,
            broker_account_id,
            access_token: None,
            connected: false,
            autotrade: true,
            daily_target_profit: 0.0,
            daily_max_loss: 0.0,
            daily_pnl: 0.0,
            created_at: Utc::now(),
        }
    }

    /// Kiểm tra xem có nên dừng trading vì đạt target hay max loss
    pub fn should_halt_trading(&self) -> Option<String> {
        if self.daily_target_profit > 0.0 && self.daily_pnl >= self.daily_target_profit {
            return Some(format!(
                "Daily target profit đạt: {:.2} >= {:.2}",
                self.daily_pnl, self.daily_target_profit
            ));
        }
        if self.daily_max_loss > 0.0 && self.daily_pnl <= -self.daily_max_loss {
            return Some(format!(
                "Daily max loss vượt: {:.2} <= -{:.2}",
                self.daily_pnl, self.daily_max_loss
            ));
        }
        None
    }
}
