#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# Termux:Boot Integration for ZeroClaw-Android
# Places a persistent startup script in ~/.termux/boot/
# ============================================================

set -e

BOOT_DIR="/data/data/com.termux/files/home/.termux/boot"
BOOT_SCRIPT="$BOOT_DIR/zeroclaw-boot-sh"

echo "🔄 Initializing boot persistence (Termux:Boot)..."
mkdir -p "$BOOT_DIR"

cat <<'EOF' > "$BOOT_SCRIPT"
#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# ZeroClaw-Android Auto-Start Script
# (Executed on Android boot)
# ============================================================

# Acquire wake lock
termux-wake-lock

echo "🐾 ZeroClaw-Android: Initializing..."

# 1. Start SSH Daemon
sshd

# 2. Start Cloudflare Tunnel (if configured)
if [ -f "/data/data/com.termux/files/home/.cloudflared/tunnel.json" ]; then
    nohup cloudflared tunnel run &
fi

# 3. Start Admin Dashboard
cd /data/data/com.termux/files/home/ZeroClaw-Android/dashboard
nohup node server.js &

echo "✅ ZeroClaw-Android services started."
EOF

chmod +x "$BOOT_SCRIPT"
echo "✅ Persistence script installed to $BOOT_SCRIPT."
