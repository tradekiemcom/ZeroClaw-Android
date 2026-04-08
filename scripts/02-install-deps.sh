#!/bin/bash
set -e

echo "[Thông tin] Cập nhật danh sách gói Termux..."
pkg update -y || { echo -e "\033[33m[Cảnh Báo] pkg update thất bại, thử dùng pkg upgrade...\033[0m"; pkg upgrade -y; }

echo "[Thông tin] Cài đặt các công cụ trích xuất và kết nối mạng..."
pkg install -y \
    curl \
    jq \
    wget \
    tar \
    openssl \
    ca-certificates \
    lsof

# Kiểm tra sự tồn tại của các lệnh quan trọng
for cmd in openssl jq curl; do
    if ! command -v $cmd &> /dev/null; then
        echo -e "\033[31m[LỖI] Không thể cài đặt $cmd. Vui lòng kiểm tra lại kết nối mạng.\033[0m"
        exit 1
    fi
done

# Cập nhật danh sách lệnh cho shell hiện tại
hash -r

# Tạo thư mục tạm an toàn trên bộ nhớ Android (Tránh lỗi Path/Hardlink)
mkdir -p "$PREFIX/tmp"
export TMPDIR="$PREFIX/tmp"

echo "[Thông tin] Cài đặt thư viện nền tảng hoàn tất."
hash -r
