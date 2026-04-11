use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use sqlx::SqlitePool;
use teloxide::Bot;
use chrono::Utc;

use crate::config::Config;
use crate::models::{Account, AccountType, Bot as TradingBot, Position, ApiClient};
use crate::ctrader::CtraderClient;

/// Thống kê hệ thống cho lệnh /status
#[derive(Debug, Default)]
pub struct SystemStatus {
    pub total_accounts: usize,
    pub active_accounts: usize,   // autotrade = true
    pub real_accounts: usize,
    pub demo_accounts: usize,
    pub connected_accounts: usize,
    pub total_real_balance: f64,  // Tổng vốn real accounts
    pub total_real_equity: f64,   // Equity hiện tại real
    pub total_float_profit: f64,  // Float P&L đang mở real
    pub total_daily_pnl: f64,     // Daily P&L tất cả accounts
    pub active_bots: usize,
    pub total_bots: usize,
    pub open_positions: usize,
    pub total_api_clients: usize,
    pub active_api_clients: usize,
    pub uptime_secs: u64,
}

/// Shared application state - Arc<AppState> được truyền vào tất cả handlers
pub struct AppState {
    pub config: Config,
    pub db: SqlitePool,
    pub ctrader: Arc<CtraderClient>,
    pub telegram_bot: Option<Arc<Bot>>,
    pub started_at: chrono::DateTime<Utc>,

    // In-memory state (sync với DB)
    pub accounts: RwLock<HashMap<i64, Account>>,
    pub bots: RwLock<HashMap<String, TradingBot>>,
    pub positions: RwLock<Vec<Position>>,
    pub api_clients: RwLock<HashMap<String, ApiClient>>, // key = api_key string
}

impl AppState {
    pub async fn new(config: Config, db: SqlitePool) -> Arc<Self> {
        let ctrader = Arc::new(CtraderClient::new(
            config.ctrader_client_id.clone(),
            config.ctrader_secret.clone(),
            config.ctrader_host.clone(),
            config.ctrader_port,
            config.is_mock(),
        ));

        let telegram_bot = Some(Arc::new(Bot::new(&config.telegram_bot_token)));

        // Load từ DB
        let accounts_vec = crate::storage::get_accounts(&db).await.unwrap_or_default();
        let bots_vec = crate::storage::get_bots(&db).await.unwrap_or_default();
        let positions_vec = crate::storage::get_open_positions(&db).await.unwrap_or_default();
        let clients_vec = crate::storage::get_api_clients(&db).await.unwrap_or_default();

        let accounts = RwLock::new(accounts_vec.into_iter().map(|a| (a.id, a)).collect());
        let bots = RwLock::new(bots_vec.into_iter().map(|b| (b.id.clone(), b)).collect());
        let positions = RwLock::new(positions_vec);
        // Index bằng api_key để lookup nhanh trong auth middleware
        let api_clients = RwLock::new(clients_vec.into_iter().map(|c| (c.api_key.clone(), c)).collect());

        Arc::new(Self {
            config,
            db,
            ctrader,
            telegram_bot,
            started_at: Utc::now(),
            accounts,
            bots,
            positions,
            api_clients,
        })
    }

    pub fn is_admin(&self, user_id: i64) -> bool {
        self.config.is_admin(user_id)
    }

    pub async fn all_account_ids(&self) -> Vec<i64> {
        self.accounts.read().await.keys().cloned().collect()
    }

    pub async fn set_all_autotrade(&self, enabled: bool) -> anyhow::Result<()> {
        let mut accounts = self.accounts.write().await;
        for acc in accounts.values_mut() {
            acc.autotrade = enabled;
            crate::storage::update_account_autotrade(&self.db, acc.id, enabled).await?;
        }
        Ok(())
    }

