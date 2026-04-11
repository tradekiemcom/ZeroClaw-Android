use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Loại tài khoản cTrader
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Real,
    Demo,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Real => write!(f, "REAL"),
            AccountType::Demo => write!(f, "DEMO"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub broker_account_id: i64,  // cTrader account ID
    pub account_type: AccountType,
    pub access_token: Option<String>,
    pub connected: bool,
    pub autotrade: bool,

    // Tài chính
    pub balance: f64,       // Vốn ban đầu/số dư tài khoản
    pub equity: f64,        // Equity hiện tại (balance + float)
    pub float_profit: f64,  // Lợi nhuận nổi đang mở
    pub daily_pnl: f64,     // P&L trong ngày hôm nay

    // Risk limits
    pub daily_target_profit: f64,  // 0 = không giới hạn
    pub daily_max_loss: f64,        // 0 = không giới hạn

    pub created_at: DateTime<Utc>,
}

impl Account {
    pub fn new(id: i64, name: String, broker_account_id: i64, account_type: AccountType) -> Self {
        Self {
            id,
            name,
            broker_account_id,
            account_type,
            access_token: None,
            connected: false,
            autotrade: true,
            balance: 0.0,
            equity: 0.0,
            float_profit: 0.0,
            daily_pnl: 0.0,
            daily_target_profit: 0.0,
            daily_max_loss: 0.0,
            created_at: Utc::now(),
        }
    }

    pub fn is_real(&self) -> bool {
        self.account_type == AccountType::Real
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
