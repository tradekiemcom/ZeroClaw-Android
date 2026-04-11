use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use sqlx::SqlitePool;
use teloxide::Bot;

use crate::config::Config;
use crate::models::{Account, Bot as TradingBot, Position};
use crate::ctrader::CtraderClient;

/// Shared application state - được truyền vào tất cả handlers
pub struct AppState {
    pub config: Config,
    pub db: SqlitePool,
    pub ctrader: Arc<CtraderClient>,
    pub telegram_bot: Option<Arc<Bot>>,

    // In-memory state (sync với DB)
    pub accounts: RwLock<HashMap<i64, Account>>,
    pub bots: RwLock<HashMap<String, TradingBot>>,
    pub positions: RwLock<Vec<Position>>,
}

impl AppState {
    pub async fn new(
        config: Config,
        db: SqlitePool,
    ) -> Arc<Self> {
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

        let accounts = RwLock::new(accounts_vec.into_iter().map(|a| (a.id, a)).collect());
        let bots = RwLock::new(bots_vec.into_iter().map(|b| (b.id.clone(), b)).collect());
        let positions = RwLock::new(positions_vec);

        Arc::new(Self {
            config,
            db,
            ctrader,
            telegram_bot,
            accounts,
            bots,
            positions,
        })
    }

    /// Kiểm tra user có phải admin không
    pub fn is_admin(&self, user_id: i64) -> bool {
        self.config.is_admin(user_id)
    }

    /// Lấy tất cả account IDs
    pub async fn all_account_ids(&self) -> Vec<i64> {
        self.accounts.read().await.keys().cloned().collect()
    }

    /// Enable/disable autotrade cho tất cả accounts
    pub async fn set_all_autotrade(&self, enabled: bool) -> anyhow::Result<()> {
        let mut accounts = self.accounts.write().await;
        for acc in accounts.values_mut() {
            acc.autotrade = enabled;
            crate::storage::update_account_autotrade(&self.db, acc.id, enabled).await?;
        }
        Ok(())
    }

    /// Enable/disable bot
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

    /// Thêm position vào state
    pub async fn add_position(&self, pos: Position) -> anyhow::Result<()> {
        crate::storage::save_position(&self.db, &pos).await?;
        self.positions.write().await.push(pos);
        Ok(())
    }

    /// Lấy danh sách positions đang mở
    pub async fn get_open_positions(&self) -> Vec<Position> {
        self.positions.read().await
            .iter()
            .filter(|p| p.status == crate::models::position::PositionStatus::Open)
            .cloned()
            .collect()
    }

    /// Lấy positions theo bot
    pub async fn get_positions_by_bot(&self, bot_id: &str) -> Vec<Position> {
        self.positions.read().await
            .iter()
            .filter(|p| p.bot_id == bot_id && p.status == crate::models::position::PositionStatus::Open)
            .cloned()
            .collect()
    }

    /// Bot summary text
    pub async fn format_bots_summary(&self) -> String {
        let bots = self.bots.read().await;
        if bots.is_empty() {
            return "❌ Chưa có bot nào được đăng ký.".to_string();
        }
        let mut lines = vec!["🤖 *Danh sách Bots:*".to_string()];
        for bot in bots.values() {
            let status = if bot.enabled { "✅" } else { "⏸️" };
            lines.push(format!("{} `{}` — {} | PnL: {:.2}", status, bot.id, bot.symbol, bot.daily_pnl));
        }
        lines.join("\n")
    }

    /// Account summary text
    pub async fn format_accounts_summary(&self) -> String {
        let accounts = self.accounts.read().await;
        if accounts.is_empty() {
            return "❌ Chưa có account nào được đăng ký.".to_string();
        }
        let mut lines = vec!["💼 *Danh sách Accounts:*".to_string()];
        for acc in accounts.values() {
            let auto = if acc.autotrade { "🟢 Auto" } else { "🔴 Manual" };
            lines.push(format!("#{} {} — {} | PnL: {:.2}", acc.id, acc.name, auto, acc.daily_pnl));
        }
        lines.join("\n")
    }
}
