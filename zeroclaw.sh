#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# ZeroClaw CLI — Service Manager
# Usage: ./zeroclaw.sh [start|stop|status|logs|dashboard]
# ============================================================

ZEROCLAW_HOME="$(cd "$(dirname "$0")" && pwd)"
LOG_DIR="$HOME/zeroclaw/logs"
DASHBOARD_PORT=7643

mkdir -p "$LOG_DIR"

# ── Colors ──────────────────────────────────────────────────
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# ── Helpers ─────────────────────────────────────────────────
is_running() {
    pgrep -f "$1" >/dev/null 2>&1
}

print_status() {
    local name="$1"
    local pattern="$2"
    if is_running "$pattern"; then
        local pid=$(pgrep -f "$pattern" | head -1)
        printf "  ${GREEN}●${NC} %-22s ${GREEN}running${NC} (PID: %s)\n" "$name" "$pid"
    else
        printf "  ${RED}●${NC} %-22s ${RED}stopped${NC}\n" "$name"
    fi
}

# ── Commands ────────────────────────────────────────────────
cmd_start() {
    echo ""
    echo -e "  ${CYAN}🐾 ZeroClaw — Starting Services${NC}"
    echo ""

    # Wake lock
    termux-wake-lock 2>/dev/null && \
        echo -e "  ${GREEN}✓${NC} Wake lock acquired" || \
        echo -e "  ${YELLOW}○${NC} Wake lock (install Termux:API)"

    # Tunnel
    if [ -f "$ZEROCLAW_HOME/tunnel/.env" ]; then
        source "$ZEROCLAW_HOME/tunnel/.env"
        if [ -n "$TUNNEL_TOKEN" ] && [ "$TUNNEL_TOKEN" != "your-cloudflare-tunnel-token-here" ]; then
            if ! is_running "cloudflared tunnel"; then
                nohup cloudflared tunnel --no-autoupdate run --token "$TUNNEL_TOKEN" \
                    >> "$LOG_DIR/tunnel.log" 2>&1 &
                echo -e "  ${GREEN}✓${NC} Cloudflare Tunnel started"
            else
                echo -e "  ${YELLOW}○${NC} Cloudflare Tunnel already running"
            fi
        else
            echo -e "  ${RED}✗${NC} Tunnel token not configured (edit tunnel/.env)"
        fi
    else
        echo -e "  ${RED}✗${NC} tunnel/.env not found"
    fi

    # Dashboard
    if ! is_running "node.*server.js"; then
        cd "$ZEROCLAW_HOME/dashboard"
        nohup node server.js >> "$LOG_DIR/dashboard.log" 2>&1 &
        echo -e "  ${GREEN}✓${NC} Dashboard started on port $DASHBOARD_PORT"
    else
        echo -e "  ${YELLOW}○${NC} Dashboard already running"
    fi

    # SSHD
    if command -v sshd &>/dev/null; then
        if ! is_running "sshd"; then
            sshd 2>/dev/null
            echo -e "  ${GREEN}✓${NC} SSHD started"
        else
            echo -e "  ${YELLOW}○${NC} SSHD already running"
        fi
    fi

    echo ""
    echo -e "  ${CYAN}Dashboard:${NC} http://localhost:$DASHBOARD_PORT"
    echo -e "  ${CYAN}Remote:${NC}    https://your-domain.com"
    echo ""
}

cmd_stop() {
    echo ""
    echo -e "  ${CYAN}🐾 ZeroClaw — Stopping Services${NC}"
    echo ""

    pkill -f "cloudflared tunnel" 2>/dev/null && \
        echo -e "  ${GREEN}✓${NC} Tunnel stopped" || \
        echo -e "  ${YELLOW}○${NC} Tunnel was not running"

    pkill -f "node.*server.js" 2>/dev/null && \
        echo -e "  ${GREEN}✓${NC} Dashboard stopped" || \
        echo -e "  ${YELLOW}○${NC} Dashboard was not running"

    termux-wake-unlock 2>/dev/null
    echo -e "  ${GREEN}✓${NC} Wake lock released"
    echo ""
}

cmd_status() {
    echo ""
    echo -e "  ${CYAN}🐾 ZeroClaw — Service Status${NC}"
    echo ""
    print_status "Cloudflare Tunnel" "cloudflared tunnel"
    print_status "Admin Dashboard" "node.*server.js"
    print_status "SSH Daemon" "sshd"
    echo ""
    echo -e "  ${CYAN}Port:${NC}   $DASHBOARD_PORT"
    echo -e "  ${CYAN}Domain:${NC} your-domain.com"
    echo -e "  ${CYAN}Logs:${NC}   $LOG_DIR/"
    echo ""
}

cmd_logs() {
    local log_name="${1:-boot}"
    local log_file="$LOG_DIR/${log_name}.log"
    if [ -f "$log_file" ]; then
        echo ""
        echo -e "  ${CYAN}── $log_name.log ──${NC}"
        echo ""
        tail -50 "$log_file"
    else
        echo -e "  ${RED}✗${NC} Log not found: $log_file"
        echo "  Available: boot, tunnel, dashboard"
    fi
}

cmd_restart() {
    cmd_stop
    sleep 2
    cmd_start
}

# ── Main ────────────────────────────────────────────────────
case "${1}" in
    start)    cmd_start ;;
    stop)     cmd_stop ;;
    restart)  cmd_restart ;;
    status)   cmd_status ;;
    logs)     cmd_logs "${2}" ;;
    *)
        echo ""
        echo "  🐾 ZeroClaw CLI"
        echo ""
        echo "  Usage: $0 <command>"
        echo ""
        echo "  Commands:"
        echo "    start     Start all ZeroClaw services"
        echo "    stop      Stop all services"
        echo "    restart   Restart all services"
        echo "    status    Show service status"
        echo "    logs      View logs (boot|tunnel|dashboard)"
        echo ""
        echo "  Examples:"
        echo "    $0 start"
        echo "    $0 logs tunnel"
        echo ""
        ;;
esac
