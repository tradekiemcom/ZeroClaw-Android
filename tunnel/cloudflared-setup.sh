#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# ZeroClaw Cloudflare Tunnel Setup
# Installs cloudflared and configures for claw.iz.life
# ============================================================

set -e

ZEROCLAW_HOME="$HOME/ZeroClaw-Android"
CLOUDFLARED_DIR="$HOME/.cloudflared"
TUNNEL_ENV="$ZEROCLAW_HOME/tunnel/.env"

echo "╔══════════════════════════════════════════╗"
echo "║   Cloudflare Tunnel Setup                ║"
echo "╚══════════════════════════════════════════╝"
echo ""

# ── 1. Install cloudflared ───────────────────────────────────
echo "[1/4] Installing cloudflared..."
if command -v cloudflared &>/dev/null; then
    CURRENT_VER=$(cloudflared --version 2>/dev/null | head -1)
    echo "  ✓ Already installed: $CURRENT_VER"
else
    ARCH=$(uname -m)
    if [ "$ARCH" = "aarch64" ]; then
        echo "  Downloading cloudflared for ARM64..."
        DOWNLOAD_URL="https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm64"
        curl -L "$DOWNLOAD_URL" -o "$PREFIX/bin/cloudflared" --progress-bar
        chmod +x "$PREFIX/bin/cloudflared"
        echo "  ✓ cloudflared installed: $(cloudflared --version 2>/dev/null | head -1)"
    else
        echo "  ✗ Unsupported architecture: $ARCH (expected aarch64)"
        exit 1
    fi
fi

echo ""

# ── 2. Create .env file ─────────────────────────────────────
echo "[2/4] Configuring tunnel token..."
if [ -f "$TUNNEL_ENV" ]; then
    source "$TUNNEL_ENV"
    if [ -n "$TUNNEL_TOKEN" ] && [ "$TUNNEL_TOKEN" != "your-cloudflare-tunnel-token-here" ]; then
        echo "  ✓ Token already configured"
    else
        echo "  Token not set yet."
        echo ""
        read -p "  Enter your Cloudflare Tunnel token (or press Enter to skip): " INPUT_TOKEN
        if [ -n "$INPUT_TOKEN" ]; then
            echo "TUNNEL_TOKEN=$INPUT_TOKEN" > "$TUNNEL_ENV"
            echo "  ✓ Token saved to $TUNNEL_ENV"
        else
            echo "  ○ Skipped. Edit $TUNNEL_ENV later."
        fi
    fi
else
    echo "  Creating .env template..."
    cp "$ZEROCLAW_HOME/tunnel/.env.example" "$TUNNEL_ENV" 2>/dev/null || \
        echo "TUNNEL_TOKEN=your-cloudflare-tunnel-token-here" > "$TUNNEL_ENV"
    echo "  ○ Edit $TUNNEL_ENV with your tunnel token."
fi

echo ""

# ── 3. Create cloudflared config ─────────────────────────────
echo "[3/4] Creating cloudflared config..."
mkdir -p "$CLOUDFLARED_DIR"
cp "$ZEROCLAW_HOME/tunnel/config.yml" "$CLOUDFLARED_DIR/config.yml" 2>/dev/null
echo "  ✓ Config written to $CLOUDFLARED_DIR/config.yml"

echo ""

# ── 4. Test connection ───────────────────────────────────────
echo "[4/4] Testing tunnel..."
if [ -f "$TUNNEL_ENV" ]; then
    source "$TUNNEL_ENV"
    if [ -n "$TUNNEL_TOKEN" ] && [ "$TUNNEL_TOKEN" != "your-cloudflare-tunnel-token-here" ]; then
        echo "  Starting test connection (5 second timeout)..."
        timeout 5 cloudflared tunnel --no-autoupdate run --token "$TUNNEL_TOKEN" >/dev/null 2>&1 &
        TEST_PID=$!
        sleep 3
        if kill -0 "$TEST_PID" 2>/dev/null; then
            echo "  ✓ Tunnel connected successfully"
            kill "$TEST_PID" 2>/dev/null
        else
            echo "  ⚠ Tunnel process exited. Check token validity."
        fi
    else
        echo "  ○ Skipped — token not configured"
    fi
else
    echo "  ○ Skipped — no .env file"
fi

echo ""
echo "╔══════════════════════════════════════════╗"
echo "║  ✓ Tunnel setup complete!                ║"
echo "║                                          ║"
echo "║  Domain: claw.iz.life                    ║"
echo "║  Config: ~/.cloudflared/config.yml       ║"
echo "║  Token:  tunnel/.env                     ║"
echo "╚══════════════════════════════════════════╝"
