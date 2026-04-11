#!/bin/bash
# ==============================================================================
# iZFx.Trade – Interactive Setup Script
# Trading Execution Hub for cTrader Open API
# ==============================================================================

export PATH="$PREFIX/bin:$PATH"

BLUE='\033[36m'
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
BOLD='\033[1m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ENV_FILE="$PROJECT_DIR/.env"

echo -e "${BLUE}${BOLD}"
echo "  ╔══════════════════════════════════════════╗"
echo "  ║   iZFx.Trade – Setup Wizard v2.0        ║"
echo "  ║   Trading Execution Hub for cTrader     ║"
echo "  ╚══════════════════════════════════════════╝"
echo -e "${NC}"

echo -e "${YELLOW}Bước cài đặt sẽ hỏi các thông số cấu hình.${NC}"
echo -e "${YELLOW}Nhấn Enter để giữ nguyên giá trị mặc định (trong ngoặc vuông).${NC}\n"

# ── Đọc giá trị hiện có từ .env nếu tồn tại ──────────────────────────────────
if [ -f "$ENV_FILE" ]; then
    source "$ENV_FILE" 2>/dev/null || true
    echo -e "${GREEN}✓ Đọc cấu hình hiện tại từ .env${NC}\n"
fi

# ── Helper function ───────────────────────────────────────────────────────────
prompt() {
    local prompt_text="$1"
    local default_val="$2"
    local secret="$3"  # "secret" để ẩn input
    local result

    if [ -n "$default_val" ]; then
        echo -ne "${BLUE}${prompt_text}${NC} [${default_val:0:20}...]: "
    else
        echo -ne "${BLUE}${prompt_text}${NC}: "
    fi

    if [ "$secret" = "secret" ]; then
        read -s result
        echo ""
    else
        read result
    fi

    if [ -z "$result" ] && [ -n "$default_val" ]; then
        result="$default_val"
    fi
    echo "$result"
}

# ─────────────────────────────────────────────────────────────────────────────
echo -e "${BOLD}[1/5] cTrader Open API${NC}"
echo -e "──────────────────────────────────────────"
echo -e "Lấy thông tin tại: ${YELLOW}https://openapi.ctrader.com/apps${NC}\n"

CTRADER_CLIENT_ID=$(prompt "Client ID" "$CTRADER_CLIENT_ID")
CTRADER_SECRET=$(prompt "Secret Key" "$CTRADER_SECRET" "secret")
CTRADER_HOST="${CTRADER_HOST:-openapi.ctrader.com}"
CTRADER_PORT="${CTRADER_PORT:-5035}"

echo ""
echo -e "  ${YELLOW}Chế độ kết nối:${NC}"
echo -e "  1. mock  — Giả lập (test Telegram + API không cần cTrader thật)"
echo -e "  2. live  — Kết nối cTrader thật"
echo -ne "${BLUE}Chọn mode${NC} [${CTRADER_MODE:-mock}]: "
read _mode
if [ "$_mode" = "2" ] || [ "$_mode" = "live" ]; then
    CTRADER_MODE="live"
else
    CTRADER_MODE="${CTRADER_MODE:-mock}"
fi

# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}[2/5] Telegram Bot${NC}"
echo -e "──────────────────────────────────────────"
echo -e "Tạo bot tại: ${YELLOW}@BotFather${NC} trên Telegram\n"

TELEGRAM_BOT_TOKEN=$(prompt "Bot Token" "$TELEGRAM_BOT_TOKEN" "secret")

echo ""
echo -e "${YELLOW}Admin IDs:${NC} Chỉ những người này mới dùng được bot"
echo -e "Tìm Telegram ID của bạn: nhắn @userinfobot\n"
TELEGRAM_ADMIN_IDS=$(prompt "Admin Telegram ID (nhiều ID cách nhau dấu phẩy)" "$TELEGRAM_ADMIN_IDS")

# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}[3/5] Telegram Notification Channel/Group${NC}"
echo -e "──────────────────────────────────────────"
echo -e "Bot sẽ push thông báo trade vào channel/group này."
echo -e "Format: ${YELLOW}@username${NC} hoặc ${YELLOW}-100xxxxxxxxx${NC} (numeric ID)${NC}\n"

TELEGRAM_NOTIFY_CHAT_ID=$(prompt "Channel/Group ID" "$TELEGRAM_NOTIFY_CHAT_ID")

# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}[4/5] REST API Configuration${NC}"
echo -e "──────────────────────────────────────────"
echo -e "API key để xác thực kết nối từ ZeroClaw, MT5, Web...\n"

if [ -z "$API_KEY" ]; then
    # Generate API key ngẫu nhiên
    API_KEY=$(cat /proc/sys/kernel/random/uuid 2>/dev/null || uuidgen 2>/dev/null || date +%s | sha256sum | head -c 32)
    echo -e "${GREEN}✓ API Key mới đã được tạo tự động${NC}"
fi
API_KEY_DISPLAY="${API_KEY:0:8}..."
API_KEY_FROM_PROMPT=$(prompt "API Key" "$API_KEY_DISPLAY")
if [ "$API_KEY_FROM_PROMPT" != "$API_KEY_DISPLAY" ]; then
    API_KEY="$API_KEY_FROM_PROMPT"
fi

API_PORT=$(prompt "API Port" "${API_PORT:-7381}")

# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}[5/5] Database${NC}"
echo -e "──────────────────────────────────────────"
DATABASE_URL=$(prompt "SQLite path" "${DATABASE_URL:-sqlite://iztrade.db}")

# ─────────────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}[Xác nhận] Cấu hình của bạn:${NC}"
echo -e "──────────────────────────────────────────"
echo -e "  cTrader Client ID : ${GREEN}${CTRADER_CLIENT_ID:0:20}...${NC}"
echo -e "  cTrader Mode      : ${GREEN}${CTRADER_MODE}${NC}"
echo -e "  Telegram Bot      : ${GREEN}${TELEGRAM_BOT_TOKEN:0:15}...${NC}"
echo -e "  Admin IDs         : ${GREEN}${TELEGRAM_ADMIN_IDS}${NC}"
echo -e "  Notify Chat       : ${GREEN}${TELEGRAM_NOTIFY_CHAT_ID}${NC}"
echo -e "  API Port          : ${GREEN}${API_PORT}${NC}"
echo -e "  API Key           : ${GREEN}${API_KEY:0:8}...${NC}"
echo -e "  Database          : ${GREEN}${DATABASE_URL}${NC}"
echo ""
echo -ne "${YELLOW}Lưu cấu hình và cài đặt? (y/n): ${NC}"
read confirm
if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
    echo -e "${RED}Hủy bỏ.${NC}"
    exit 0
fi

# ─────────────────────────────────────────────────────────────────────────────
# Ghi file .env
cat > "$ENV_FILE" << EOF
# iZFx.Trade Configuration
# Generated by iztrade-setup.sh on $(date)

# ── cTrader Open API ──────────────────────────────────────────
CTRADER_CLIENT_ID=${CTRADER_CLIENT_ID}
CTRADER_SECRET=${CTRADER_SECRET}
CTRADER_HOST=${CTRADER_HOST}
CTRADER_PORT=${CTRADER_PORT}
CTRADER_MODE=${CTRADER_MODE}

# ── Telegram Bot ─────────────────────────────────────────────
TELEGRAM_BOT_TOKEN=${TELEGRAM_BOT_TOKEN}
TELEGRAM_ADMIN_IDS=${TELEGRAM_ADMIN_IDS}
TELEGRAM_NOTIFY_CHAT_ID=${TELEGRAM_NOTIFY_CHAT_ID}

# ── REST API ──────────────────────────────────────────────────
API_KEY=${API_KEY}
API_PORT=${API_PORT}

# ── Database ──────────────────────────────────────────────────
DATABASE_URL=${DATABASE_URL}

# ── Logging ───────────────────────────────────────────────────
LOG_LEVEL=info
EOF

echo -e "${GREEN}✅ Cấu hình đã được lưu vào .env${NC}"
chmod 600 "$ENV_FILE"  # Bảo vệ file chứa secrets

# ── Cài đặt Rust nếu chưa có ─────────────────────────────────────────────────
echo ""
echo -e "${BOLD}[Build] Biên dịch iZFx.Trade...${NC}"
echo -e "──────────────────────────────────────────"

if ! command -v cargo &>/dev/null; then
    echo -e "${YELLOW}Rust chưa được cài đặt. Đang cài...${NC}"
    pkg install rust clang make binutils -y
fi

# ── Build ─────────────────────────────────────────────────────────────────────
cd "$PROJECT_DIR"

# Linker fix cho Android/Termux
export CC=clang
export CXX=clang++
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=clang
export RUSTFLAGS="-C linker=clang"

echo -e "${YELLOW}Đang biên dịch (có thể mất 10-20 phút lần đầu)...${NC}"
if cargo build --release -j 1; then
    # Copy binary vào PATH
    cp target/release/iztrade "$PREFIX/bin/iztrade" 2>/dev/null || \
    cp target/release/iztrade "$HOME/.cargo/bin/iztrade" 2>/dev/null || true
    chmod +x "${PREFIX:-$HOME/.cargo}/bin/iztrade" 2>/dev/null || true

    echo ""
    echo -e "${GREEN}${BOLD}✅ iZFx.Trade đã được cài đặt thành công!${NC}"
    echo ""
    echo -e "${BOLD}Cách sử dụng:${NC}"
    echo -e "  ${YELLOW}iztrade${NC}           — Khởi động iZFx.Trade"
    echo -e "  ${YELLOW}iztrade --help${NC}    — Xem trợ giúp"
    echo ""
    echo -e "${BOLD}API Key của bạn:${NC}"
    echo -e "  ${GREEN}${API_KEY}${NC}"
    echo ""
    echo -e "${BOLD}Kiểm tra kết nối:${NC}"
    echo -e "  curl http://localhost:${API_PORT}/health"
    echo ""
    echo -e "${BOLD}Gửi lệnh qua API:${NC}"
    echo -e "  curl -X POST http://localhost:${API_PORT}/api/order \\"
    echo -e "    -H 'Authorization: Bearer ${API_KEY}' \\"
    echo -e "    -H 'Content-Type: application/json' \\"
    echo -e "    -d '{\"bot_id\":\"gold_scalper\",\"action\":\"OPEN\",\"symbol\":\"XAUUSD\",\"side\":\"buy\",\"volume\":0.01}'"
else
    echo -e "${RED}❌ Build thất bại. Kiểm tra lỗi ở trên.${NC}"
    exit 1
fi
