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

# 2. Xóa các tệp tin tạm của quá trình biên dịch Rust (nếu có)
echo "  - Loại bỏ các thành phần biên dịch cũ (~/.cargo, ~/.rustup)..."
rm -rf ~/.cargo ~/.rustup > /dev/null 2>&1 || true

# 3. Xóa thư mục làm việc tạm thời
echo "  - Xóa thư mục tạm thời và log cũ..."
rm -rf /data/data/com.termux/files/usr/tmp/zeroclaw-* > /dev/null 2>&1 || true
rm -rf ~/.zeroclaw/log/* > /dev/null 2>&1 || true

# 4. Gỡ bỏ các gói phụ trợ build nếu người dùng đã cài lỗi trước đó
echo "  - Gỡ bỏ các công cụ biên dịch nặng (rust, clang) để lấy lại dung lượng..."
pkg uninstall rust clang make binutils -y > /dev/null 2>&1 || true
pkg autoremove -y > /dev/null 2>&1 || true

echo -e "\033[32m[Thành Công] Đã giải phóng dung lượng. Hệ thống đã sạch sẽ.\033[0m"
