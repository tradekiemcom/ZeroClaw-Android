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

# Nhận diện hệ điều hành
OS_TYPE=$(uname)
IS_TERMUX=false
if [[ "$OS_TYPE" == "Linux" && -d "/data/data/com.termux" ]]; then
    IS_TERMUX=true
fi

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
echo -e "  ${YELLOW}Chế độ kết nối (Mode):${NC}"
echo -e "  1. mock  — Giả lập (Test logic, Telegram, API không cần cTrader thật)"
echo -e "  2. live  — KẾT NỐI THẬT (Giao dịch trực tiếp với server cTrader)"
echo -ne "${BLUE}Chọn mode${NC} [${CTRADER_MODE:-mock}]: "
read _mode_input
if [ "$_mode_input" = "2" ] || [ "$_mode_input" = "live" ]; then
    CTRADER_MODE="live"
elif [ "$_mode_input" = "1" ] || [ "$_mode_input" = "mock" ]; then
    CTRADER_MODE="mock"
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
    if [ "$IS_TERMUX" = true ]; then
        pkg install rust clang make binutils -y
    elif [[ "$OS_TYPE" == "Darwin" ]]; then
        if ! command -v brew &>/dev/null; then
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi
        brew install rust
    else
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    fi
fi

# ── Build ─────────────────────────────────────────────────────────────────────
cd "$PROJECT_DIR"

if ! command -v sqlite3 &>/dev/null; then
    echo -e "${YELLOW}Cài đặt sqlite3 để tạo schema cho compiler...${NC}"
    pkg install sqlite -y
fi

DB_FILE=$(echo "$DATABASE_URL" | sed 's/sqlite:\/\///')
echo -e "${YELLOW}Khởi tạo schema DB tạm tại $DB_FILE cho sqlx macro...${NC}"
sqlite3 "$DB_FILE" "
CREATE TABLE IF NOT EXISTS accounts (id INTEGER PRIMARY KEY, name TEXT NOT NULL, broker_account_id INTEGER NOT NULL UNIQUE, account_type TEXT NOT NULL DEFAULT 'demo', access_token TEXT, connected INTEGER NOT NULL DEFAULT 0, autotrade INTEGER NOT NULL DEFAULT 1, balance REAL NOT NULL DEFAULT 0, equity REAL NOT NULL DEFAULT 0, float_profit REAL NOT NULL DEFAULT 0, daily_pnl REAL NOT NULL DEFAULT 0, daily_target_profit REAL NOT NULL DEFAULT 0, daily_max_loss REAL NOT NULL DEFAULT 0, created_at TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS bots (id TEXT PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL DEFAULT 1, symbol TEXT NOT NULL DEFAULT 'XAUUSD', timeframe TEXT NOT NULL DEFAULT 'M15', daily_target_profit REAL NOT NULL DEFAULT 0, daily_max_loss REAL NOT NULL DEFAULT 0, daily_pnl REAL NOT NULL DEFAULT 0, trade_count_today INTEGER NOT NULL DEFAULT 0, created_at TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS api_clients (id TEXT PRIMARY KEY, name TEXT NOT NULL, api_key TEXT NOT NULL UNIQUE, source TEXT NOT NULL DEFAULT 'API', enabled INTEGER NOT NULL DEFAULT 1, description TEXT, allowed_actions TEXT NOT NULL DEFAULT '[]', request_count INTEGER NOT NULL DEFAULT 0, last_used_at TEXT, created_at TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS positions (id TEXT PRIMARY KEY, order_id TEXT NOT NULL, account_id INTEGER NOT NULL, bot_id TEXT NOT NULL, source TEXT NOT NULL, symbol TEXT NOT NULL, side TEXT NOT NULL, volume REAL NOT NULL, open_price REAL NOT NULL DEFAULT 0, sl REAL, tp REAL, pnl REAL NOT NULL DEFAULT 0, status TEXT NOT NULL DEFAULT 'open', opened_at TEXT NOT NULL, closed_at TEXT);
CREATE TABLE IF NOT EXISTS requests (id TEXT PRIMARY KEY, source TEXT NOT NULL, api_client_id TEXT, bot_id TEXT NOT NULL, action TEXT NOT NULL, payload TEXT NOT NULL, result TEXT, created_at TEXT NOT NULL);
"

# Linker fix cho Android/Termux
export CC=clang
export CXX=clang++
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=clang
export RUSTFLAGS="-C linker=clang"

echo -e "${YELLOW}Đang biên dịch (có thể mất 10-20 phút lần đầu)...${NC}"
if cargo build --release -j 1; then
    # Cài đặt binary
    if [ "$IS_TERMUX" = true ]; then
        cp target/release/iztrade "$PREFIX/bin/iztrade" 2>/dev/null || true
        chmod +x "$PREFIX/bin/iztrade" 2>/dev/null || true
    else
        mkdir -p "$HOME/.local/bin"
        cp target/release/iztrade "$HOME/.local/bin/iztrade"
        chmod +x "$HOME/.local/bin/iztrade"
        echo -e "${YELLOW}Đã cài đặt binary vào $HOME/.local/bin/iztrade${NC}"
        # Tạo symlink iz nếu chưa có
        ln -sf "$HOME/.local/bin/iztrade" "$HOME/.local/bin/iz"
    fi

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
