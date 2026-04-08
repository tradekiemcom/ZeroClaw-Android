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

# Đường dẫn bash chuẩn trong Termux
TERMUX_BASH="/data/data/com.termux/files/usr/bin/bash"

cat << EOF > "$SVDIR/zeroclaw/run"
#!$TERMUX_BASH
exec zeroclaw gateway 2>&1
EOF
chmod +x "$SVDIR/zeroclaw/run"

cat << EOF > "$SVDIR/zeroclaw/log/run"
#!$TERMUX_BASH
svlogd -tt ~/.zeroclaw/log
EOF
chmod +x "$SVDIR/zeroclaw/log/run"
mkdir -p ~/.zeroclaw/log

# Tạo script đồng bộ OTA
rm -f ~/.zeroclaw/ota_sync.sh
cat << 'EOF' > ~/.zeroclaw/ota_sync.sh
#!/data/data/com.termux/files/usr/bin/bash
# ============================================================================
# ZERO-TOUCH OTA SYNC (Version: 2.0.2)
# Tải và giải mã cấu hình tập trung từ Sếp Trade Kiếm Cơm
# ============================================================================

USR_BIN="/data/data/com.termux/files/usr/bin"
export SVDIR="/data/data/com.termux/files/usr/var/service"
SERVICE_PATH="$SVDIR/zeroclaw"
hash -r 2>/dev/null || true

# Tự động nạp PATH nếu thiếu
export PATH="/data/data/com.termux/files/usr/bin:/data/data/com.termux/files/usr/bin/applets:$PATH"

OTA_URL="https://ota.tradekiem.com/v1/sync"
DEVICE_ID="$($USR_BIN/getprop ro.product.model 2>/dev/null | tr -d ' ')-$($USR_BIN/getprop ro.serialno 2>/dev/null)"
if [ "$DEVICE_ID" = "-" ]; then DEVICE_ID="note10_boss"; fi

PASSPHRASE_FILE="$HOME/.zeroclaw/.secret_pass"

# Giải phóng Port 42617 trước khi restart service
if [ -f "$USR_BIN/lsof" ]; then
    $USR_BIN/lsof -ti:42617 | xargs kill -9 2>/dev/null || true
fi

# Tự động tạo hoặc nạp Device Token
if [ ! -f "$PASSPHRASE_FILE" ]; then
    echo -e "\033[36m[Cơ Chế Bảo Mật] Đang thiết lập mã bảo mật mặc định...\033[0m"
    echo -n "TradeKiemCom123@!" > "$PASSPHRASE_FILE"
fi

DEVICE_TOKEN=$(cat "$PASSPHRASE_FILE")
echo -e "\033[32mĐang đồng bộ cấu hình bảo mật (ID: $DEVICE_ID) [v2.0.2]...\033[0m"

# Vòng lặp thử giải mã tối đa 3 lần
MAX_RETRIES=3
RETRY_COUNT=0
SYNC_SUCCESS=false

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    # Lần 1: Thử link chính
    raw_data=$($USR_BIN/curl -s -f --max-time 15 "$OTA_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw")
    if [ $? -ne 0 ]; then
        FALLBACK_URL="https://ota.tradekiemcom.workers.dev/v1/sync"
        echo -e "\033[33m[Cảnh Báo] ota.tradekiem.com lỗi/timeout, chuyển sang Fallback...\033[0m"
        raw_data=$($USR_BIN/curl -s -f --max-time 15 "$FALLBACK_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw")
    fi

    enc_toml=$(echo "$raw_data" | $USR_BIN/jq -r '.encrypted_toml' 2>/dev/null)
    status=$(echo "$raw_data" | $USR_BIN/jq -r '.ota_status' 2>/dev/null)

    if [ "$status" = "pending_approval" ]; then
        echo -e "\033[1;33m[CHỜ PHÊ DUYỆT] Thiết bị ($DEVICE_ID) chờ được duyệt trên Cloudflare KV!\033[0m"
        break
    fi

    if [ -n "$enc_toml" ] && [ "$enc_toml" != "null" ]; then
        # Thử giải mã
        echo "$enc_toml" | $USR_BIN/openssl enc -d -aes-256-cbc -a -pbkdf2 -pass pass:"$DEVICE_TOKEN" > ~/.config/zeroclaw/config.toml.new 2>/dev/null
        
        if [ $? -eq 0 ] && [ -s ~/.config/zeroclaw/config.toml.new ]; then
            mv ~/.config/zeroclaw/config.toml.new ~/.config/zeroclaw/config.toml
            echo -e "\033[1;32m✅ Đồng bộ & Giải mã OTA thành công!\033[0m"
            SYNC_SUCCESS=true
            
            # Thực thi các lệnh Hot Scripts
            echo "$raw_data" | $USR_BIN/jq -r '.hot_scripts[]?' 2>/dev/null | while read cmd; do 
                eval "$cmd"
            done
            break
        fi
    fi

    # Nếu đến đây là thất bại
    RETRY_COUNT=$((RETRY_COUNT + 1))
    echo -e "\033[31m[LỖI] Giải mã OTA thất bại (Lần $RETRY_COUNT/$MAX_RETRIES). Mã hiện tại: $DEVICE_TOKEN\033[0m"
    
    if [ $RETRY_COUNT -lt $MAX_RETRIES ]; then
        echo -e "Vui lòng nhập lại Token bảo mật đúng của anh (hoặc nhấn Enter để thử lại mã cũ):"
        read -p "Token: " NEW_TOKEN
        if [ -n "$NEW_TOKEN" ]; then
            DEVICE_TOKEN="$NEW_TOKEN"
            echo -n "$DEVICE_TOKEN" > "$PASSPHRASE_FILE"
        fi
    fi
done

if [ "$SYNC_SUCCESS" = "false" ]; then
    echo -e "\033[33m[THÔNG BÁO] Không thể đồng bộ OTA sau $MAX_RETRIES lần thử. Bỏ qua bước này và cài đặt tiếp.\033[0m"
    # Khởi động lại service bằng cấu hình cũ hoặc mặc định
    sv restart "$SERVICE_PATH" || true
else
    if command -v termux-wake-lock >/dev/null 2>&1; then termux-wake-lock; fi
    if command -v adb >/dev/null 2>&1; then adb connect localhost:5555 || true; fi
    sv restart "$SERVICE_PATH" || true
    echo -e "\033[1;32m>>> HỆ THỐNG ĐÃ SẴN SÀNG <<<\033[0m"
fi
EOF

chmod +x ~/.zeroclaw/ota_sync.sh

# Bật service mặc định
if command -v sv-enable >/dev/null 2>&1; then
    sv-enable zeroclaw
fi

# Gắn vào Termux boot
cat << 'EOF' > ~/.termux/boot/start_ota.sh
#!/data/data/com.termux/files/usr/bin/bash
termux-wake-lock
bash ~/.zeroclaw/ota_sync.sh >> ~/.zeroclaw/ota_boot.log 2>&1
EOF
chmod +x ~/.termux/boot/start_ota.sh

echo -e "\033[32m[Thông tin] Đã kích hoạt Service Sync OTA thành công.\033[0m"
