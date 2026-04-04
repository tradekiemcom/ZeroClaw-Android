#!/bin/bash
set -e

BIN_DIR="$PREFIX/bin"
CLOUDFLARE_DL_URL="https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm64"

echo "[Thông tin] Khởi tạo hệ thống mạng nội bộ & Cloudflare Tunnel..."

if [ ! -f "$BIN_DIR/cloudflared" ]; then
    echo "Tải bộ kết nối Cloudflare Zero Trust (Native ARM64)..."
    curl -L "$CLOUDFLARE_DL_URL" -o "$BIN_DIR/cloudflared"
    chmod +x "$BIN_DIR/cloudflared"
fi

echo "[Thông tin] Cloudflare Tunnel đã sẵn sàng."
echo 'Để chạy kết nối ra internet, bạn nên đăng ký một tunnel token và chạy lệnh:'
echo -e '\033[33mcloudflared service install <TOKEN_CUẢ_BẠN>\033[0m'
