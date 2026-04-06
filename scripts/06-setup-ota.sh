# ==============================================================================
# BƯỚC 6: Tích Hợp OTA Sync & Quản Trị Hệ Thống Từ Xa
# ==============================================================================

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
cat << 'EOF' > ~/.zeroclaw/ota_sync.sh
#!/data/data/com.termux/files/usr/bin/bash
# ============================================================================
# ZERO-TOUCH OTA SYNC
# Tải và giải mã cấu hình tập trung từ Sếp Trade Kiếm Cơm
# ============================================================================

export SVDIR="/data/data/com.termux/files/usr/var/service"
hash -r 2>/dev/null || true

OTA_URL="https://ota.tradekiem.com/v1/sync"
DEVICE_ID="$(getprop ro.product.model 2>/dev/null | tr -d ' ')-$(getprop ro.serialno 2>/dev/null)"
if [ "$DEVICE_ID" = "-" ]; then DEVICE_ID="note10_boss"; fi

PASSPHRASE_FILE="$HOME/.zeroclaw/.secret_pass"

# Giải phóng Port 42617 trước khi restart service
if command -v lsof >/dev/null 2>&1; then
    lsof -ti:42617 | xargs kill -9 2>/dev/null || true
fi

# Tự động tạo Device Token
if [ ! -f "$PASSPHRASE_FILE" ]; then
    echo -e "\033[36m[Zero-Touch] Đang khởi tạo mã bảo mật riêng cho thiết bị...\033[0m"
    if command -v openssl >/dev/null 2>&1; then
        openssl rand -hex 16 > "$PASSPHRASE_FILE"
    else
        echo -e "\033[31m[!] Lỗi: Không thấy lệnh openssl.\033[0m"
        exit 1
    fi
fi

DEVICE_TOKEN=$(cat "$PASSPHRASE_FILE")

echo -e "\033[32mĐang đồng bộ cấu hình bảo mật (ID: $DEVICE_ID)...\033[0m"

# Lần 1: Thử link chính
raw_data=$(curl -s -f --max-time 15 "$OTA_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw")
if [ $? -ne 0 ]; then
    FALLBACK_URL="https://ota.tradekiemcom.workers.dev/v1/sync"
    echo -e "\033[33m[Cảnh Báo] ota.tradekiem.com lỗi/timeout, chuyển sang Fallback...\033[0m"
    raw_data=$(curl -s -f --max-time 15 "$FALLBACK_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw")
fi

enc_toml=$(echo "$raw_data" | jq -r '.encrypted_toml' 2>/dev/null)
status=$(echo "$raw_data" | jq -r '.ota_status' 2>/dev/null)
pass="$DEVICE_TOKEN"

if [ "$status" = "pending_approval" ]; then
    echo -e "\033[1;33m[CHỜ PHÊ DUYỆT] Thiết bị ($DEVICE_ID) chờ được duyệt trên Cloudflare KV!\033[0m"
    echo -e "Vui lòng đổi trạng thái thiết bị thành 'approved' trong KV 'KV_DEVICES'."
    sleep 30
    SVDIR=$SVDIR sv restart zeroclaw || true
    exit 0
fi

if [ "$enc_toml" = "null" ] || [ -z "$enc_toml" ]; then
    echo -e "\033[31m[LỖI] Phân giải OTA thất bại.\033[0m"
    echo -e "Nội dung nhận được từ Server: \n$raw_data"
    SVDIR=$SVDIR sv restart zeroclaw || true
    exit 1
fi

echo "$enc_toml" | openssl enc -d -aes-256-cbc -a -pbkdf2 -pass pass:"$pass" > ~/.config/zeroclaw/config.toml.temp 2>/dev/null

if [ $? -eq 0 ]; then
    mv ~/.config/zeroclaw/config.toml.temp ~/.config/zeroclaw/config.toml
    echo -e "\033[1;32m✅ Giải mã Config thành công!\033[0m"
    echo "$raw_data" | jq -r '.hot_scripts[]?' | while read cmd; do 
        eval "$cmd"
    done
    
    if command -v termux-wake-lock >/dev/null 2>&1; then termux-wake-lock; fi
    if command -v adb >/dev/null 2>&1; then adb connect localhost:5555 || true; fi
    
    SVDIR=$SVDIR sv restart zeroclaw || true
    echo -e "\033[1;32m>>> HỆ THỐNG ĐÃ SẴN SÀNG <<<\033[0m"
else
    echo -e "\033[31m[LỖI] Giải mã OTA lỗi.\033[0m"
    SVDIR=$SVDIR sv restart zeroclaw || true
fi
EOF

chmod +x ~/.zeroclaw/ota_sync.sh

# Bật service mặc định
if command -v sv-enable >/dev/null 2>&1; then
    SVDIR=$PREFIX/var/service sv-enable zeroclaw
fi

# Gắn vào Termux boot
cat << 'EOF' > ~/.termux/boot/start_ota.sh
#!/data/data/com.termux/files/usr/bin/bash
termux-wake-lock
bash ~/.zeroclaw/ota_sync.sh >> ~/.zeroclaw/ota_boot.log 2>&1
EOF
chmod +x ~/.termux/boot/start_ota.sh

echo -e "\033[32m[Thông tin] Đã kích hoạt Service Sync OTA thành công.\033[0m"
