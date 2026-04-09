#!/bin/bash
# ==============================================================================
# ZERO-CLAW DEEP CLEAN UTILITY
# Dọn dẹp hệ thống, giải phóng bộ nhớ và xóa dấu vết cài đặt cũ
# ==============================================================================

export PATH="$PREFIX/bin:$PATH"

echo -e "\033[36m[Dọn Dẹp] Bắt đầu quá trình giải phóng dung lượng...\033[0m"

# 1. Dọn dẹp cache của trình quản lý gói
echo "  - Dọn dẹp bộ nhớ đệm pkg/apt..."
pkg clean -y > /dev/null 2>&1 || true

# 2. Giải phóng bộ nhớ đệm và file rác (NPM, Cargo, Rust, Cache)
echo "  - Dọn dẹp cache NPM, bộ công cụ biên dịch (~/.cargo, ~/.rustup, ~/.npm)..."
rm -rf ~/.cargo ~/.rustup ~/.npm ~/.cache > /dev/null 2>&1 || true

# 3. Xóa các thư mục tạm & Giải phóng Proot (Extreme Clean)
echo "  - Xóa triệt để thư mục /tmp, log cũ và bộ giả lập lỗi..."
rm -rf $PREFIX/tmp/* > /dev/null 2>&1 || true
rm -rf ~/.zeroclaw/log/* > /dev/null 2>&1 || true
rm -rf /data/data/com.termux/files/usr/tmp/zeroclaw-* > /dev/null 2>&1 || true
pkg uninstall proot -y > /dev/null 2>&1 || true
rm -f "$PREFIX/bin/zeroclaw.bin" > /dev/null 2>&1 || true

# 4. Kiểm soát và Dọn dẹp dịch vụ ngầm (Port Liberator)
echo "  - Đang tìm và giải phóng các cổng quan trọng (42617, 5555)..."
for PORT in 42617 5555; do
    PIDS=$(lsof -ti:$PORT 2>/dev/null)
    if [ -n "$PIDS" ]; then
        echo "  - Phát hiện tiến trình chiếm cổng $PORT. Đang dừng..."
        echo "$PIDS" | xargs kill -9 2>/dev/null || true
    fi
done

# 5. Gỡ bỏ các gói build nặng để lấy lại dung lượng
echo "  - Thu hồi dung lượng từ các gói build (rust, clang)..."
pkg uninstall rust clang make binutils -y > /dev/null 2>&1 || true
pkg autoremove -y > /dev/null 2>&1 || true

echo -e "\033[1;32m[Thành Công] Hệ thống đã sạch bóng - Sẵn sàng cài đặt v7.3!\033[0m"
