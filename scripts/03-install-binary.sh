#!/bin/bash
set -e

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
curl -L -f "$DOWNLOAD_URL" -o "$TAR_FILE" || {
    echo -e "\033[31m[LỖI] Không tìm thấy Binary cho kiến trúc $ARCH trên server.\033[0m"
    echo "Thử kiểm tra lại phiên bản hoặc liên hệ quản trị viên."
    exit 1
}

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

# Chạy thử lệnh help để kiểm tra tương thích (Nếu có thể)
echo "[Thông tin] Kiểm tra tính tương thích của Binary..."
chmod +x zeroclaw
./zeroclaw --version > /dev/null 2>&1 || {
    echo -e "\033[33m[Cảnh Báo] Binary có vẻ không tương thích trực tiếp với môi trường này (Lỗi: $?).\033[0m"
    echo "Vẫn tiến hành cài đặt nhưng hệ thống có thể không chạy được."
}

# Gắn nhị phân vào Termux
mv zeroclaw "$BIN_DIR/zeroclaw"
chmod +x "$BIN_DIR/zeroclaw"

# Dọn dẹp
rm "$TAR_FILE"

echo "[Thông tin] Cài đặt ZeroClaw thành công."
