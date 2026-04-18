use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, error};
use crate::state::AppState;
use crate::ctrader::session::{AccountSession, SessionCommand};
use crate::models::OrderRequest;

pub struct ConnectionPool {
    sessions: RwLock<HashMap<i64, Arc<AccountSession>>>,
}

impl ConnectionPool {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Đăng ký và khởi chạy một session mới cho account
    pub async fn connect_account(&self, account_id: i64, is_mock: bool, state: Arc<AppState>) -> anyhow::Result<()> {
        let mut sessions = self.sessions.write().await;
        
        // Nếu đã có session đang chạy, ngắt kết nối cũ trước
        if let Some(old_session) = sessions.remove(&account_id) {
            let _ = old_session.send_command(SessionCommand::Disconnect).await;
        }

        let session = AccountSession::new(account_id, is_mock, state);
        sessions.insert(account_id, session);
        
        info!("💠 Account {} added to ConnectionPool (Mock: {})", account_id, is_mock);
        Ok(())
    }

    /// Hủy kết nối một account
    pub async fn disconnect_account(&self, account_id: i64) -> anyhow::Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.remove(&account_id) {
            session.send_command(SessionCommand::Disconnect).await?;
            info!("💠 Account {} removed from ConnectionPool", account_id);
        }
        Ok(())
    }

    /// Gửi lệnh tới một account cụ thể và chờ kết quả
    pub async fn execute_order(&self, account_id: i64, req: OrderRequest) -> anyhow::Result<crate::ctrader::ExecutionResult> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(&account_id) {
            session.send_command(SessionCommand::ExecuteOrder(req, tx)).await?;
            let res = rx.await.map_err(|_| anyhow::anyhow!("Session {} dropped result channel", account_id))?;
            Ok(res)
        } else {
            error!("❌ Cannot execute order: No active session for account {}", account_id);
            anyhow::bail!("No active session for account {}", account_id)
        }
    }

    /// Đóng lệnh và chờ kết quả
    pub async fn close_order(&self, account_id: i64, order_id: &str) -> anyhow::Result<bool> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(&account_id) {
            session.send_command(SessionCommand::ClosePosition(order_id.to_string(), tx)).await?;
            let res = rx.await.map_err(|_| anyhow::anyhow!("Session {} dropped result channel", account_id))?;
            Ok(res)
        } else {
            Ok(false)
        }
    }

    /// Lấy trạng thái của một session
    pub async fn get_session_status(&self, account_id: i64) -> crate::ctrader::session::SessionStatus {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(&account_id) {
            *session.status.read().await
        } else {
            crate::ctrader::session::SessionStatus::Disconnected
        }
    }

    /// Khởi tạo kết nối cho tất cả tài khoản đang hoạt động trong DB
    pub async fn connect_all(&self, state: Arc<AppState>) -> anyhow::Result<()> {
        let accounts = crate::storage::get_accounts(&state.db).await?;
        let is_mock = state.config.is_mock();
        for acc in accounts {
            if let Err(e) = self.connect_account(acc.id, is_mock, state.clone()).await {
                error!("❌ Failed to connect account {}: {}", acc.id, e);
            }
        }
        Ok(())
    }

    /// Đóng tất cả kết nối (khi shutdown app)
    pub async fn shutdown(&self) {
        let mut sessions = self.sessions.write().await;
        for (_, session) in sessions.drain() {
            let _ = session.send_command(SessionCommand::Disconnect).await;
        }
    }
}
