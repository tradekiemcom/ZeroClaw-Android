#!/bin/bash
set -e

echo "[Info] Cài đặt ZeroClaw gốc (Thích ứng cho Termux)..."

# The official zeroclaw install script inherently detects Termux 
# and fetches the appropriate aarch64-linux-android artifacts.
curl -sL https://raw.githubusercontent.com/zeroclaw-labs/zeroclaw/master/install.sh | bash

# Create an alias if the path isn't magically sourced yet
if ! command -v zeroclaw &> /dev/null; then
    echo "zeroclaw chưa được tìm thấy ở PATH. Liên kết vào $PREFIX/bin..."
    ln -sf "$HOME/.zeroclaw/bin/zeroclaw" "$PREFIX/bin/zeroclaw" || true
fi

echo "[Info] ZeroClaw đã được cài đặt."
