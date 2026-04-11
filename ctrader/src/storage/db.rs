use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use crate::models::{Account, Bot, Position, position::PositionStatus};
use chrono::Utc;

pub async fn init_pool(database_url: &str) -> Result<SqlitePool> {
    // Tạo file DB nếu chưa có (sqlite://)
    let url = database_url.trim_start_matches("sqlite://");
    let _ = std::fs::OpenOptions::new().create(true).write(true).open(url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    migrate(&pool).await?;
    Ok(pool)
}

/// Tạo tables nếu chưa có
async fn migrate(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            broker_account_id INTEGER NOT NULL UNIQUE,
            access_token TEXT,
            connected INTEGER NOT NULL DEFAULT 0,
            autotrade INTEGER NOT NULL DEFAULT 1,
            daily_target_profit REAL NOT NULL DEFAULT 0,
            daily_max_loss REAL NOT NULL DEFAULT 0,
            daily_pnl REAL NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS bots (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            symbol TEXT NOT NULL DEFAULT 'XAUUSD',
            timeframe TEXT NOT NULL DEFAULT 'M15',
            daily_target_profit REAL NOT NULL DEFAULT 0,
            daily_max_loss REAL NOT NULL DEFAULT 0,
            daily_pnl REAL NOT NULL DEFAULT 0,
            trade_count_today INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS positions (
            id TEXT PRIMARY KEY,
            order_id TEXT NOT NULL,
            account_id INTEGER NOT NULL,
            bot_id TEXT NOT NULL,
            source TEXT NOT NULL,
            symbol TEXT NOT NULL,
            side TEXT NOT NULL,
            volume REAL NOT NULL,
            open_price REAL NOT NULL DEFAULT 0,
            sl REAL,
            tp REAL,
            pnl REAL NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'open',
            opened_at TEXT NOT NULL,
            closed_at TEXT
        );

        CREATE TABLE IF NOT EXISTS requests (
            id TEXT PRIMARY KEY,
            source TEXT NOT NULL,
            bot_id TEXT NOT NULL,
            action TEXT NOT NULL,
            payload TEXT NOT NULL,
            result TEXT,
            created_at TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ── Account CRUD ──────────────────────────────────────────────────────────────

pub async fn get_accounts(pool: &SqlitePool) -> Result<Vec<Account>> {
    let rows = sqlx::query!(
        "SELECT id, name, broker_account_id, access_token, connected, autotrade,
         daily_target_profit, daily_max_loss, daily_pnl, created_at FROM accounts"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Account {
        id: r.id,
        name: r.name,
        broker_account_id: r.broker_account_id,
        access_token: r.access_token,
        connected: r.connected != 0,
        autotrade: r.autotrade != 0,
        daily_target_profit: r.daily_target_profit,
        daily_max_loss: r.daily_max_loss,
        daily_pnl: r.daily_pnl,
        created_at: r.created_at.parse().unwrap_or_else(|_| Utc::now()),
    }).collect())
}

pub async fn upsert_account(pool: &SqlitePool, acc: &Account) -> Result<()> {
    sqlx::query!(
        r#"INSERT INTO accounts (id, name, broker_account_id, access_token, connected,
           autotrade, daily_target_profit, daily_max_loss, daily_pnl, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           ON CONFLICT(id) DO UPDATE SET
           name=excluded.name, access_token=excluded.access_token,
           connected=excluded.connected, autotrade=excluded.autotrade,
           daily_target_profit=excluded.daily_target_profit,
           daily_max_loss=excluded.daily_max_loss, daily_pnl=excluded.daily_pnl"#,
        acc.id,
        acc.name,
        acc.broker_account_id,
        acc.access_token,
        acc.connected as i64,
        acc.autotrade as i64,
        acc.daily_target_profit,
        acc.daily_max_loss,
        acc.daily_pnl,
        acc.created_at.to_rfc3339(),
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_account_autotrade(pool: &SqlitePool, id: i64, autotrade: bool) -> Result<()> {
    sqlx::query!("UPDATE accounts SET autotrade=? WHERE id=?", autotrade as i64, id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_account_pnl(pool: &SqlitePool, id: i64, pnl: f64) -> Result<()> {
    sqlx::query!("UPDATE accounts SET daily_pnl=? WHERE id=?", pnl, id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn reset_daily_pnl(pool: &SqlitePool) -> Result<()> {
    sqlx::query!("UPDATE accounts SET daily_pnl=0; UPDATE bots SET daily_pnl=0, trade_count_today=0")
        .execute(pool)
        .await?;
    Ok(())
}

// ── Bot CRUD ─────────────────────────────────────────────────────────────────

pub async fn get_bots(pool: &SqlitePool) -> Result<Vec<Bot>> {
    let rows = sqlx::query!(
        "SELECT id, name, enabled, symbol, timeframe, daily_target_profit,
         daily_max_loss, daily_pnl, trade_count_today, created_at FROM bots"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Bot {
        id: r.id,
        name: r.name,
        enabled: r.enabled != 0,
        symbol: r.symbol,
        timeframe: r.timeframe,
        daily_target_profit: r.daily_target_profit,
        daily_max_loss: r.daily_max_loss,
        daily_pnl: r.daily_pnl,
        trade_count_today: r.trade_count_today as i32,
        created_at: r.created_at.parse().unwrap_or_else(|_| Utc::now()),
    }).collect())
}

pub async fn upsert_bot(pool: &SqlitePool, bot: &Bot) -> Result<()> {
    sqlx::query!(
        r#"INSERT INTO bots (id, name, enabled, symbol, timeframe,
           daily_target_profit, daily_max_loss, daily_pnl, trade_count_today, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           ON CONFLICT(id) DO UPDATE SET
           name=excluded.name, enabled=excluded.enabled,
           symbol=excluded.symbol, timeframe=excluded.timeframe,
           daily_target_profit=excluded.daily_target_profit,
           daily_max_loss=excluded.daily_max_loss,
           daily_pnl=excluded.daily_pnl,
           trade_count_today=excluded.trade_count_today"#,
        bot.id, bot.name, bot.enabled as i64, bot.symbol, bot.timeframe,
        bot.daily_target_profit, bot.daily_max_loss, bot.daily_pnl,
        bot.trade_count_today, bot.created_at.to_rfc3339(),
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_bot_enabled(pool: &SqlitePool, bot_id: &str, enabled: bool) -> Result<()> {
    sqlx::query!("UPDATE bots SET enabled=? WHERE id=?", enabled as i64, bot_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_bot(pool: &SqlitePool, bot_id: &str) -> Result<Option<Bot>> {
    let row = sqlx::query!(
        "SELECT id, name, enabled, symbol, timeframe, daily_target_profit,
         daily_max_loss, daily_pnl, trade_count_today, created_at FROM bots WHERE id=?",
        bot_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Bot {
        id: r.id,
        name: r.name,
        enabled: r.enabled != 0,
        symbol: r.symbol,
        timeframe: r.timeframe,
        daily_target_profit: r.daily_target_profit,
        daily_max_loss: r.daily_max_loss,
        daily_pnl: r.daily_pnl,
        trade_count_today: r.trade_count_today as i32,
        created_at: r.created_at.parse().unwrap_or_else(|_| Utc::now()),
    }))
}

// ── Position CRUD ─────────────────────────────────────────────────────────────

pub async fn save_position(pool: &SqlitePool, pos: &Position) -> Result<()> {
    sqlx::query!(
        r#"INSERT INTO positions
           (id, order_id, account_id, bot_id, source, symbol, side, volume,
            open_price, sl, tp, pnl, status, opened_at, closed_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        pos.id, pos.order_id, pos.account_id, pos.bot_id, pos.source,
        pos.symbol, pos.side, pos.volume, pos.open_price,
        pos.sl, pos.tp, pos.pnl,
        pos.status.to_string(),
        pos.opened_at.to_rfc3339(),
        pos.closed_at.map(|d| d.to_rfc3339()),
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_open_positions(pool: &SqlitePool) -> Result<Vec<Position>> {
    let rows = sqlx::query!(
        "SELECT id, order_id, account_id, bot_id, source, symbol, side, volume,
         open_price, sl, tp, pnl, status, opened_at, closed_at
         FROM positions WHERE status='open'"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Position {
        id: r.id,
        order_id: r.order_id,
        account_id: r.account_id,
        bot_id: r.bot_id,
        source: r.source,
        symbol: r.symbol,
        side: r.side,
        volume: r.volume,
        open_price: r.open_price,
        sl: r.sl,
        tp: r.tp,
        pnl: r.pnl,
        status: PositionStatus::Open,
        opened_at: r.opened_at.parse().unwrap_or_else(|_| Utc::now()),
        closed_at: r.closed_at.and_then(|s| s.parse().ok()),
    }).collect())
}

pub async fn close_positions_by_bot(pool: &SqlitePool, bot_id: &str) -> Result<i64> {
    let result = sqlx::query!(
        "UPDATE positions SET status='closed', closed_at=? WHERE bot_id=? AND status='open'",
        Utc::now().to_rfc3339(),
        bot_id,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() as i64)
}

pub async fn close_all_positions(pool: &SqlitePool) -> Result<i64> {
    let result = sqlx::query!(
        "UPDATE positions SET status='closed', closed_at=? WHERE status='open'",
        Utc::now().to_rfc3339(),
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() as i64)
}
