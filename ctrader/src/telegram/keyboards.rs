use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup,
    KeyboardButton, KeyboardMarkup, ReplyMarkup,
};
use crate::models::{Account, Bot};

// ── Reply Keyboards (persistent bottom keyboard) ──────────────────────────────

/// Bàn phím tầng 1 – App Scope (toàn bộ tài khoản)
pub fn app_reply_keyboard(scope_badge: &str) -> KeyboardMarkup {
    // Xóa '*' và '\' markdown để text trên button đẹp hơn
    let clean_badge = scope_badge.replace('*', "").replace('\\', "");
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new(clean_badge),
        ],
        vec![
            KeyboardButton::new("📊 Status"),
            KeyboardButton::new("💼 Accounts"),
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
            KeyboardButton::new("🆕 Add Bot"),
            KeyboardButton::new("➕ Add Account"),
            KeyboardButton::new("⚙️ Settings"),
            KeyboardButton::new("❓ Help"),
        ],
    ])
    .resize_keyboard(true)
}

/// Bàn phím tầng 2 – Account Scope (1 tài khoản cụ thể)
pub fn account_reply_keyboard(scope_badge: &str) -> KeyboardMarkup {
    let clean_badge = scope_badge.replace('*', "").replace('\\', "");
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new(clean_badge),
        ],
        vec![
            KeyboardButton::new("ℹ️ Info"),
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
            KeyboardButton::new("🔴 Close All"),
            KeyboardButton::new("💰 Close Profit"),
            KeyboardButton::new("📉 Close Loss"),
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
            InlineKeyboardButton::callback("➕ Add Account", "app:add_acc"),
        ]);
    }

    InlineKeyboardMarkup::new(rows)
}

// ── Cấu trúc Mới (Tier-1 & Tier-2) ─────────────────────────────────────────────

// ── Pending Orders ──
pub fn pending_inline_keyboard(acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let pfx = if acc_id.is_some() { "acc:pnd" } else { "app:pnd" };
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("❌ Cancel Buy Limit", format!("{}:act:cbl", pfx)),
            InlineKeyboardButton::callback("❌ Cancel Sell Limit", format!("{}:act:csl", pfx)),
        ],
        vec![
            InlineKeyboardButton::callback("❌ Cancel Buy Stop", format!("{}:act:cbs", pfx)),
            InlineKeyboardButton::callback("❌ Cancel Sell Stop", format!("{}:act:css", pfx)),
        ],
        vec![
            InlineKeyboardButton::callback("❌ Cancel Buy Stop Limit", format!("{}:act:cbsl", pfx)),
            InlineKeyboardButton::callback("❌ Cancel Sell Stop Limit", format!("{}:act:cssl", pfx)),
        ],
        vec![
            InlineKeyboardButton::callback("🔴 CANCEL ALL PENDING", format!("{}:act:ca", pfx)),
        ],
        vec![
            InlineKeyboardButton::callback("🔄 Refresh", format!("{}:ref", pfx)),
        ],
    ])
}

// ── Positions ──
pub fn positions_tier1_keyboard(symbols: &[String], acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];
    let pfx = if acc_id.is_some() { "acc:pos" } else { "app:pos" };

    rows.push(vec![InlineKeyboardButton::callback("🌐 ALL SYMBOLS", format!("{}:sym:ALL", pfx))]);

    for chunk in symbols.chunks(2) {
        let row: Vec<_> = chunk.iter().map(|sym| {
            InlineKeyboardButton::callback(sym.clone(), format!("{}:sym:{}", pfx, sym))
        }).collect();
        rows.push(row);
    }
    
    rows.push(vec![InlineKeyboardButton::callback("🔄 Refresh", format!("{}:ref", pfx))]);
    InlineKeyboardMarkup::new(rows)
}

