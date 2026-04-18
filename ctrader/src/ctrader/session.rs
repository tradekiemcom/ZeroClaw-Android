use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn, error};
use chrono::Utc;
use crate::models::{OrderRequest, PriceQuote, Position};
use crate::state::AppState;

/// Trạng thái kết nối của một Session
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

/// Lệnh gửi vào Session từ Engine
#[derive(Debug)]
pub enum SessionCommand {
    ExecuteOrder(OrderRequest, tokio::sync::oneshot::Sender<crate::ctrader::ExecutionResult>),
    ClosePosition(String, tokio::sync::oneshot::Sender<bool>), // pos_id
    SubscribeSymbol(String),
    Heartbeat,
    Disconnect,
}

pub struct AccountSession {
    pub account_id: i64,
    pub is_mock: bool,
    pub status: Arc<RwLock<SessionStatus>>,
    cmd_tx: mpsc::Sender<SessionCommand>,
}

impl AccountSession {
    pub fn new(account_id: i64, is_mock: bool, state: Arc<AppState>) -> Arc<Self> {
        let (cmd_tx, cmd_rx) = mpsc::channel(100);
        let status = Arc::new(RwLock::new(SessionStatus::Disconnected));
        
        let session = Arc::new(Self {
            account_id,
            is_mock,
            status: status.clone(),
            cmd_tx,
        });

        // Khởi động vòng lặp xử lý chính
        let session_clone = session.clone();
        tokio::spawn(async move {
            session_clone.run_loop(cmd_rx, state).await;
        });

        session
    }

    /// Gửi lệnh vào session
    pub async fn send_command(&self, cmd: SessionCommand) -> anyhow::Result<()> {
        self.cmd_tx.send(cmd).await
            .map_err(|e| anyhow::anyhow!("Failed to send command to session {}: {}", self.account_id, e))
    }

    /// Vòng lặp xử lý chính
    async fn run_loop(&self, mut cmd_rx: mpsc::Receiver<SessionCommand>, state: Arc<AppState>) {
        info!("[START] Session {} started (Mock: {})", self.account_id, self.is_mock);
        
        *self.status.write().await = SessionStatus::Connected;

        if self.is_mock {
            self.run_mock_loop(cmd_rx, state).await;
        } else {
            self.run_live_loop(cmd_rx, state).await;
        }

        *self.status.write().await = SessionStatus::Disconnected;
        info!("[STOP] Session {} stopped", self.account_id);
    }

    /// Vòng lặp cho chế độ Mock
    async fn run_mock_loop(&self, mut cmd_rx: mpsc::Receiver<SessionCommand>, state: Arc<AppState>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(2000)); // 2s cập nhật giá
        
