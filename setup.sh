#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# ZeroClaw — Master Setup Script
# Run inside Termux on Galaxy Note 10+
# ============================================================

set -e

ZEROCLAW_HOME="$(cd "$(dirname "$0")" && pwd)"
LOG_DIR="$HOME/zeroclaw/logs"

echo ""
echo "  ╔═══════════════════════════════════════════════╗"
echo "  ║                                               ║"
echo "  ║   🐾  ZeroClaw Android — Master Setup         ║"
echo "  ║                                               ║"
echo "  ║   Target: Galaxy Note 10+                     ║"
echo "  ║   Port:   7643                                ║"
echo "  ║   Domain: claw.iz.life                        ║"
echo "  ║                                               ║"
echo "  ╚═══════════════════════════════════════════════╝"
echo ""

mkdir -p "$LOG_DIR"

# ── Step 1: Install Termux Packages ──────────────────────────
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  [1/6] Installing Termux packages..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

pkg update -y
pkg upgrade -y
pkg install -y nodejs-lts openssh curl termux-api 2>/dev/null || {
    echo "  ⚠ Some packages failed. Trying individually..."
    pkg install -y nodejs-lts 2>/dev/null || echo "  ✗ nodejs-lts failed"
    pkg install -y openssh 2>/dev/null || echo "  ○ openssh (optional)"
    pkg install -y curl 2>/dev/null || echo "  ○ curl (optional)"
    pkg install -y termux-api 2>/dev/null || echo "  ○ termux-api (optional)"
}

echo "  ✓ Node.js: $(node --version 2>/dev/null || echo 'not found')"
echo ""

# ── Step 2: Set file permissions ─────────────────────────────
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  [2/6] Setting file permissions..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

find "$ZEROCLAW_HOME" -name "*.sh" -exec chmod 755 {} \;
echo "  ✓ All .sh files set to 755"
echo ""

# ── Step 3: Setup Cloudflare Tunnel ──────────────────────────
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  [3/6] Cloudflare Tunnel Setup..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

bash "$ZEROCLAW_HOME/tunnel/cloudflared-setup.sh"
echo ""

# ── Step 4: Install Boot Script ──────────────────────────────
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  [4/6] Installing Boot Script..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

bash "$ZEROCLAW_HOME/boot/install-boot.sh"
echo ""

# ── Step 5: Configure Dashboard ──────────────────────────────
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  [5/6] Dashboard Configuration..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "  Default credentials:"
echo "    Username: admin"
echo "    Password: ZeroClaw@2026"
echo ""
echo "  ⚠  Change password after first login!"
echo "  ✓ Dashboard ready on port 7643"
echo ""

# ── Step 6: Shield Reminder ──────────────────────────────────
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  [6/6] Battery & Process Shield..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "  ⚠  Shield requires ADB from a PC/Mac."
echo "  Run from your computer:"
echo ""
echo "    cd ZeroClaw-Android"
echo "    bash shield/setup-shield.sh"
echo ""
echo "  Then verify:"
echo "    bash shield/verify-shield.sh"
echo ""

# ── Done ─────────────────────────────────────────────────────
echo ""
echo "  ╔═══════════════════════════════════════════════╗"
echo "  ║                                               ║"
echo "  ║   🐾  Setup Complete!                         ║"
echo "  ║                                               ║"
echo "  ║   Start:  ./zeroclaw.sh start                 ║"
echo "  ║   Status: ./zeroclaw.sh status                ║"
echo "  ║   Logs:   ./zeroclaw.sh logs                  ║"
echo "  ║                                               ║"
echo "  ║   Local:  http://localhost:7643                ║"
echo "  ║   Remote: https://claw.iz.life                ║"
echo "  ║                                               ║"
echo "  ╚═══════════════════════════════════════════════╝"
echo ""
