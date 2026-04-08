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
    # Nếu không có bản Android 32-bit, dùng bản Generic ARM + Proot
    if [[ "$ARCH" == "arm"* ]]; then
        echo -e "\033[33m[Thông báo] Không tìm thấy Binary Android 32-bit. Sử dụng giải pháp [Generic ARM + Proot]...\033[0m"
        GENERIC_URL="https://github.com/zeroclaw-labs/zeroclaw/releases/download/${LATEST_VERSION}/zeroclaw-arm-unknown-linux-gnueabihf.tar.gz"
        pkg install proot -y
        curl -L -f "$GENERIC_URL" -o "$TAR_FILE"
        COMPILATION_REQUIRED=false
        USE_PROOT=true
    else
        echo -e "\033[31m[LỖI] Không tìm thấy Binary cho kiến trúc $ARCH.\033[0m"
        exit 1
    fi
fi

echo "[Thông tin] Giải nén và cấu hình..."
cd "$TMP_DIR"
tar -xzf "$TAR_FILE"

# Gắn nhị phân vào Termux
if [ "$USE_PROOT" = "true" ]; then
    mv zeroclaw "$BIN_DIR/zeroclaw.bin"
    # Tạo script wrapper để chạy qua proot (giả lập glibc)
    cat << EOF > "$BIN_DIR/zeroclaw"
#!/bin/bash
export PATH="\$PREFIX/bin:\$PATH"
proot -0 -b /dev -b /proc -b /sys "$BIN_DIR/zeroclaw.bin" "\$@"
EOF
    chmod +x "$BIN_DIR/zeroclaw" "$BIN_DIR/zeroclaw.bin"
else
    mv zeroclaw "$BIN_DIR/zeroclaw"
    chmod +x "$BIN_DIR/zeroclaw"
fi

rm -f "$TAR_FILE"

echo "[Thông tin] Cài đặt ZeroClaw thành công."