    pub async fn set_bot_enabled(&self, bot_id: &str, enabled: bool) -> anyhow::Result<bool> {
        let mut bots = self.bots.write().await;
        if let Some(bot) = bots.get_mut(bot_id) {
            bot.enabled = enabled;
            crate::storage::set_bot_enabled(&self.db, bot_id, enabled).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn add_position(&self, pos: Position) -> anyhow::Result<()> {
        crate::storage::save_position(&self.db, &pos).await?;
        self.positions.write().await.push(pos);
        Ok(())
    }

    pub async fn get_open_positions(&self) -> Vec<Position> {
        self.positions.read().await
            .iter()
            .filter(|p| p.status == crate::models::PositionStatus::Open)
            .cloned()
            .collect()
    }

    pub async fn get_positions_by_bot(&self, bot_id: &str) -> Vec<Position> {
        self.positions.read().await
            .iter()
            .filter(|p| p.bot_id == bot_id && p.status == crate::models::PositionStatus::Open)
            .cloned()
            .collect()
    }

    // ── System Status (cho /status) ───────────────────────────────────────────

    pub async fn get_system_status(&self) -> SystemStatus {
        let accounts = self.accounts.read().await;
        let bots = self.bots.read().await;
        let positions = self.positions.read().await;
        let clients = self.api_clients.read().await;

        let mut status = SystemStatus::default();
        status.uptime_secs = (Utc::now() - self.started_at).num_seconds().max(0) as u64;

        for acc in accounts.values() {
            status.total_accounts += 1;
            if acc.autotrade { status.active_accounts += 1; }
            if acc.connected { status.connected_accounts += 1; }
            if acc.is_real() {
                status.real_accounts += 1;
                status.total_real_balance += acc.balance;
                status.total_real_equity += acc.equity;
                status.total_float_profit += acc.float_profit;
            } else {
                status.demo_accounts += 1;
            }
            status.total_daily_pnl += acc.daily_pnl;
        }

        for bot in bots.values() {
            status.total_bots += 1;
            if bot.enabled { status.active_bots += 1; }
        }

        status.open_positions = positions.iter()
            .filter(|p| p.status == crate::models::PositionStatus::Open)
            .count();

        for client in clients.values() {
            status.total_api_clients += 1;
            if client.enabled { status.active_api_clients += 1; }
        }

        status
    }

    pub fn format_uptime(secs: u64) -> String {
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        if h > 0 { format!("{}h {}m {}s", h, m, s) }
        else if m > 0 { format!("{}m {}s", m, s) }
        else { format!("{}s", s) }
    }

    // ── Api Client Management ─────────────────────────────────────────────────

    /// Xác thực Bearer token, trả về ApiClient nếu hợp lệ
    pub async fn authenticate_key(&self, api_key: &str) -> Option<ApiClient> {
        // Trước tiên check master key từ config
        if api_key == self.config.api_key {
            return Some(ApiClient {
                id: "master".to_string(),
                name: "Master Key".to_string(),
                api_key: api_key.to_string(),
                source: "API".to_string(),
                enabled: true,
                description: Some("Built-in master key".to_string()),
                allowed_actions: vec![],
                request_count: 0,
                last_used_at: None,
                created_at: self.started_at,
            });
        }
        // Sau đó check api_clients
        let clients = self.api_clients.read().await;
        clients.get(api_key).filter(|c| c.enabled).cloned()
    }

    pub async fn add_api_client(&self, client: ApiClient) -> anyhow::Result<()> {
        crate::storage::insert_api_client(&self.db, &client).await?;
        self.api_clients.write().await.insert(client.api_key.clone(), client);
        Ok(())
    }

    pub async fn delete_api_client(&self, client_id: &str) -> anyhow::Result<bool> {
        let key_to_remove = {
            let clients = self.api_clients.read().await;
            clients.values().find(|c| c.id == client_id || c.id.starts_with(client_id))
                .map(|c| c.api_key.clone())
        };

        if let Some(key) = key_to_remove {
            crate::storage::delete_api_client(&self.db, client_id).await?;
            self.api_clients.write().await.remove(&key);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn set_client_enabled(&self, client_id: &str, enabled: bool) -> anyhow::Result<bool> {
        let key_to_update = {
            let clients = self.api_clients.read().await;
            clients.values().find(|c| c.id == client_id || c.id.starts_with(client_id))
                .map(|c| (c.api_key.clone(), c.id.clone()))
        };

        if let Some((key, id)) = key_to_update {
            crate::storage::set_api_client_enabled(&self.db, &id, enabled).await?;
            if let Some(client) = self.api_clients.write().await.get_mut(&key) {
                client.enabled = enabled;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn list_api_clients(&self) -> Vec<ApiClient> {
        let clients = self.api_clients.read().await;
        let mut list: Vec<_> = clients.values().cloned().collect();
        list.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        list
    }

    // ── Telegram formatted summaries ──────────────────────────────────────────

    pub async fn format_bots_summary(&self) -> String {
        let bots = self.bots.read().await;
        if bots.is_empty() { return "❌ Chưa có bot nào.".to_string(); }
        let mut lines = vec![format!("🤖 *Bots* ({}/{})", bots.values().filter(|b| b.enabled).count(), bots.len())];
        for bot in bots.values() {
            let st = if bot.enabled { "✅" } else { "⏸️" };
            lines.push(format!("{} `{}` — {} | P&L: {:.2}", st, bot.id, bot.symbol, bot.daily_pnl));
        }
        lines.join("\n")
    }

    pub async fn format_accounts_summary(&self) -> String {
        let accounts = self.accounts.read().await;
        if accounts.is_empty() { return "❌ Chưa có account.".to_string(); }
        let mut lines = vec![format!("💼 *Accounts* ({})", accounts.len())];
        for acc in accounts.values() {
            let auto = if acc.autotrade { "🟢" } else { "🔴" };
            let atype = if acc.is_real() { "REAL" } else { "DEMO" };
            lines.push(format!("{} #{} {} [{}] | P&L: {:.2}", auto, acc.id, acc.name, atype, acc.daily_pnl));
        }
        lines.join("\n")
    }
}
