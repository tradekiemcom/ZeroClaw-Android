#!/bin/bash
set -e

echo "[Thông tin] Cài đặt công cụ nền tảng cho việc biên dịch..."
pkg update -y
pkg install -y \
    rust \
    binutils \
    clang \
    make \
    cmake \
    python \
    openssl \
    pkg-config \
    lld

# Tạo thư mục tạm an toàn trên bộ nhớ Android (Tránh lỗi Path/Hardlink)
mkdir -p "$PREFIX/tmp"
export TMPDIR="$PREFIX/tmp"

echo "[Thông tin] Cài đặt Dependencies hoàn tất."
