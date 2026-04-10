#!/bin/bash
set -e

# Đảm bảo PATH của Termux luôn được nạp
export PATH="$PREFIX/bin:$PATH"

BIN_DIR="$PREFIX/bin"
TMP_DIR="$PREFIX/tmp"

echo "[Thông tin] Bắt đầu tải tệp thực thi (Native Android Binary) đã được Build sẵn từ máy chủ ZeroClaw..."

# Lấy phiên bản mới nhất từ Github (Tránh lỗi do thiếu jq trên một số dòng Termux)
# https://github.com/zeroclaw-labs/zeroclaw/releases/latest sẽ chuyển hướng tới URL chứa version
LATEST_VERSION=$(curl -s -L -I -o /dev/null -w '%{url_effective}\n' "https://github.com/zeroclaw-labs/zeroclaw/releases/latest" | awk -F'/' '{print $NF}')


if [ -z "$LATEST_VERSION" ] || [ "$LATEST_VERSION" == "null" ]; then
    echo -e "\033[31m[LỖI] Không thể kết nối với Github API để lấy thông tin phiên bản gốc.\033[0m"
    echo "Hãy kiểm tra lại mạng hoặc thử lại sau."
    exit 1
fi

echo "[Thông tin] Phiên bản mới nhất: $LATEST_VERSION"

# Nhận dạng kiến trúc thiết bị
ARCH=$(uname -m)
case "$ARCH" in
    aarch64)
        BINARY_TARGET="aarch64-linux-android"
        ;;
    armv7l|armv8l|arm)
        BINARY_TARGET="armv7-linux-android"
        ;;
    *)
        echo -e "\033[31m[LỖI] Kiến trúc CPU ($ARCH) chưa được hỗ trợ chính thức binary.\033[0m"
        echo "Vui lòng liên hệ hỗ trợ hoặc thử build từ mã nguồn."
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/zeroclaw-labs/zeroclaw/releases/download/${LATEST_VERSION}/zeroclaw-${BINARY_TARGET}.tar.gz"
TAR_FILE="$TMP_DIR/zeroclaw-android.tar.gz"

echo "[Thông tin] Đang kiểm tra Binary cho kiến trúc $ARCH..."

# Thử tải bản Android chính chủ trước
if curl -L -f "$DOWNLOAD_URL" -o "$TAR_FILE" 2>/dev/null; then
    COMPILATION_REQUIRED=false
else
    echo -e "\033[33m[Thông báo] Không tìm thấy Binary Android build sẵn. Bắt đầu [Biên dịch Native]...\033[0m"
    COMPILATION_REQUIRED=true
fi

if [ "$COMPILATION_REQUIRED" = "true" ]; then
    echo "[Thông tin] Thiết lập môi trường biên dịch (Fix Linker)..."
    pkg install rust clang make binutils -y
    
    # SỬA LỖI LINKER QUAN TRỌNG CHO ARMv7 TRÊN TERMUX
    export CC=clang
    export CXX=clang++
    export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=clang
    
    echo "[Thông tin] Tải mã nguồn ZeroClaw mới nhất..."
    cd "$TMP_DIR"
    rm -rf zeroclaw-src
    git clone https://github.com/zeroclaw-labs/zeroclaw zeroclaw-src --depth 1
    cd zeroclaw-src
    
    echo "[1/2] Đang biên dịch ZeroClaw (Dành riêng cho chip của bạn)..."
    # Sử dụng profile release-small để giảm kích thước và j1 để tiết kiệm RAM
    cargo build --profile release-small -j 1
    
    if [ -f "target/release-small/zeroclaw" ]; then
        cp target/release-small/zeroclaw "$BIN_DIR/zeroclaw"
        echo -e "\033[32m✅ Biên dịch Native thành công rực rỡ!\033[0m"
    else
        echo -e "\033[31m[LỖI] Biên dịch thất bại. Hãy đảm bảo máy có ít nhất 1.5GB trống.\033[0m"
        exit 1
    fi
else
    echo "[Thông tin] Giải nén và cấu hình..."
    cd "$TMP_DIR"
    tar -xzf "$TAR_FILE"
    mv zeroclaw "$BIN_DIR/zeroclaw"
    chmod +x "$BIN_DIR/zeroclaw"
fi

rm -f "$TAR_FILE"

echo "[Thông tin] Cài đặt ZeroClaw thành công."
