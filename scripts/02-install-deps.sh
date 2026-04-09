#!/bin/bash
set -e

# Đảm bảo PATH của Termux luôn được nạp
export PATH="$PREFIX/bin:$PATH"

echo "[Thông tin] Đồng bộ và sửa lỗi Repository Termux (Fixing mirrors)..."
# Tự động chuyển mirror nếu bị lỗi (Dùng cho các dòng TVBox mạng yếu hoặc bị chặn)
if command -v termux-change-repo >/dev/null 2>&1; then
    echo "Đang thử tối ưu hóa mirror bằng termux-change-repo..."
    # Không thể chạy tương tác, nhưng có thể thử update lại
fi

echo "[Thông tin] Cập nhật danh sách gói Termux..."
pkg update -y || apt-get update -y || {
    echo -e "\033[33m[Cảnh Báo] pkg update lỗi. Thử dọn dẹp cache...\033[0m"
    rm -rf $PREFIX/var/lib/apt/lists/*
    pkg update -y
}

echo "[Thông tin] Cài đặt các công cụ nền tảng (Hardened Mode)..."
# Cài đặt từng gói để tránh lỗi một gói làm hỏng cả chuỗi
for pkg_name in curl jq wget tar openssl ca-certificates lsof termux-api android-tools; do
    echo "Đang cài đặt $pkg_name..."
    pkg install -y $pkg_name || apt-get install -y $pkg_name || echo "Bỏ qua lỗi cài đặt $pkg_name (có thể đã tồn tại)."
done

# Xác định đường dẫn Bin của Termux một cách linh động
TUX_BIN="$PREFIX/bin"

# Kiểm tra sự tồn tại của lệnh quan trọng và báo cáo đường dẫn
echo "[Thông tin] Kiểm tra môi trường thực thi:"
for cmd in openssl jq curl; do
    # 1. Thử lấy từ PATH hoặc dùng command -v
    CMD_PATH=$(command -v $cmd 2>/dev/null || echo "")
    
    # 2. Nếu không thấy, thử dùng đường dẫn chuẩn dựa trên PREFIX
    if [ -z "$CMD_PATH" ] && [ -f "$TUX_BIN/$cmd" ]; then
        CMD_PATH="$TUX_BIN/$cmd"
    fi
    
    if [ -z "$CMD_PATH" ]; then
        echo -e "\033[31m[LỖI] Không thấy lệnh $cmd. Đang thử cài đặt lại...\033[0m"
        pkg install $cmd -y || true
        CMD_PATH=$(command -v $cmd 2>/dev/null || echo "MISSING")
    fi
    
    if [ "$CMD_PATH" = "MISSING" ]; then
        echo -e "\033[31m[LỖI] Thất bại khi tìm kiếm $cmd. Vui lòng cài đặt thủ công.\033[0m"
        exit 1
    else
        echo "  - $cmd: $CMD_PATH"
    fi
done

# Cập nhật danh sách lệnh cho shell hiện tại
hash -r

# Tạo thư mục tạm an toàn trên bộ nhớ Android (Tránh lỗi Path/Hardlink)
mkdir -p "$PREFIX/tmp"
export TMPDIR="$PREFIX/tmp"

echo "[Thông tin] Cài đặt thư viện nền tảng hoàn tất."
hash -r
