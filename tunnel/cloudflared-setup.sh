#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# Cloudflare Tunnel Setup for Termux (ARM64/aarch64)
# Downloads the correct binary from cloudflare/cloudflared
# ============================================================

set -e

CLOUDFLARE_DL_URL="https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm64"
BIN_DIR="/data/data/com.termux/files/usr/bin"

if [ -f "$BIN_DIR/cloudflared" ]; then
    echo "☁ cloudflared is already installed."
else
    echo "☁ Downloading cloudflared-linux-arm64..."
    curl -L "$CLOUDFLARE_DL_URL" -o "$BIN_DIR/cloudflared"
    chmod +x "$BIN_DIR/cloudflared"
    echo "✅ cloudflared binary installed to $BIN_DIR/cloudflared."
fi

# Initial configuration template
echo "☁ Initializing cloudflared config..."
# cloudflared tunnel login or other automation.
# (User needs to provide tunnel token/config on the device.)
