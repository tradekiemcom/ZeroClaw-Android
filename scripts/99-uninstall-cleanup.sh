#!/bin/bash
# ==============================================================================
# IZ-TRADE NUCLEAR CLEANUP & UNINSTALLER
# Hỗ trợ: macOS (M1/Intel), Linux, Android (Termux)
# ==============================================================================

set -e

# Phát hiện OS
OS_TYPE=$(uname)
IS_TERMUX=false
if [ -n "$PREFIX" ]; then IS_TERMUX=true; fi

echo -e "\033[36m[Dọn Dẹp] Bắt đầu quá trình gỡ bỏ toàn bộ dấu vết iZ-Trade...\033[0m"

# 1. Dừng các tiến trình đang chạy
echo "  - Đang tìm và dừng các tiến trình zeroclaw..."
if [ "$OS_TYPE" == "Darwin" ] || [ "$OS_TYPE" == "Linux" ]; then
    killall zeroclaw 2>/dev/null || true
    sleep 1
fi

# 2. Xóa Binary khỏi hệ thống
echo "  - Xóa binary khỏi PATH..."
if $IS_TERMUX; then
    rm -f "$PREFIX/bin/zeroclaw"
else
    # Thử xóa trong các đường dẫn phổ biến
    sudo rm -f /usr/local/bin/zeroclaw 2>/dev/null || rm -f /usr/local/bin/zeroclaw 2>/dev/null || true
fi

# 3. Gỡ bỏ LaunchAgents / Services (macOS)
if [ "$OS_TYPE" == "Darwin" ]; then
    echo "  - Gỡ bỏ LaunchAgents..."
    LAUNCH_AGENT_DIR="$HOME/Library/LaunchAgents"
    if [ -f "$LAUNCH_AGENT_DIR/com.iztrade.hub.plist" ]; then
        launchctl unload "$LAUNCH_AGENT_DIR/com.iztrade.hub.plist" 2>/dev/null || true
        rm -f "$LAUNCH_AGENT_DIR/com.iztrade.hub.plist"
    fi
fi

# 4. Xóa Logs và Dữ liệu tạm
echo "  - Xóa Logs và dữ liệu tạm..."
rm -rf ~/.zeroclaw/log/* 2>/dev/null || true
rm -f iztrade.log 2>/dev/null || true

# 5. Xóa Cơ sở dữ liệu & Cấu hình (Hỏi trước hoặc dùng --force)
CLEAN_DB=false
if [[ "$1" == "--force" ]]; then
    CLEAN_DB=true
fi

if [ "$CLEAN_DB" = true ]; then
    echo "  - [NUCLEAR] Xóa cơ sở dữ liệu và file môi trường .env..."
    rm -f iztrade.db 2>/dev/null || true
    rm -f .env 2>/dev/null || true
    echo "  - Đã xóa sạch toàn bộ lịch sử giao dịch và mã khóa."
else
    echo "  - [Bỏ qua] Giữ lại iztrade.db và .env (Sử dụng --force để xóa trắng)."
fi

# 6. Dọn dẹp Cargo Target (nếu đang ở thư mục dev)
if [ -d "target" ]; then
    echo "  - Dọn dẹp thư mục build (target)..."
    cargo clean 2>/dev/null || true
fi

echo -e "\033[1;32m✅ Hệ thống đã được dọn dẹp sạch sẽ.\033[0m"
