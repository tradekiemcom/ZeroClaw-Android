use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup,
    KeyboardButton, KeyboardMarkup, ReplyMarkup,
};
use crate::models::{Account, Bot};

// ── Reply Keyboards (persistent bottom keyboard) ──────────────────────────────

/// Bàn phím tầng 1 – App Scope (toàn bộ tài khoản)
pub fn app_reply_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("📊 Status"),
            KeyboardButton::new("💼 Tài khoản"),
            KeyboardButton::new("🤖 Bots"),
            KeyboardButton::new("🔑 API Keys"),
        ],
        vec![
            KeyboardButton::new("🟢 Auto ON"),
            KeyboardButton::new("🔴 Auto OFF"),
            KeyboardButton::new("📈 Positions"),
            KeyboardButton::new("📑 Report"),
        ],
        vec![
            KeyboardButton::new("🆕 Thêm Bot"),
            KeyboardButton::new("➕ Thêm Account"),
            KeyboardButton::new("⚙️ Cài đặt"),
            KeyboardButton::new("❓ Trợ giúp"),
        ],
    ])
    .resize_keyboard(true)
}

/// Bàn phím tầng 2 – Account Scope (1 tài khoản cụ thể)
pub fn account_reply_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("ℹ️ Thông tin"),
            KeyboardButton::new("📊 Report"),
            KeyboardButton::new("📈 Positions"),
            KeyboardButton::new("⏳ Pending"),
        ],
        vec![
            KeyboardButton::new("🤖 Bots"),
            KeyboardButton::new("🟢 Auto ON"),
            KeyboardButton::new("🔴 Auto OFF"),
            KeyboardButton::new("⬅️ Top Menu"),
        ],
        vec![
            KeyboardButton::new("🔴 Đóng Tất Cả"),
            KeyboardButton::new("💰 Đóng Lời"),
            KeyboardButton::new("📉 Đóng Lỗ"),
            KeyboardButton::new("🔄 Refresh"),
        ],
    ])
    .resize_keyboard(true)
}

// ── Inline Keyboards (context-specific, gắn vào message) ─────────────────────

/// Danh sách accounts để chọn vào quản lý
pub fn accounts_list_keyboard(accounts: &[Account]) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];

    for chunk in accounts.chunks(2) {
        let row: Vec<InlineKeyboardButton> = chunk.iter().map(|acc| {
            let atype = if acc.account_type == crate::models::AccountType::Real { "🔴" } else { "🔵" };
            let auto = if acc.autotrade { "🟢" } else { "⏸️" };
            let label = format!("{}{} #{} {}", atype, auto, acc.id, truncate(&acc.name, 12));
            InlineKeyboardButton::callback(label, format!("acc:sel:{}", acc.id))
        }).collect();
        rows.push(row);
    }

    if accounts.is_empty() {
        rows.push(vec![
            InlineKeyboardButton::callback("➕ Thêm tài khoản", "app:add_acc"),
        ]);
    }

    InlineKeyboardMarkup::new(rows)
}

/// Context keyboard cho Positions view (App scope — tất cả)
pub fn positions_app_inline_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("💰 Đóng Lời", "app:pos:cp"),
            InlineKeyboardButton::callback("📉 Đóng Lỗ", "app:pos:cl"),
        ],
        vec![
            InlineKeyboardButton::callback("📈 Đóng Buy", "app:pos:cb"),
            InlineKeyboardButton::callback("📉 Đóng Sell", "app:pos:cs"),
        ],
        vec![
            InlineKeyboardButton::callback("🔴 ĐÓNG TẤT CẢ + Tắt Auto", "app:pos:ca"),
        ],
        vec![
            InlineKeyboardButton::callback("🔄 Refresh", "app:pos:ref"),
        ],
    ])
}

/// Context keyboard cho Positions view (Account scope — 1 tài khoản)
pub fn positions_account_inline_keyboard(acc_id: i64) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("💰 Đóng Lời", format!("acc:pos:cp:{}", acc_id)),
            InlineKeyboardButton::callback("📉 Đóng Lỗ", format!("acc:pos:cl:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("📈 Đóng Buy", format!("acc:pos:cb:{}", acc_id)),
            InlineKeyboardButton::callback("📉 Đóng Sell", format!("acc:pos:cs:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("🔴 ĐÓNG TẤT CẢ", format!("acc:pos:ca:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("🟢 Auto ON", format!("acc:ao:{}", acc_id)),
            InlineKeyboardButton::callback("🔄 Refresh", format!("acc:pos:ref:{}", acc_id)),
        ],
    ])
}