pub fn positions_tier2_keyboard(symbol: &str, acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let pfx = if acc_id.is_some() { "acc:pos" } else { "app:pos" };
    let sym_arg = if symbol == "ALL" { "ALL" } else { symbol };

    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("💰 Close Profit", format!("{}:act:cp:{}", pfx, sym_arg)),
            InlineKeyboardButton::callback("📉 Close Loss", format!("{}:act:cl:{}", pfx, sym_arg)),
        ],
        vec![
            InlineKeyboardButton::callback("📈 Close Buy", format!("{}:act:cb:{}", pfx, sym_arg)),
            InlineKeyboardButton::callback("📉 Close Sell", format!("{}:act:cs:{}", pfx, sym_arg)),
        ],
        vec![
            InlineKeyboardButton::callback("🔴 CLOSE ALL", format!("{}:act:ca:{}", pfx, sym_arg)),
        ],
        vec![
            InlineKeyboardButton::callback("⬅️ Back", format!("{}:lst", pfx)),
        ]
    ])
}

// ── Bots ──
pub fn bots_tier1_keyboard(bots: &[Bot], acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];
    let pfx = if acc_id.is_some() { "acc:bot" } else { "app:bot" };

    rows.push(vec![InlineKeyboardButton::callback("🤖 MANAGE ALL BOTS", format!("{}:sel:ALL", pfx))]);

    for chunk in bots.chunks(2) {
        let row: Vec<_> = chunk.iter().map(|bot| {
            let st = if bot.enabled { "✅" } else { "🔴" };
            let short_id = &bot.id[..bot.id.len().min(12)];
            let label = format!("{} {}", st, truncate(&bot.symbol, 10)); // VD: ✅ XAUUSD
            InlineKeyboardButton::callback(label, format!("{}:sel:{}", pfx, short_id))
        }).collect();
        rows.push(row);
    }
    
    rows.push(vec![InlineKeyboardButton::callback("🔄 Refresh", format!("{}:ref", pfx))]);
    InlineKeyboardMarkup::new(rows)
}

pub fn bots_tier2_keyboard(bot_id: &str, enabled: Option<bool>, acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let pfx = if acc_id.is_some() { "acc:bot" } else { "app:bot" };
    let short_id = &bot_id[..bot_id.len().min(12)];
    
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];
    
    if bot_id != "ALL" {
        if let Some(en) = enabled {
            let label = if en { "🔴 Turn OFF" } else { "🟢 Turn ON" };
            let act = if en { "off" } else { "on" };
            rows.push(vec![InlineKeyboardButton::callback(label.to_string(), format!("{}:act:{}:{}", pfx, act, short_id))]);
        }
    } else {
        rows.push(vec![
            InlineKeyboardButton::callback("🟢 Turn ON All", format!("{}:act:onall:ALL", pfx)),
            InlineKeyboardButton::callback("🔴 Turn OFF All", format!("{}:act:offall:ALL", pfx)),
        ]);
    }

    rows.push(vec![
        InlineKeyboardButton::callback("💰 Close Profit", format!("{}:act:cp:{}", pfx, short_id)),
        InlineKeyboardButton::callback("📉 Close Loss", format!("{}:act:cl:{}", pfx, short_id)),
    ]);
    rows.push(vec![
        InlineKeyboardButton::callback("📈 Close Buy", format!("{}:act:cb:{}", pfx, short_id)),
        InlineKeyboardButton::callback("📉 Close Sell", format!("{}:act:cs:{}", pfx, short_id)),
    ]);
    rows.push(vec![
        InlineKeyboardButton::callback("🔴 CLOSE ALL ORDERS", format!("{}:act:ca:{}", pfx, short_id)),
    ]);
    rows.push(vec![
        InlineKeyboardButton::callback("⬅️ Back", format!("{}:lst", pfx)),
    ]);

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
        InlineKeyboardButton::callback("📊 All bots", all_cb),
        InlineKeyboardButton::callback("📅 Today", "rpt:today"),
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
        InlineKeyboardButton::callback("❌ Cancel", "cancel"),
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
            InlineKeyboardButton::callback("📈 View Positions", format!("acc:pos:{}", acc_id)),
            InlineKeyboardButton::callback("⏳ View Pending", format!("acc:pnd:{}", acc_id)),
        ],
        vec![
            InlineKeyboardButton::callback("🔄 Refresh", format!("acc:inf:{}", acc_id)),
            InlineKeyboardButton::callback("⬅️ Account List", "app:list"),
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
