use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup,
    KeyboardButton, KeyboardMarkup,
};
use crate::models::{Account, Bot};

// ── Reply Keyboards (Persistent 3x3 Layout) ──────────────────────────────────

/// Bàn phím 3x3 cố định cho toàn bộ ứng dụng
pub fn main_reply_keyboard(acc_id: Option<i64>) -> KeyboardMarkup {
    let first_btn = match acc_id {
        Some(id) => format!("Acc #{} Info", id),
        None => "App Info".to_string(),
    };

    KeyboardMarkup::new(vec![
        // Hàng 1: Cố định (Navigation & Info)
        vec![
            KeyboardButton::new(first_btn),
            KeyboardButton::new("Top"),
            KeyboardButton::new("Back"),
        ],
        // Hàng 2: Menu ngữ cảnh cấp 1
        vec![
            KeyboardButton::new("Bots"),
            KeyboardButton::new("Positions"),
            KeyboardButton::new("Pending"),
        ],
        // Hàng 3: Hệ thống / Report
        vec![
            KeyboardButton::new("ON Autotrade"),
            KeyboardButton::new("Report"),
            KeyboardButton::new("OFF Autotrade"),
        ],
    ])
    .resize_keyboard(true)
}

// ── Inline Keyboards (Context-specific) ─────────────────────────────────────

/// Cấp 2: Danh sách accounts
pub fn accounts_list_keyboard(accounts: &[Account]) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];
    for chunk in accounts.chunks(2) {
        let row: Vec<_> = chunk.iter().map(|acc| {
            let label = format!("Account #{}", acc.id);
            InlineKeyboardButton::callback(label, format!("acc:sel:{}", acc.id))
        }).collect();
        rows.push(row);
    }
    InlineKeyboardMarkup::new(rows)
}

/// Cấp 2: Danh sách symbols (Positions)
pub fn positions_tier1_keyboard(symbols: &[String], acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];
    let pfx = if acc_id.is_some() { "acc:pos" } else { "app:pos" };

    rows.push(vec![InlineKeyboardButton::callback("All Symbols", format!("{}:sym:all", pfx))]);
    for chunk in symbols.chunks(2) {
        let row: Vec<_> = chunk.iter().map(|sym| {
            InlineKeyboardButton::callback(sym.clone(), format!("{}:sym:{}", pfx, sym))
        }).collect();
        rows.push(row);
    }
    rows.push(vec![InlineKeyboardButton::callback("Refresh", format!("{}:ref", pfx))]);
    InlineKeyboardMarkup::new(rows)
}

/// Cấp 3: Điều khiển Position (Close actions)
pub fn positions_actions_keyboard(symbol: &str, acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let pfx = if acc_id.is_some() { "acc:pos" } else { "app:pos" };
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Close Buy", format!("{}:act:cb:{}", pfx, symbol)),
            InlineKeyboardButton::callback("Close Sell", format!("{}:act:cs:{}", pfx, symbol)),
        ],
        vec![
            InlineKeyboardButton::callback("Close Profit", format!("{}:act:cp:{}", pfx, symbol)),
            InlineKeyboardButton::callback("Close Loss", format!("{}:act:cl:{}", pfx, symbol)),
        ],
        vec![
            InlineKeyboardButton::callback("Close All", format!("{}:act:ca:{}", pfx, symbol)),
        ],
    ])
}

/// Cấp 2: Danh sách Bots
pub fn bots_tier1_keyboard(bots: &[Bot], acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![];
    let pfx = if acc_id.is_some() { "acc:bot" } else { "app:bot" };

    rows.push(vec![InlineKeyboardButton::callback("Manage All Bots", format!("{}:sel:all", pfx))]);
    for chunk in bots.chunks(2) {
        let row: Vec<InlineKeyboardButton> = chunk.iter().map(|bot: &Bot| -> InlineKeyboardButton {
            InlineKeyboardButton::callback(bot.id.clone(), format!("{}:sel:{}", pfx, bot.id))
        }).collect();
        rows.push(row);
    }
    InlineKeyboardMarkup::new(rows)
}

/// Cấp 3: Điều khiển Bot (Enable/Disable/Config)
pub fn bots_tier2_keyboard(bot_id: &str, enabled: Option<bool>, acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let pfx = if acc_id.is_some() { "acc:bot" } else { "app:bot" };
    
    let mut rows = vec![];
    if let Some(en) = enabled {
        let toggle_label = if en { "[DISABLE]" } else { "[ENABLE]" };
        let toggle_act = if en { "off" } else { "on" };
        rows.push(vec![InlineKeyboardButton::callback(toggle_label, format!("{}:act:{}:{}", pfx, toggle_act, bot_id))]);
    }

    rows.push(vec![
        InlineKeyboardButton::callback("Config", format!("{}:cfg:{}", pfx, bot_id)),
        InlineKeyboardButton::callback("Report", format!("{}:rpt:{}", pfx, bot_id)),
    ]);
    
    InlineKeyboardMarkup::new(rows)
}

/// Cấp 2: Danh sách Pending
pub fn pending_inline_keyboard(acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let pfx = if acc_id.is_some() { "acc:pnd" } else { "app:pnd" };
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Delete All Pending", format!("{}:act:ca", pfx)),
        ],
        vec![
            InlineKeyboardButton::callback("Refresh", format!("{}:ref", pfx)),
        ],
    ])
}

/// Cấp 2: Report
pub fn report_inline_keyboard(acc_id: Option<i64>) -> InlineKeyboardMarkup {
    let pfx = match acc_id {
        Some(id) => format!("acc:rpt:{}", id),
        None => "app:rpt".to_string(),
    };
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Detail Report", format!("{}:det", pfx)),
            InlineKeyboardButton::callback("Summary", format!("{}:sum", pfx)),
        ],
    ])
}
