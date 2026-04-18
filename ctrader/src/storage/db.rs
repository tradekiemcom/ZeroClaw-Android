use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use crate::models::{Account, AccountType, Bot, Position, PositionStatus, ApiClient};
use chrono::Utc;

pub async fn init_pool(database_url: &str) -> Result<SqlitePool> {
    let url = database_url.trim_start_matches("sqlite://");
    let _ = std::fs::OpenOptions::new().create(true).write(true).open(url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    migrate(&pool).await?;
    Ok(pool)
}

async fn migrate(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            broker_account_id INTEGER NOT NULL UNIQUE,
            account_type TEXT NOT NULL DEFAULT 'demo',
            access_token TEXT,
            connected INTEGER NOT NULL DEFAULT 0,
            autotrade INTEGER NOT NULL DEFAULT 1,
            balance REAL NOT NULL DEFAULT 0,
            equity REAL NOT NULL DEFAULT 0,
            float_profit REAL NOT NULL DEFAULT 0,
            daily_pnl REAL NOT NULL DEFAULT 0,
            daily_target_profit REAL NOT NULL DEFAULT 0,
            daily_max_loss REAL NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS bots (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            account_id INTEGER NOT NULL DEFAULT 0,
            enabled INTEGER NOT NULL DEFAULT 1,
            symbol TEXT NOT NULL DEFAULT 'XAUUSD',
            timeframe TEXT NOT NULL DEFAULT 'M15',
            daily_target_profit REAL NOT NULL DEFAULT 0,
            daily_max_loss REAL NOT NULL DEFAULT 0,
            daily_pnl REAL NOT NULL DEFAULT 0,
            trade_count_today INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS api_clients (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            api_key TEXT NOT NULL UNIQUE,
            source TEXT NOT NULL DEFAULT 'API',
            enabled INTEGER NOT NULL DEFAULT 1,
            description TEXT,
            allowed_actions TEXT NOT NULL DEFAULT '[]',
            request_count INTEGER NOT NULL DEFAULT 0,
            last_used_at TEXT,
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
            api_client_id TEXT,
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
        "SELECT id, name, broker_account_id, account_type, access_token, connected,
         autotrade, balance, equity, float_profit, daily_pnl,
         daily_target_profit, daily_max_loss, created_at FROM accounts"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Account {
        id: r.id,
        name: r.name,
        broker_account_id: r.broker_account_id,
        account_type: if r.account_type == "real" { AccountType::Real } else { AccountType::Demo },
        access_token: r.access_token,
        connected: r.connected != 0,
        autotrade: r.autotrade != 0,
        balance: r.balance,
        equity: r.equity,
        float_profit: r.float_profit,
        daily_pnl: r.daily_pnl,
        daily_target_profit: r.daily_target_profit,
        daily_max_loss: r.daily_max_loss,
        created_at: r.created_at.parse().unwrap_or_else(|_| Utc::now()),
    }).collect())
}

