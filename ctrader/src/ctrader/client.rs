use anyhow::Result;
use tracing::{info, warn};
use crate::models::order::{OrderRequest, OrderAction, TradeSide};
use crate::models::position::{Position, PositionStatus};
use chrono::Utc;
use uuid::Uuid;

/// Kết quả từ cTrader sau khi execute lệnh
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub order_id: String,
    pub message: String,
    pub position: Option<Position>,
}

impl ExecutionResult {
    pub fn success(order_id: String, position: Position) -> Self {
        Self {
            success: true,
            order_id,
            message: "Order executed successfully".to_string(),
            position: Some(position),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            order_id: String::new(),
            message,
            position: None,
        }
    }

    pub fn mock(req: &OrderRequest, account_id: i64) -> Self {
        let order_id = Uuid::new_v4().to_string();
        let symbol = req.symbol.clone().unwrap_or("XAUUSD".to_string());
        let side = req.side.as_ref().map(|s| s.to_string()).unwrap_or("BUY".to_string());
        let volume = req.volume.unwrap_or(0.01);

        let position = Position {
            id: Uuid::new_v4().to_string(),
            order_id: order_id.clone(),
            account_id,
            bot_id: req.bot_id.clone(),
            source: req.source.to_string(),
            symbol: symbol.clone(),
            side: side.clone(),
            volume,
            open_price: mock_price(&symbol, &side),
            sl: req.sl,
            tp: req.tp,
            pnl: 0.0,
            status: PositionStatus::Open,
            opened_at: Utc::now(),
            closed_at: None,
        };

        Self::success(order_id, position)
    }
}

/// Client kết nối cTrader
/// - Mode "mock": Log lệnh, return giả lập position
/// - Mode "live": Kết nối qua TCP + Protobuf (phase 2)
pub struct CtraderClient {
    pub client_id: String,
    pub secret: String,
    pub host: String,
    pub port: u16,
    pub is_mock: bool,
}

impl CtraderClient {
    pub fn new(client_id: String, secret: String, host: String, port: u16, is_mock: bool) -> Self {
        Self { client_id, secret, host, port, is_mock }
    }

    /// Kết nối và xác thực Application
    pub async fn connect(&self) -> Result<()> {
        if self.is_mock {
            info!("🔌 [MOCK] cTrader client initialized - Mock mode active");
            info!("   Client ID: {}...", &self.client_id[..20.min(self.client_id.len())]);
            return Ok(());
        }

        info!("🔌 Connecting to cTrader {}:{}", self.host, self.port);
        // TODO Phase 2: Implement TCP + Protobuf connection
        // 1. TLS TCP connect to host:port
        // 2. Send ProtoOAApplicationAuthReq { clientId, clientSecret }
        // 3. Wait for ProtoOAApplicationAuthRes
        warn!("⚠️  Live cTrader connection not yet implemented. Falling back to mock.");
        Ok(())
    }

    /// Thực thi lệnh trade
    pub async fn execute(&self, req: &OrderRequest, account_id: i64) -> Result<ExecutionResult> {
        if self.is_mock {
            return self.execute_mock(req, account_id).await;
        }
        // TODO Phase 2: Send ProtoOANewOrderReq via TCP
        self.execute_mock(req, account_id).await
    }

    async fn execute_mock(&self, req: &OrderRequest, account_id: i64) -> Result<ExecutionResult> {
        info!(
            "📤 [MOCK] Execute | Action: {:?} | Bot: {} | Account: {} | Symbol: {:?} | Side: {:?} | Vol: {:?}",
            req.action, req.bot_id, account_id, req.symbol, req.side, req.volume
        );
        Ok(ExecutionResult::mock(req, account_id))
    }

    /// Đóng lệnh theo order_id
    pub async fn close_order(&self, account_id: i64, order_id: &str, bot_id: &str) -> Result<bool> {
        if self.is_mock {
            info!("📤 [MOCK] Close | Account: {} | OrderID: {} | Bot: {}", account_id, order_id, bot_id);
            return Ok(true);
        }
        // TODO Phase 2: Send ProtoOAClosePositionReq
        Ok(true)
    }

    /// Đóng tất cả lệnh của một bot
    pub async fn close_all_by_bot(&self, account_id: i64, bot_id: &str) -> Result<i32> {
        info!("📤 [MOCK] CloseAll | Account: {} | Bot: {}", account_id, bot_id);
        Ok(0) // số lệnh đã đóng
    }
}

/// Giá mock dựa trên symbol để test realistic
fn mock_price(symbol: &str, side: &str) -> f64 {
    let base = match symbol.to_uppercase().as_str() {
        "XAUUSD" | "GOLD" => 3300.0,
        "BTCUSD" => 85000.0,
        "EURUSD" => 1.085,
        "GBPUSD" => 1.265,
        "USDJPY" => 149.5,
        _ => 1.0,
    };
    if side.to_uppercase() == "BUY" { base } else { base - base * 0.0001 }
}
