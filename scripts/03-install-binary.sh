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

echo "[Thông tin] Đang tải mã nguồn Binary cho kiến trúc $ARCH..."
echo "  - URL: $DOWNLOAD_URL"
# Thử tải bản Android
if curl -L -f "$DOWNLOAD_URL" -o "$TAR_FILE" 2>/dev/null; then
    COMPILATION_REQUIRED=false
else
    echo -e "\033[33m[Thông báo] Không tìm thấy Binary build sẵn cho kiến trúc $ARCH.\033[0m"
    echo -e "Hệ thống sẽ chuyển sang chế độ: \033[1;36mBIÊN DỊCH TRỰC TIẾP TỪ MÃ NGUỒN\033[0m"
    COMPILATION_REQUIRED=true
fi

if [ "$COMPILATION_REQUIRED" = "true" ]; then
    echo "[Thông tin] Bắt đầu cài đặt bộ công cụ biên dịch (Rust, Cargo, Clang)..."
    pkg install rust clang make binutils -y || {
        echo -e "\033[31m[LỖI] Không thể cài đặt bộ công cụ biên dịch. Vui lòng kiểm tra lại mạng.\033[0m"
        exit 1
    }
    
    echo "[Thông tin] Tải mã nguồn ZeroClaw mới nhất..."
    cd "$TMP_DIR"
    rm -rf zeroclaw-src
    git clone https://github.com/zeroclaw-labs/zeroclaw zeroclaw-src --depth 1
    cd zeroclaw-src
    
    echo "[1/2] Đang biên dịch thuật toán ZeroClaw (Có thể mất 5-15 phút)..."
    # Dùng -j 1 để tránh lỗi OOM (Hết RAM) trên TV Box
    cargo build --release -j 1
    
    if [ -f "target/release/zeroclaw" ]; then
        cp target/release/zeroclaw "$BIN_DIR/zeroclaw"
        echo -e "\033[32m✅ Biên dịch thành công!\033[0m"
    else
        echo -e "\033[31m[LỖI] Biên dịch thất bại. Máy có thể đã hết RAM hoặc bộ nhớ.\033[0m"
        exit 1
    fi
else
    # Kiểm tra dung lượng file tải về
    FILE_SIZE=$(ls -lh "$TAR_FILE" | awk '{print $5}')
    echo "[Thông tin] Đã tải xong: $FILE_SIZE"

    if [ ! -s "$TAR_FILE" ]; then
        echo -e "\033[31m[LỖI] File tải về bị trống (0 bytes). Vui lòng thử lại.\033[0m"
        exit 1
    fi

    echo "[Thông tin] Giải nén và cấu hình..."
    cd "$TMP_DIR"
    tar -xzf "$TAR_FILE"

    # Kiểm tra file binary sau khi giải nén
    if [ ! -f "zeroclaw" ]; then
        echo -e "\033[31m[LỖI] Không thấy file 'zeroclaw' sau khi giải nén.\033[0m"
        ls -la
        exit 1
    fi

    # Gắn nhị phân vào Termux
    mv zeroclaw "$BIN_DIR/zeroclaw"
fi

chmod +x "$BIN_DIR/zeroclaw"
rm -f "$TAR_FILE"

echo "[Thông tin] Cài đặt ZeroClaw thành công."
