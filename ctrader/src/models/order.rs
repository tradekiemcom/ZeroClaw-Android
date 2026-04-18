use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradeSource {
    Api,
    Telegram,
    Cli,
}

impl fmt::Display for TradeSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TradeSide {
    Buy,
    Sell,
}

impl fmt::Display for TradeSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountScope {
    Single,
    All,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderAction {
    // Trading
    Open,
    Close,
    CloseProfit,
    CloseLoss,
    CloseBuy,
    CloseSell,
    Modify,
    Delete,
    BuyLimit,
    SellLimit,
    BuyStop,
    SellStop,
    
    // Bot/Account Management
    EnableBot,
    DisableBot,
    EnableAutotrade,
    DisableAutotrade,
    CloseAll,
    ListAccounts,
    ListPositions,
    ListPending,

    // NEW: System & Admin Management
    AddAccount,
    DeleteAccount,
    UpdateAccount,
    AddApiClient,
    DeleteApiClient,
    ListApiClients,
    SystemStatus,
    SystemCleanup,
    SystemNuclearWipe,
    AccountReport,
    BotReport,
    ListGrouped,
    CloseBotPositions,
    AgentOn,
    AgentOff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub request_id: String,
    pub timestamp: u64,
    pub source: TradeSource,
    pub account_scope: AccountScope,
    pub account_ids: Vec<i64>,
    pub bot_id: String,
    pub action: OrderAction,
    pub target_id: String,
    pub symbol: Option<String>,
    pub side: Option<TradeSide>,
    pub volume: Option<f64>,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
    pub price: Option<f64>,
    pub grid_count: Option<i32>,
    pub grid_step: Option<i32>,
    pub lot_multiplier: Option<f64>,
    pub expiration_life: Option<i32>,
}

impl OrderRequest {
    pub fn new(source: TradeSource, bot_id: String, action: OrderAction) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            request_id: Uuid::new_v4().to_string(),
            timestamp: now,
            source,
            account_scope: AccountScope::All,
            account_ids: vec![],
            bot_id,
            action,
            target_id: String::new(),
            symbol: None,
            side: None,
            volume: None,
            tp: None,
            sl: None,
            price: None,
            grid_count: None,
            grid_step: None,
            lot_multiplier: None,
            expiration_life: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub request_id: String,
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl OrderResponse {
    pub fn success(request_id: String, message: String) -> Self {
        Self { request_id, success: true, message, data: None }
    }
    pub fn error(request_id: String, message: String) -> Self {
        Self { request_id, success: false, message, data: None }
    }
    pub fn to_telegram_msg(&self) -> String {
        let status = if self.success { "[SUCCESS]" } else { "[ERROR]" };
        format!("{} Xử lý lệnh\n━━━━━━━━━━━━━\n{}", status, self.message)
    }
}
