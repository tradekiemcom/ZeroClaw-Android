use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeSource {
    Telegram,
    Api,
    Web,
    Zeroclaw,
    Mt5,
}

impl std::fmt::Display for TradeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeSource::Telegram => write!(f, "TELEGRAM"),
            TradeSource::Api => write!(f, "API"),
            TradeSource::Web => write!(f, "WEB"),
            TradeSource::Zeroclaw => write!(f, "ZEROCLAW"),
            TradeSource::Mt5 => write!(f, "MT5"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountScope {
    All,
    Single,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderAction {
    Open,
    Close,
    CloseAll,
    Modify,
    EnableBot,
    DisableBot,
    EnableAutotrade,
    DisableAutotrade,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TradeSide {
    Buy,
    Sell,
}

impl std::fmt::Display for TradeSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeSide::Buy => write!(f, "BUY"),
            TradeSide::Sell => write!(f, "SELL"),
        }
    }
}

/// Order chuẩn hóa – tất cả lệnh đều đi qua struct này
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub request_id: String,
    pub source: TradeSource,
    pub account_scope: AccountScope,
    pub account_ids: Vec<i64>,
    pub bot_id: String,
    pub action: OrderAction,
    pub symbol: Option<String>,
    pub side: Option<TradeSide>,
    pub volume: Option<f64>,
    pub sl: Option<f64>,
    pub tp: Option<f64>,
    pub price: Option<f64>,  // Cho lệnh pending
    pub created_at: DateTime<Utc>,
}

impl OrderRequest {
    pub fn new(source: TradeSource, bot_id: String, action: OrderAction) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            source,
            account_scope: AccountScope::All,
            account_ids: vec![],
            bot_id,
            action,
            symbol: None,
            side: None,
            volume: None,
            sl: None,
            tp: None,
            price: None,
            created_at: Utc::now(),
        }
    }

    pub fn market_order(
        source: TradeSource,
        bot_id: String,
        symbol: String,
        side: TradeSide,
        volume: f64,
        sl: Option<f64>,
        tp: Option<f64>,
    ) -> Self {
        let mut req = Self::new(source, bot_id, OrderAction::Open);
        req.symbol = Some(symbol);
        req.side = Some(side);
        req.volume = Some(volume);
        req.sl = sl;
        req.tp = tp;
        req
    }
}
