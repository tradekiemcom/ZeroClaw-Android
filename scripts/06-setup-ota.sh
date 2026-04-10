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

mkdir -p ~/.zeroclaw/skills
mkdir -p ~/.zeroclaw/tools
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

# Tạo script đồng bộ OTA Daemon (v16.6)
rm -f ~/.zeroclaw/ota_sync.sh
cat << 'EOF' > ~/.zeroclaw/ota_sync.sh
#!/usr/bin/env bash
# ============================================================================
# ZERO-CLAW AUTONOMOUS OTA DAEMON (v16.6)
# ============================================================================

# 1. Môi trường & Version
SOFTWARE_VERSION="1.0"
[ -z "$PREFIX" ] && PREFIX="/data/data/com.termux/files/usr"
USR_BIN="$PREFIX/bin"
export PATH="$USR_BIN:$PATH"
SERVICE_PATH="$PREFIX/var/service/zeroclaw"

OTA_URL="https://ota.tradekiem.com/v1/sync"
DEVICE_ID="$($USR_BIN/getprop ro.product.model 2>/dev/null | tr -d ' ')-$($USR_BIN/getprop ro.serialno 2>/dev/null)"
[ "$DEVICE_ID" = "-" ] && DEVICE_ID="boss_tablet"

PASSPHRASE_FILE="$HOME/.zeroclaw/.secret_pass"
DEFAULT_TOKEN="TradeKiemCom123@!"

echo "[$(date)] OTA Daemon v16.6 started..."

while true; do
    # Thu thập Telemetry (Native)
    CPU_VAL=$(top -n 1 | grep "Id" | head -n 1 | awk '{print 100 - $8}' 2>/dev/null || echo "0")
    RAM_VAL=$(free -m | grep Mem | awk '{print $3"/"$2"MB"}' 2>/dev/null || echo "0/0MB")
    DISK_VAL=$(df -h /data | tail -n 1 | awk '{print $3"/"$2}' 2>/dev/null || echo "0/0")

    # Nạp Token
    [ -f "$PASSPHRASE_FILE" ] && DEVICE_TOKEN=$(cat "$PASSPHRASE_FILE") || DEVICE_TOKEN="$DEFAULT_TOKEN"

    # Gửi yêu cầu Sync kèm Telemetry
    raw_data=$($USR_BIN/curl -s -f --max-time 15 \
        "$OTA_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw&cpu=$CPU_VAL&ram=$RAM_VAL&disk=$DISK_VAL")
    
    if [ $? -eq 0 ]; then
        status=$(echo "$raw_data" | $USR_BIN/jq -r '.ota_status' 2>/dev/null)
        
        if [ "$status" = "pending_approval" ]; then
            echo "[WAIT] Thiết bị đang chờ duyệt trên Server quản trị..."
        elif [ "$status" = "active" ]; then
            # A. KIỂM TRA CẬP NHẬT PHẦN MỀM (Self-Update)
            remote_ver=$(echo "$raw_data" | $USR_BIN/jq -r '.version' 2>/dev/null)
            binary_url=$(echo "$raw_data" | $USR_BIN/jq -r '.binary_url' 2>/dev/null)
            
            if [ "$remote_ver" != "$SOFTWARE_VERSION" ] && [ "$binary_url" != "" ] && [ "$binary_url" != "null" ]; then
                echo "[UPDATE] Phát hiện bản nâng cấp $remote_ver. Đang tải..."
                $USR_BIN/curl -L -o "$USR_BIN/zeroclaw.tmp" "$binary_url"
                if [ $? -eq 0 ]; then
                    mv "$USR_BIN/zeroclaw.tmp" "$USR_BIN/zeroclaw"
                    chmod +x "$USR_BIN/zeroclaw"
                    SOFTWARE_VERSION="$remote_ver"
                    echo "[OK] Đã nâng cấp lên version $remote_ver."
                    sv restart zeroclaw 2>/dev/null || true
                fi
            fi

            # B. ĐỒNG BỘ CẤU HÌNH (TOML)
            enc_toml=$(echo "$raw_data" | $USR_BIN/jq -r '.encrypted_toml' 2>/dev/null)
            if [ -n "$enc_toml" ] && [ "$enc_toml" != "null" ]; then
                echo "$enc_toml" | $USR_BIN/openssl enc -d -aes-256-cbc -a -pbkdf2 -pass pass:"$DEVICE_TOKEN" > ~/.config/zeroclaw/config.toml.new 2>/dev/null
                if [ $? -eq 0 ] && [ -s ~/.config/zeroclaw/config.toml.new ]; then
                    mv ~/.config/zeroclaw/config.toml.new ~/.config/zeroclaw/config.toml
                    echo "[SYNC] Đã đồng bộ cấu hình mới."
                    sv restart zeroclaw 2>/dev/null || true
                fi
            fi

            # C. CHẠY HOT SCRIPTS
            echo "$raw_data" | $USR_BIN/jq -r '.hot_scripts[]?' 2>/dev/null | while read cmd; do eval "$cmd"; done
        fi
    else
        echo "[ERROR] Không thể kết nối OTA Server. Thử lại sau..."
    fi

    # Nghỉ 5 phút (300 giây) trước khi check lần tiếp theo
    sleep 300
done
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

echo -e "\033[32m[Thông tin] Đã kích hoạt Service Sync OTA (Omni-Agent v16.6) thành công.\033[0m"
