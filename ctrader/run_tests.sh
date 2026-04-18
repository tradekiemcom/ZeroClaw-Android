#!/bin/bash
set -e

echo "🧪 Starting Automated Testing Suite for iZTrade..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 1. Build & Unit/Integration Tests
echo "📦 Running Rust integration tests..."
cargo test --test parser_tests

# 2. CLI Execution Tests (Mock Mode)
echo "🖥️ Testing CLI Direct Commands (Mock)..."
export DATABASE_URL="sqlite://test.db"
export LOG_LEVEL="info"

echo "   [+] Adding sample account..."
sqlite3 test.db "INSERT OR REPLACE INTO accounts (id, name, broker_account_id, account_type, connected, autotrade, balance, equity, created_at) VALUES (101, 'Test Account', 101, 'demo', 1, 1, 10000.0, 10000.0, '2026-04-13T09:00:00Z');"

echo "   [+] Executing #BUY order..."
./target/debug/iztrade --exec "#BUY XAUUSD 0.1" --id 101 > test_exec.log 2>&1

if grep -iq "SUCCESS" test_exec.log || grep -iq "Opened" test_exec.log || grep -q "Xử lý lệnh" test_exec.log; then
    echo "   ✅ Execution success."
else
    echo "   ❌ Execution failed. See test_exec.log"
    cat test_exec.log
    exit 1
fi

echo "   [+] Verifying P&L Reporting..."
./target/debug/iztrade -r > test_report_raw.log
if grep -q "Acc 101" test_report_raw.log; then
    echo "   ✅ Report command works."
else
    echo "   ❌ Report parsing error."
    exit 1
fi

echo "   [+] Testing Close All Command..."
./target/debug/iztrade -c --id 101 > test_close.log
if grep -iq "SUCCESS" test_close.log || grep -q "Đã đóng" test_close.log; then
    echo "   ✅ Close All command works."
else
    echo "   ❌ Close All failed. See test_close.log"
    cat test_close.log
    exit 1
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 ALL TESTS PASSED SUCCESSFULLY!"
