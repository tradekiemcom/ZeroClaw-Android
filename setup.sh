#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# ZeroClaw-Android — Master Setup Script
# Automated installation for Termux on Android
# ============================================================

set -e

# Detect Termux environment
if [ -z "$TERMUX_VERSION" ]; then
    echo "⚠ This script is intended for Termux on Android."
    echo "If you are testing, proceed with caution."
fi

# Paths
ZEROCLAW_DIR="$(cd "$(dirname "$0")" && pwd)"
PREFIX="/data/data/com.termux/files/usr"
HOME_DIR="/data/data/com.termux/files/home"

echo "🐾 ZeroClaw-Android: Starting Master Setup..."

# 1. Update and install core dependencies
echo "📦 Installing system dependencies..."
pkg update -y
pkg upgrade -y
pkg install -y nodejs-lts git curl termux-api termux-boot

# 2. Setup Cloudflare Tunnel (ARM64 Downloader)
echo "☁ Setting up Cloudflare Tunnel..."
bash "$ZEROCLAW_DIR/tunnel/cloudflared-setup.sh"

# 3. Setup Boot persistence
echo "🔄 Setting up persistence (Termux:Boot)..."
bash "$ZEROCLAW_DIR/boot/install-boot.sh"

# 4. Final configuration
echo "✅ Master setup complete!"
echo "Check README.md for the 'Shield Setup' step (one-time ADB required)."