/// Context keyboard cho Pending view (Account scope)
pub fn pending_account_inline_keyboard(acc_id: i64) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("💰 Hủy Lời", format!("pnd:cp:{}", acc_id)),
            InlineKeyboardButton::callback("📉 Hủy Lỗ", format!("pnd:cl:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("📈 Hủy Buy", format!("pnd:cb:{}", acc_id)),
            InlineKeyboardButton::callback("📉 Hủy Sell", format!("pnd:cs:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("🔴 HỦY TẤT CẢ Pending", format!("pnd:ca:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("🔄 Refresh", format!("pnd:ref:{}", acc_id)),
        ],
    ])
}

/// Context keyboard cho Bots view  
pub fn bots_inline_keyboard(bots: &[Bot], acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];

    // Toggle mỗi bot
    for bot in bots {
        let status = if bot.enabled { "✅" } else { "⏸️" };
        let pnl = format!("{:+.2}", bot.daily_pnl);
        let label = format!("{} {} | {}", status, truncate(&bot.id, 14), pnl);
        let action = if bot.enabled {
            match acc_id {
                Some(id) => format!("acc:bof:{}:{}", id, &bot.id[..bot.id.len().min(20)]),
                None => format!("app:bof:{}", &bot.id[..bot.id.len().min(20)]),
            }
        } else {
            match acc_id {
                Some(id) => format!("acc:bon:{}:{}", id, &bot.id[..bot.id.len().min(20)]),
                None => format!("app:bon:{}", &bot.id[..bot.id.len().min(20)]),
            }
        };
        rows.push(vec![InlineKeyboardButton::callback(label, action)]);
    }

    // Actions
    if let Some(id) = acc_id {
        rows.push(vec![
            InlineKeyboardButton::callback("✅ Bật tất cả", format!("acc:ball:{}", id)),
            InlineKeyboardButton::callback("⏸️ Tắt tất cả", format!("acc:boff:{}", id)),
        ]);
    } else {
        rows.push(vec![
            InlineKeyboardButton::callback("✅ Bật tất cả", "app:ball"),
            InlineKeyboardButton::callback("⏸️ Tắt tất cả", "app:boff"),
        ]);
    }

    InlineKeyboardMarkup::new(rows)
}

/// Context keyboard cho Report view
pub fn report_inline_keyboard(bots: &[Bot], acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];

    // Overview options
    let all_cb = match acc_id {
        Some(id) => format!("rpt:all:{}", id),
        None => "rpt:all:0".to_string(),
    };
    rows.push(vec![
        InlineKeyboardButton::callback("📊 Tất cả bots", all_cb),
        InlineKeyboardButton::callback("📅 Hôm nay", "rpt:today"),
    ]);

    // Per-bot report
    let bot_rows: Vec<_> = bots.chunks(2).map(|chunk| {
        chunk.iter().map(|bot| {
            let label = format!("🤖 {}", truncate(&bot.id, 16));
            let cb = match acc_id {
                Some(id) => format!("rpt:bot:{}:{}", id, &bot.id[..bot.id.len().min(18)]),
                None => format!("rpt:gbot:{}", &bot.id[..bot.id.len().min(18)]),
            };
            InlineKeyboardButton::callback(label, cb)
        }).collect::<Vec<_>>()
    }).collect();

    rows.extend(bot_rows);

    InlineKeyboardMarkup::new(rows)
}

/// Confirm keyboard cho các hành động nguy hiểm
pub fn confirm_inline_keyboard(action: &str, label: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback(format!("✅ {}", label), action.to_string()),
        InlineKeyboardButton::callback("❌ Hủy", "cancel"),
    ]])
}

/// Keyboard thông tin account với quick actions
pub fn account_info_inline_keyboard(acc_id: i64) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🟢 Auto ON", format!("acc:ao:{}", acc_id)),
            InlineKeyboardButton::callback("🔴 Auto OFF", format!("acc:ad:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("📈 Xem Positions", format!("acc:pos:{}", acc_id)),
            InlineKeyboardButton::callback("⏳ Xem Pending", format!("acc:pnd:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("🔄 Refresh", format!("acc:inf:{}", acc_id)),
            InlineKeyboardButton::callback("⬅️ Danh sách", "app:list"),
        ],
    ])
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max-1])
    }
}
