#!/bin/bash
set -e

echo "[Thông tin] Cài đặt các công cụ trích xuất và kết nối mạng..."
pkg update -y
pkg install -y \
    curl \
    jq \
    wget \
    tar \
    openssl

# Lệnh nâng cấp CA certificates để phòng tránh lỗi SSL khi curl tải Github API
pkg install -y ca-certificates

# Tạo thư mục tạm an toàn trên bộ nhớ Android (Tránh lỗi Path/Hardlink)
mkdir -p "$PREFIX/tmp"
export TMPDIR="$PREFIX/tmp"

echo "[Thông tin] Cài đặt thư viện nền tảng hoàn tất."
hash -r
