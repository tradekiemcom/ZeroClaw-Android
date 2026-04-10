#!/bin/bash
set -e

# Đảm bảo PATH của Termux luôn được nạp
export PATH="$PREFIX/bin:$PATH"

BIN_DIR="$PREFIX/bin"
TMP_DIR="$PREFIX/tmp"

echo -e "\n\033[36m[Lựa Chọn] Sếp muốn cài đặt theo cách nào cho Note 10+?\033[0m"
echo -e "  1. [NHANH] Tải bản Build siêu tốc (Đã tối ưu cho Note 10+ / aarch64) - Mất 10 giây"
echo -e "  2. [NATIVE] Tự biên dịch từ mã nguồn (Sạch tuyệt đối) - Mất 15-20 phút"
read -p "Lựa chọn (1/2): " build_choice

if [[ "$build_choice" == "1" ]]; then
    echo -e "\n\033[32m[>>>] Đang tải bản Build tối ưu cho Samsung Note 10+...\033[0m"
    # Lấy phiên bản mới nhất từ Github
    LATEST_VERSION=$(curl -s -L -I -o /dev/null -w '%{url_effective}\n' "https://github.com/zeroclaw-labs/zeroclaw/releases/latest" | awk -F'/' '{print $NF}')
    BINARY_TARGET="aarch64-linux-android"
    DOWNLOAD_URL="https://github.com/zeroclaw-labs/zeroclaw/releases/download/${LATEST_VERSION}/zeroclaw-${BINARY_TARGET}.tar.gz"
    TAR_FILE="$TMP_DIR/zeroclaw-android.tar.gz"

    if curl -L -f "$DOWNLOAD_URL" -o "$TAR_FILE" 2>/dev/null; then
        cd "$TMP_DIR"
        tar -xzf "$TAR_FILE"
        mv zeroclaw "$BIN_DIR/zeroclaw"
        chmod +x "$BIN_DIR/zeroclaw"
        rm -f "$TAR_FILE"
        echo -e "\033[32m✅ Tải bản Build siêu tốc thành công!\033[0m"
    else
        echo -e "\033[31m[LỖI] Không thể tải bản build sẵn. Đang chuyển hướng sang biên dịch Native...\033[0m"
        build_choice="2"
    fi
fi

if [[ "$build_choice" == "2" ]]; then
    echo -e "\n\033[33m[>>>] Bắt đầu biên dịch Native (Sẽ mất khá nhiều thời gian)...\033[0m"
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
    # Sử dụng j 1 để tránh treo máy Note 10+
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
