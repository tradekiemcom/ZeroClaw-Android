#!/bin/bash
set -e

# Đảm bảo PATH của Termux luôn được nạp
export PATH="$PREFIX/bin:$PATH"

BIN_DIR="$PREFIX/bin"
TMP_DIR="$PREFIX/tmp"

echo -e "\033[36m[Cấu Hình] Chế độ v17.8: Bắt đầu [Biên dịch Native] từ mã nguồn trực tiếp... \033[0m"
echo "[Thông tin] Quá trình này giúp loại bỏ hoàn toàn các xung đột từ bản build sẵn."

echo "[Thông tin] Chế độ v17.8: Bắt đầu [Biên dịch Native] từ mã nguồn để loại bỏ xung đột..."
COMPILATION_REQUIRED=true

if [ "$COMPILATION_REQUIRED" = "true" ]; then
    echo "[Thông tin] Thiết lập môi trường biên dịch (Fix Linker)..."
    pkg install rust clang make binutils -y
    
    # SỬA LỖI LINKER QUAN TRỌNG CHO ANDROID TRÊN TERMUX
    export CC=clang
    export CXX=clang++
    export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=clang
    export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=clang
    export RUSTFLAGS="-C linker=clang"
    
    echo "[Thông tin] Tải mã nguồn ZeroClaw mới nhất..."
    cd "$TMP_DIR"
    rm -rf zeroclaw-src
    git clone https://github.com/zeroclaw-labs/zeroclaw zeroclaw-src --depth 1
    cd zeroclaw-src
    
    echo "[1/2] Đang biên dịch ZeroClaw (Bản Release chuẩn)..."
    # Sử dụng profile release bản chuẩn để đảm bảo tương thích
    cargo build --release -j 1
    
    if [ -f "target/release/zeroclaw" ]; then
        cp target/release/zeroclaw "$BIN_DIR/zeroclaw"
        echo -e "\033[32m✅ Biên dịch Native thành công rực rỡ!\033[0m"
    else
        echo -e "\033[31m[LỖI] Biên dịch thất bại. Hãy đảm bảo máy có ít nhất 1.5GB trống.\033[0m"
        exit 1
    fi
fi

rm -f "$TAR_FILE"

echo "[Thông tin] Cài đặt ZeroClaw thành công."