pub async fn upsert_account(pool: &SqlitePool, acc: &Account) -> Result<()> {
    let acc_type = if acc.account_type == AccountType::Real { "real" } else { "demo" };
    let created_at = acc.created_at.to_rfc3339();
    let acc_id = acc.id;
    let b_acc_id = acc.broker_account_id;
    let con = acc.connected as i64;
    let auto = acc.autotrade as i64;

    sqlx::query!(
        r#"INSERT INTO accounts (id, name, broker_account_id, account_type, access_token,
           connected, autotrade, balance, equity, float_profit, daily_pnl,
           daily_target_profit, daily_max_loss, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           ON CONFLICT(id) DO UPDATE SET
           name=excluded.name, account_type=excluded.account_type,
           access_token=excluded.access_token, connected=excluded.connected,
           autotrade=excluded.autotrade, balance=excluded.balance,
           equity=excluded.equity, float_profit=excluded.float_profit,
           daily_pnl=excluded.daily_pnl,
           daily_target_profit=excluded.daily_target_profit,
           daily_max_loss=excluded.daily_max_loss"#,
        acc_id, acc.name, b_acc_id, acc_type,
        acc.access_token, con, auto,
        acc.balance, acc.equity, acc.float_profit, acc.daily_pnl,
        acc.daily_target_profit, acc.daily_max_loss,
        created_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_account(pool: &SqlitePool, id: i64) -> Result<()> {
    sqlx::query!("DELETE FROM accounts WHERE id=?", id)
        .execute(pool).await?;
    Ok(())
}

pub async fn update_account_autotrade(pool: &SqlitePool, id: i64, autotrade: bool) -> Result<()> {
    let auto_val = autotrade as i64;
    sqlx::query!("UPDATE accounts SET autotrade=? WHERE id=?", auto_val, id)
        .execute(pool).await?;
    Ok(())
}

pub async fn update_account_pnl(pool: &SqlitePool, id: i64, pnl: f64) -> Result<()> {
    sqlx::query!("UPDATE accounts SET daily_pnl=? WHERE id=?", pnl, id)
        .execute(pool).await?;
    Ok(())
}

pub async fn update_account_equity(pool: &SqlitePool, id: i64, equity: f64, float_profit: f64) -> Result<()> {
    sqlx::query!("UPDATE accounts SET equity=?, float_profit=? WHERE id=?", equity, float_profit, id)
        .execute(pool).await?;
    Ok(())
}

pub async fn reset_daily_pnl(pool: &SqlitePool) -> Result<()> {
    sqlx::query!("UPDATE accounts SET daily_pnl=0")
        .execute(pool).await?;
    sqlx::query!("UPDATE bots SET daily_pnl=0, trade_count_today=0")
        .execute(pool).await?;
    Ok(())
}

// ── ApiClient CRUD ────────────────────────────────────────────────────────────

pub async fn get_api_clients(pool: &SqlitePool) -> Result<Vec<ApiClient>> {
    let rows = sqlx::query!(
        "SELECT id, name, api_key, source, enabled, description,
         allowed_actions, request_count, last_used_at, created_at FROM api_clients"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ApiClient {
        id: r.id.unwrap_or_default(),
        name: r.name,
        api_key: r.api_key,
        source: r.source,
        enabled: r.enabled != 0,
        description: r.description,
        allowed_actions: serde_json::from_str(&r.allowed_actions).unwrap_or_default(),
        request_count: r.request_count,
        last_used_at: r.last_used_at.and_then(|s| s.parse().ok()),
        created_at: r.created_at.parse().unwrap_or_else(|_| Utc::now()),
    }).collect())
}

pub async fn insert_api_client(pool: &SqlitePool, client: &ApiClient) -> Result<()> {
    let allowed = serde_json::to_string(&client.allowed_actions)?;
    let created_at = client.created_at.to_rfc3339();
    let enabled_int = client.enabled as i64;
    let c_id = client.id.clone();
    let c_name = client.name.clone();
    let c_key = client.api_key.clone();
    let c_src = client.source.clone();

    sqlx::query!(
        r#"INSERT INTO api_clients
           (id, name, api_key, source, enabled, description, allowed_actions, request_count, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?)"#,
        c_id, c_name, c_key, c_src,
        enabled_int, client.description, allowed,
        created_at,
    )
    .execute(pool).await?;
    Ok(())
}

