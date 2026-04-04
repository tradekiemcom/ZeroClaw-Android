#!/bin/bash
set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_DIR="$PROJECT_ROOT/core"
export TMPDIR="$PREFIX/tmp"

if [ ! -d "$CORE_DIR/src" ]; then
    echo -e "\033[31m[LỖI] Mã nguồn 'core/' không tồn tại. Đang thử clone lại submodule...\033[0m"
    cd "$PROJECT_ROOT" && git submodule update --init --recursive
fi

echo "[Thông tin] Bắt đầu quá trình biên dịch ZeroClaw. Quá trình này sẽ tốn thời gian phụ thuộc vào chip điện thoại (10 - 20 phút)..."
cd "$CORE_DIR"

# Tùy biến Rust Flags để biên dịch ổn định trên Android Bionic
export CARGO_BUILD_JOBS=$(nproc)
if [ "$CARGO_BUILD_JOBS" -gt 4 ]; then
    # Giới hạn số luồng build xuống 4 để tránh quá tải nhiệt / sập RAM
    export CARGO_BUILD_JOBS=4
fi

# Biên dịch tệp nhị phân release
cargo build --release

echo "[Thông tin] Biên dịch thành công! Gắn lối tắt lệnh (symlink) vào hệ thống..."
ln -sf "$CORE_DIR/target/release/zeroclaw" "$PREFIX/bin/zeroclaw"

echo "[Thông tin] ZeroClaw đã sẵn sàng hoạt động."
