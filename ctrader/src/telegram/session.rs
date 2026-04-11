use std::collections::HashMap;
use tokio::sync::RwLock;

/// Session của mỗi Telegram user — lưu cấp độ đang thao tác
#[derive(Debug, Clone)]
pub struct UserSession {
    pub user_id: i64,
    /// None = App scope (tất cả accounts)
    /// Some(id) = Account scope (account cụ thể)
    pub account_id: Option<i64>,
    /// Tên account đang xem (để hiển thị)
    pub account_name: Option<String>,
    /// Menu/view đang active (để biết context của keyboard)
    pub current_view: CurrentView,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentView {
    Menu,
    Positions,
    Pending,
    Bots,
    Report,
    Keys,
}

impl Default for UserSession {
    fn default() -> Self {
        Self {
            user_id: 0,
            account_id: None,
            account_name: None,
            current_view: CurrentView::Menu,
        }
    }
}

impl UserSession {
    pub fn new(user_id: i64) -> Self {
        Self { user_id, ..Default::default() }
    }

    pub fn is_app_scope(&self) -> bool {
        self.account_id.is_none()
    }

    /// Format scope badge — hiển thị đầu mọi tin nhắn
    pub fn scope_badge(&self) -> String {
        if self.is_app_scope() {
            "🌐 *APP SCOPE* — Tất cả tài khoản".to_string()
        } else {
            let acc_name = self.account_name.as_deref().unwrap_or("Account");
            let acc_id = self.account_id.unwrap_or(0);
            format!("💼 *{}* \\[\\#{}\\]", escape_md(acc_name), acc_id)
        }
    }

    pub fn set_app_scope(&mut self) {
        self.account_id = None;
        self.account_name = None;
        self.current_view = CurrentView::Menu;
    }

    pub fn set_account_scope(&mut self, acc_id: i64, acc_name: String) {
        self.account_id = Some(acc_id);
        self.account_name = Some(acc_name);
        self.current_view = CurrentView::Menu;
    }
}

fn escape_md(s: &str) -> String {
    s.chars().flat_map(|c| {
        if "_*[]()~`>#+-=|{}.!".contains(c) { vec!['\\', c] } else { vec![c] }
    }).collect()
}
