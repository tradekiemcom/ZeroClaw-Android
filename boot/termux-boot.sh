#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# ZeroClaw Termux:Boot — Auto-start script
# Place in: ~/.termux/boot/zeroclaw-boot.sh
# ============================================================

ZEROCLAW_HOME="$HOME/ZeroClaw-Android"
LOG_DIR="$HOME/zeroclaw/logs"
LOG_FILE="$LOG_DIR/boot.log"
DASHBOARD_PORT=7643

mkdir -p "$LOG_DIR"

log() {
    echo "[$(date '+%Y-%m-%dT%H:%M:%S%z')] $1" >> "$LOG_FILE"
}

log "=========================================="
log "ZeroClaw Boot Sequence Initiated"
log "Device: $(getprop ro.product.model 2>/dev/null || echo 'unknown')"
log "Android: $(getprop ro.build.version.release 2>/dev/null || echo 'unknown')"
log "=========================================="

# ── Step 1: Acquire Wake Lock ────────────────────────────────
log "[1/5] Acquiring wake lock..."
termux-wake-lock 2>/dev/null
if [ $? -eq 0 ]; then
    log "  ✓ Wake lock acquired"
else
    log "  ✗ Wake lock failed (Termux:API installed?)"
fi

# ── Step 2: Start SSHD ──────────────────────────────────────
log "[2/5] Starting SSH daemon..."
if command -v sshd &>/dev/null; then
    sshd 2>/dev/null
    log "  ✓ SSHD started on port 8022"
else
    log "  ○ SSHD not installed (optional: pkg install openssh)"
fi

# ── Step 3: Start Cloudflare Tunnel ──────────────────────────
log "[3/5] Starting Cloudflare Tunnel..."
TUNNEL_ENV="$ZEROCLAW_HOME/tunnel/.env"
if [ -f "$TUNNEL_ENV" ]; then
    source "$TUNNEL_ENV"
    if [ -n "$TUNNEL_TOKEN" ] && [ "$TUNNEL_TOKEN" != "your-cloudflare-tunnel-token-here" ]; then
        # Kill any existing tunnel process
        pkill -f "cloudflared tunnel" 2>/dev/null
        sleep 1
        nohup cloudflared tunnel --no-autoupdate run --token "$TUNNEL_TOKEN" \
            >> "$LOG_DIR/tunnel.log" 2>&1 &
        TUNNEL_PID=$!
        log "  ✓ Cloudflare tunnel started (PID: $TUNNEL_PID)"
    else
        log "  ✗ Tunnel token not configured. Edit: $TUNNEL_ENV"
    fi
else
    log "  ✗ Tunnel .env not found: $TUNNEL_ENV"
fi

# ── Step 4: Start Admin Dashboard ────────────────────────────
log "[4/5] Starting Admin Dashboard on port $DASHBOARD_PORT..."
if [ -f "$ZEROCLAW_HOME/dashboard/server.js" ]; then
    # Kill any existing dashboard process
    pkill -f "node.*server.js" 2>/dev/null
    sleep 1
    cd "$ZEROCLAW_HOME/dashboard"
    nohup node server.js >> "$LOG_DIR/dashboard.log" 2>&1 &
    DASH_PID=$!
    log "  ✓ Dashboard started (PID: $DASH_PID, port: $DASHBOARD_PORT)"
else
    log "  ✗ Dashboard not found: $ZEROCLAW_HOME/dashboard/server.js"
fi

# ── Step 5: Verify Services ─────────────────────────────────
log "[5/5] Verifying services..."
sleep 3

check_service() {
    local name="$1"
    local pattern="$2"
    if pgrep -f "$pattern" >/dev/null 2>&1; then
        log "  ✓ $name is running"
    else
        log "  ✗ $name is NOT running"
    fi
}

check_service "SSHD" "sshd"
check_service "Cloudflare Tunnel" "cloudflared tunnel"
check_service "Dashboard" "node.*server.js"

log "=========================================="
log "ZeroClaw Boot Sequence Complete"
log "Dashboard: http://localhost:$DASHBOARD_PORT"
log "=========================================="
