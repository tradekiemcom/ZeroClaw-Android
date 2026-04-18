use iztrade::state::AppState;
use iztrade::config::Config;
use iztrade::ctrader::session::AccountSession;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_live_account_sync() {
    dotenv::dotenv().ok();
    let config = Config::from_env().unwrap();
    if config.is_mock() {
        println!("Skipping live test in MOCK mode");
        return;
    }

    let db = iztrade::storage::init_pool(&config.database_url).await.unwrap();
    let state = AppState::new(config, db).await;
    
    // Đảm bảo có tài khoản trong DB (đã add trước đó)
    let accounts = state.accounts.read().await;
    let target_id = 46832911;
    
    if !accounts.contains_key(&target_id) {
        panic!("Account {} not found in state for testing", target_id);
    }

    println!("🚀 Starting sync test for account {}", target_id);
    
    // Tạo session và chạy thủ công một lần sync
    let session = AccountSession::new(target_id, false, state.clone());
    
    // Đợi 15 giây để vòng lặp sync đầu tiên hoàn tất
    sleep(Duration::from_secs(15)).await;

    // Kiểm tra Balance
    let accounts_after = state.accounts.read().await;
    let acc = accounts_after.get(&target_id).unwrap();
    println!("💰 Sync Result - Balance: {}", acc.balance);
    assert!(acc.balance > 0.0, "Balance should be updated from API");

    // Kiểm tra Positions
    let positions = state.positions.read().await;
    println!("📈 Sync Result - Found {} positions", positions.len());
    
    for pos in positions.iter() {
        if pos.account_id == target_id {
            println!("   🔹 Position: {} {} lot {} @ {}", pos.symbol, pos.side, pos.volume, pos.open_price);
        }
    }
}