pub async fn delete_api_client(pool: &SqlitePool, client_id: &str) -> Result<bool> {
    let result = sqlx::query!("DELETE FROM api_clients WHERE id=?", client_id)
        .execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn set_api_client_enabled(pool: &SqlitePool, client_id: &str, enabled: bool) -> Result<bool> {
    let c_id = client_id.to_string();
    let enabled_int = enabled as i64;
    let result = sqlx::query!("UPDATE api_clients SET enabled=? WHERE id=?", enabled_int, c_id)
        .execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn find_api_client_by_key(pool: &SqlitePool, api_key: &str) -> Result<Option<ApiClient>> {
    let row = sqlx::query!(
        "SELECT id, name, api_key, source, enabled, description,
         allowed_actions, request_count, last_used_at, created_at
         FROM api_clients WHERE api_key=? AND enabled=1",
        api_key
    )
    .fetch_optional(pool).await?;

    Ok(row.map(|r| ApiClient {
        id: r.id.unwrap_or_default(),
        name: r.name,
        api_key: r.api_key,
        source: r.source,
        enabled: r.enabled != 0,
        description: r.description,
        allowed_actions: serde_json::from_str(&r.allowed_actions).unwrap_or_default(),
        request_count: r.request_count,
        last_used_at: r.last_used_at.and_then(|s| s.parse().ok()),
        created_at: r.created_at.parse().unwrap_or_else(|_| Utc::now()),
    }))
}

pub async fn increment_client_usage(pool: &SqlitePool, client_id: &str) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    sqlx::query!(
        "UPDATE api_clients SET request_count=request_count+1, last_used_at=? WHERE id=?",
        now, client_id
    )
    .execute(pool).await?;
    Ok(())
}

// ── Bot CRUD ─────────────────────────────────────────────────────────────────

pub async fn get_bots(pool: &SqlitePool) -> Result<Vec<Bot>> {
    let rows = sqlx::query(
        "SELECT id, name, account_id, enabled, symbol, timeframe, daily_target_profit,
         daily_max_loss, daily_pnl, trade_count_today, created_at FROM bots"
    )
    .fetch_all(pool).await?;

    use sqlx::Row;
    Ok(rows.into_iter().map(|r| Bot {
        id: r.get::<Option<String>, _>("id").unwrap_or_default(), 
        name: r.get("name"), 
        account_id: r.get("account_id"),
        enabled: r.get::<i64, _>("enabled") != 0,
        symbol: r.get("symbol"), 
        timeframe: r.get("timeframe"),
        daily_target_profit: r.get("daily_target_profit"),
        daily_max_loss: r.get("daily_max_loss"), 
        daily_pnl: r.get("daily_pnl"),
        trade_count_today: r.get::<i64, _>("trade_count_today") as i32,
        created_at: r.get::<String, _>("created_at").parse().unwrap_or_else(|_| Utc::now()),
    }).collect())
}

pub async fn upsert_bot(pool: &SqlitePool, bot: &Bot) -> Result<()> {
    let created_at = bot.created_at.to_rfc3339();
    let con = bot.enabled as i64;
    let b_id = bot.id.clone();
    let b_name = bot.name.clone();
    let b_acc = bot.account_id;
    let b_sym = bot.symbol.clone();
    let b_tf = bot.timeframe.clone();

    sqlx::query(
        r#"INSERT INTO bots (id, name, account_id, enabled, symbol, timeframe,
           daily_target_profit, daily_max_loss, daily_pnl, trade_count_today, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           ON CONFLICT(id) DO UPDATE SET
           name=excluded.name, account_id=excluded.account_id, enabled=excluded.enabled,
           symbol=excluded.symbol, timeframe=excluded.timeframe,
           daily_target_profit=excluded.daily_target_profit,
           daily_max_loss=excluded.daily_max_loss, daily_pnl=excluded.daily_pnl,
           trade_count_today=excluded.trade_count_today"#
    )
    .bind(b_id).bind(b_name).bind(b_acc).bind(con).bind(b_sym).bind(b_tf)
    .bind(bot.daily_target_profit).bind(bot.daily_max_loss).bind(bot.daily_pnl)
    .bind(bot.trade_count_today as i64).bind(created_at)
    .execute(pool).await?;
    Ok(())
}

pub async fn set_bot_enabled(pool: &SqlitePool, bot_id: &str, enabled: bool) -> Result<bool> {
    let auto_val = enabled as i64;
    let result = sqlx::query!("UPDATE bots SET enabled=? WHERE id=?", auto_val, bot_id)
        .execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_bot(pool: &SqlitePool, bot_id: &str) -> Result<Option<Bot>> {
    let row = sqlx::query(
        "SELECT id, name, account_id, enabled, symbol, timeframe, daily_target_profit,
         daily_max_loss, daily_pnl, trade_count_today, created_at FROM bots WHERE id=?"
    )
    .bind(bot_id)
    .fetch_optional(pool).await?;

    use sqlx::Row;
    Ok(row.map(|r| Bot {
        id: r.get::<Option<String>, _>("id").unwrap_or_default(), 
        name: r.get("name"), 
        account_id: r.get("account_id"),
        enabled: r.get::<i64, _>("enabled") != 0,
        symbol: r.get("symbol"), 
        timeframe: r.get("timeframe"),
        daily_target_profit: r.get("daily_target_profit"),
        daily_max_loss: r.get("daily_max_loss"), 
        daily_pnl: r.get("daily_pnl"),
        trade_count_today: r.get::<i64, _>("trade_count_today") as i32,
        created_at: r.get::<String, _>("created_at").parse().unwrap_or_else(|_| Utc::now()),
    }))
}

// ── Position CRUD ─────────────────────────────────────────────────────────────

pub async fn save_position(pool: &SqlitePool, pos: &Position) -> Result<()> {
    let opened_at = pos.opened_at.to_rfc3339();
    let closed_at = pos.closed_at.map(|d| d.to_rfc3339());
    let p_id = pos.id.clone();
    let o_id = pos.order_id.clone();
    let b_id = pos.bot_id.clone();
    let src = pos.source.clone();
    let sym = pos.symbol.clone();
    let side = pos.side.clone();
    let status = pos.status.to_string();

    sqlx::query!(
        r#"INSERT INTO positions
           (id, order_id, account_id, bot_id, source, symbol, side, volume,
            open_price, sl, tp, pnl, status, opened_at, closed_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        p_id, o_id, pos.account_id, b_id, src,
        sym, side, pos.volume, pos.open_price,
        pos.sl, pos.tp, pos.pnl, status,
        opened_at,
        closed_at,
    )
    .execute(pool).await?;
    Ok(())
}

pub async fn get_open_positions(pool: &SqlitePool) -> Result<Vec<Position>> {
    let rows = sqlx::query!(
        "SELECT id, order_id, account_id, bot_id, source, symbol, side, volume,
         open_price, sl, tp, pnl, status, opened_at, closed_at
         FROM positions WHERE status='open'"
    )
    .fetch_all(pool).await?;

    Ok(rows.into_iter().map(|r| Position {
        id: r.id.unwrap_or_default(), 
        order_id: r.order_id, 
        account_id: r.account_id,
        bot_id: r.bot_id, 
        source: r.source, 
        symbol: r.symbol,
        side: r.side, 
        volume: r.volume, 
        open_price: r.open_price,
        sl: r.sl, tp: r.tp, pnl: r.pnl,
        status: PositionStatus::Open,
        opened_at: r.opened_at.parse().unwrap_or_else(|_| Utc::now()),
        closed_at: r.closed_at.and_then(|s| s.parse().ok()),
    }).collect())
}

pub async fn close_positions_by_bot(pool: &SqlitePool, bot_id: &str) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query!(
        "UPDATE positions SET status='closed', closed_at=? WHERE bot_id=? AND status='open'",
        now, bot_id,
    )
    .execute(pool).await?;
    Ok(result.rows_affected() as i64)
}

pub async fn close_positions_by_pnl(pool: &SqlitePool, bot_id: &str, is_profit: bool) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    let sql = if is_profit {
        "UPDATE positions SET status='closed', closed_at=? WHERE bot_id=? AND status='open' AND pnl > 0"
    } else {
        "UPDATE positions SET status='closed', closed_at=? WHERE bot_id=? AND status='open' AND pnl <= 0"
    };
    
    let result = sqlx::query(sql)
        .bind(now)
        .bind(bot_id)
        .execute(pool).await?;
        
    Ok(result.rows_affected() as i64)
}

pub async fn close_all_positions(pool: &SqlitePool) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query!(
        "UPDATE positions SET status='closed', closed_at=? WHERE status='open'",
        now,
    )
    .execute(pool).await?;
    Ok(result.rows_affected() as i64)
}