        loop {
            tokio::select! {
                // Nhận lệnh từ Engine
                Some(cmd) = cmd_rx.recv() => {
                    match cmd {
                        SessionCommand::ExecuteOrder(req, resp_tx) => {
                            info!("[MOCK-{}] Executing order: {:?}", self.account_id, req);
                            
                            // Giả lập khớp lệnh ngay lập tức
                            let symbol = req.symbol.clone().unwrap_or("XAUUSD".to_string());
                            let side = req.side.clone().unwrap_or(crate::models::TradeSide::Buy);
                            let order_id = format!("mock_ord_{}", Utc::now().timestamp_millis());

                            let mut pos = crate::models::Position {
                                id: format!("mock_pos_{}", Utc::now().timestamp_millis()),
                                order_id: order_id.clone(),
                                account_id: self.account_id,
                                bot_id: req.bot_id.clone(),
                                source: req.source.to_string(),
                                symbol: symbol.clone(),
                                side: if side == crate::models::TradeSide::Buy { "BUY".to_string() } else { "SELL".to_string() },
                                volume: req.volume.unwrap_or(0.01),
                                open_price: 0.0,
                                sl: req.sl,
                                tp: req.tp,
                                pnl: 0.0,
                                status: crate::models::PositionStatus::Open,
                                opened_at: Utc::now(),
                                closed_at: None,
                            };

                            pos.open_price = if let Some(q) = state.get_price(&symbol).await {
                                if side == crate::models::TradeSide::Buy { q.ask } else { q.bid }
                            } else { 1.0 };

                            let res = crate::ctrader::ExecutionResult {
                                success: true,
                                order_id: order_id,
                                message: "Mock order executed".to_string(),
                                position: Some(pos),
                            };
                            let _ = resp_tx.send(res);
                        }
                        SessionCommand::ClosePosition(_pos_id, resp_tx) => {
                            info!("[MOCK-{}] Closing positions", self.account_id);
                            let _ = resp_tx.send(true);
                        }
                        SessionCommand::Disconnect => break,
                        _ => {}
                    }
                }
                
                // Giả lập Price Feed
                _ = interval.tick() => {
                    // Update mock prices cho các cặp phổ biến
                    let symbols = vec!["XAUUSD", "BTCUSD", "EURUSD"];
                    for sym in symbols {
                        if let Some(mut quote) = state.get_price(sym).await {
                            // Biến động nhẹ +- 0.01%
                            let change = quote.bid * 0.0001 * (rand::random::<f32>() as f64 - 0.5);
                            quote.bid += change;
                            quote.ask += change;
                            quote.timestamp = Utc::now();
                            state.update_price(quote).await;
                        }
                    }
                }
            }
        }
    }

    /// Vòng lặp cho chế độ Live
    async fn run_live_loop(&self, mut cmd_rx: mpsc::Receiver<SessionCommand>, state: Arc<AppState>) {
        info!("[LIVE-{}] Running with cTrader Spotware API integration", self.account_id);
        
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
        let client = reqwest::Client::new();

        loop {
            tokio::select! {
                Some(cmd) = cmd_rx.recv() => {
                    if matches!(cmd, SessionCommand::Disconnect) { break; }
                }
                
                _ = interval.tick() => {
                    if let Err(e) = self.sync_account_data(&client, &state).await {
                        error!("[ERROR] [LIVE-{}] Sync error: {}", self.account_id, e);
                    }
                }
            }
        }
    }

    async fn sync_account_data(&self, client: &reqwest::Client, state: &Arc<AppState>) -> anyhow::Result<()> {
        let accounts = state.accounts.read().await;
        let Some(acc) = accounts.get(&self.account_id) else { return Ok(()); };
        let Some(token) = &acc.access_token else { return Ok(()); };

        // 1. Sync Balance & Account Info
        let url = format!("https://api.spotware.com/connect/tradingaccounts?access_token={}", token);
        let resp: serde_json::Value = client.get(url).send().await?.json().await?;
        
        if let Some(data) = resp["data"].as_array() {
            if let Some(acc_data) = data.iter().find(|a| a["accountId"] == self.account_id) {
                let balance = acc_data["balance"].as_f64().unwrap_or(0.0) / 100.0;
                let mut accounts_write = state.accounts.write().await;
                if let Some(acc_entry) = accounts_write.get_mut(&self.account_id) {
                    acc_entry.balance = balance;
                }
                // Update DB
                let _ = crate::storage::update_account_equity(&state.db, self.account_id, balance, 0.0).await;
            }
        }

        // 2. Sync Positions
        let pos_url = format!("https://api.spotware.com/connect/tradingaccounts/{}/positions?access_token={}", self.account_id, token);
        let pos_resp = client.get(pos_url).send().await?;
        if pos_resp.status().is_success() {
            let pos_data: serde_json::Value = pos_resp.json().await?;
            if let Some(items) = pos_data["data"].as_array() {
                let mut current_positions = Vec::new();
                for item in items {
                    let side = if item["tradeSide"].as_str() == Some("BUY") { "BUY" } else { "SELL" };
                    let pos = Position {
                        id: item["positionId"].to_string(),
                        order_id: item["positionId"].to_string(),
                        account_id: self.account_id,
                        bot_id: "manual".to_string(),
                        source: "cTrader".to_string(),
                        symbol: item["symbolName"].as_str().unwrap_or("XAUUSD").to_string(),
                        side: side.to_string(),
                        volume: item["volume"].as_f64().unwrap_or(0.0) / 100000.0,
                        open_price: item["entryPrice"].as_f64().unwrap_or(0.0),
                        sl: item["stopLoss"].as_f64(),
                        tp: item["takeProfit"].as_f64(),
                        pnl: item["unrealizedGrossProfit"].as_f64().unwrap_or(0.0) / 100.0,
                        status: crate::models::PositionStatus::Open,
                        opened_at: Utc::now(),
                        closed_at: None,
                    };
                    current_positions.push(pos);
                }
                // Cập nhật bộ nhớ
                let mut positions_write = state.positions.write().await;
                positions_write.retain(|p| p.account_id != self.account_id);
                positions_write.extend(current_positions);
            }
        }

        Ok(())
    }
}
