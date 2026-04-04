#!/bin/bash
set -e

BIN_DIR="$PREFIX/bin"
TMP_DIR="$PREFIX/tmp"

echo "[Thông tin] Bắt đầu tải tệp thực thi (Native Android Binary) đã được Build sẵn từ máy chủ ZeroClaw..."

# Lấy phiên bản mới nhất từ Github API
LATEST_VERSION=$(curl -s "https://api.github.com/repos/zeroclaw-labs/zeroclaw/releases/latest" | jq -r '.tag_name')

if [ -z "$LATEST_VERSION" ] || [ "$LATEST_VERSION" == "null" ]; then
    echo -e "\033[31m[LỖI] Không thể kết nối với Github API để lấy thông tin phiên bản gốc.\033[0m"
    echo "Hãy kiểm tra lại mạng hoặc thử lại sau."
    exit 1
fi

echo "[Thông tin] Phiên bản mới nhất: $LATEST_VERSION"

DOWNLOAD_URL="https://github.com/zeroclaw-labs/zeroclaw/releases/download/${LATEST_VERSION}/zeroclaw-aarch64-linux-android.tar.gz"
TAR_FILE="$TMP_DIR/zeroclaw-android.tar.gz"

echo "Đang tải mã nguồn nén từ: $DOWNLOAD_URL ..."
curl -L "$DOWNLOAD_URL" -o "$TAR_FILE"

echo "[Thông tin] Giải nén và cấu hình..."
cd "$TMP_DIR"
tar -xzf "$TAR_FILE"

# Gắn nhị phân vào Termux
mv zeroclaw "$BIN_DIR/zeroclaw"
chmod +x "$BIN_DIR/zeroclaw"

# Dọn dẹp
rm "$TAR_FILE"

echo "[Thông tin] Cài đặt ZeroClaw thành công."
