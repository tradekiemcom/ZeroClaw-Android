use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// API Client - mỗi ứng dụng kết nối dùng 1 key riêng
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiClient {
    pub id: String,              // UUID
    pub name: String,            // Tên ứng dụng (vd: "MT5 EA", "TradingView Bot")
    pub api_key: String,         // Bearer key để xác thực
    pub source: String,          // Nguồn: TELEGRAM, MT5, TRADINGVIEW, ZEROCLAW...
    pub enabled: bool,           // Bật/tắt key
    pub description: Option<String>,
    pub allowed_actions: Vec<String>, // [] = cho phép tất cả
    pub request_count: i64,      // Số lần đã sử dụng
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl ApiClient {
    pub fn new(name: String, source: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            api_key: generate_api_key(),
            source,
            enabled: true,
            description: None,
            allowed_actions: vec![],
            request_count: 0,
            last_used_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn status_text(&self) -> &'static str {
        if self.enabled { "[ACTIVE]" } else { "[DISABLED]" }
    }

    pub fn format_list_item(&self) -> String {
        format!(
            "{} `{}` — *{}* ({})\n   🔑 `{}...`\n   📊 {} requests | {}",
            self.status_text(),
            self.id[..8].to_string(),
            self.name,
            self.source,
            &self.api_key[..16],
            self.request_count,
            self.last_used_at
                .map(|t| t.format("%m/%d %H:%M").to_string())
                .unwrap_or_else(|| "Chưa dùng".to_string()),
        )
    }
}

/// Tạo API key ngẫu nhiên dạng `izt_xxxxxxxxxxxxxxxxxxxx`
fn generate_api_key() -> String {
    use std::time::SystemTime;
    let _ts = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    format!("izt_{}", Uuid::new_v4().to_string().replace('-', "")[..24].to_string())
}
