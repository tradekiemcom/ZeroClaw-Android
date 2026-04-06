#!/usr/bin/env bash
# ==============================================================================
# BƯỚC 6: Tích Hợp OTA Sync & Quản Trị Hệ Thống Từ Xa
# ==============================================================================

echo -e "\033[36m[6/6] Cài đặt Module Quản Trị OTA & Remote ADB...\033[0m"

# Nếu chạy ngoài Termux (test trên Mac), bỏ qua cài pkg
if [ -n "$PREFIX" ]; then
    pkg install android-tools termux-services openssl jq -y
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

echo "#!/usr/bin/env bash" > "$SVDIR/zeroclaw/run"
echo "exec zeroclaw gateway 2>&1" >> "$SVDIR/zeroclaw/run"
chmod +x "$SVDIR/zeroclaw/run"

echo "#!/usr/bin/env bash" > "$SVDIR/zeroclaw/log/run"
echo "svlogd -tt ~/.zeroclaw/log" >> "$SVDIR/zeroclaw/log/run"
chmod +x "$SVDIR/zeroclaw/log/run"
mkdir -p ~/.zeroclaw/log

# Tạo script đồng bộ OTA
cat << 'EOF' > ~/.zeroclaw/ota_sync.sh
#!/usr/bin/env bash
# ============================================================================
# ZERO-TOUCH OTA SYNC
# Tải và giải mã cấu hình tập trung từ Sếp Trade Kiếm Cơm
# ============================================================================

export SVDIR="$PREFIX/var/service"
# Cập nhật hash lệnh cho phiên bash hiện tại
hash -r 2>/dev/null || true

OTA_URL="https://ota.tradekiem.com/v1/sync"
DEVICE_ID="$(getprop ro.product.model 2>/dev/null | tr -d ' ')-$(getprop ro.serialno 2>/dev/null)"
if [ "$DEVICE_ID" = "-" ]; then DEVICE_ID="note10_boss"; fi

PASSPHRASE_FILE="$HOME/.zeroclaw/.secret_pass"

# Giải phóng Port 42617 trước khi restart service để tránh lỗi Address already in use
if command -v lsof >/dev/null 2>&1; then
    lsof -ti:42617 | xargs kill -9 2>/dev/null || true
fi

# Tự động tạo Device Token 1 lần duy nhất thay vì hỏi Mật khẩu
if [ ! -f "$PASSPHRASE_FILE" ]; then
    echo -e "\033[36m[Zero-Touch] Đang khởi tạo mã bảo mật riêng cho thiết bị...\033[0m"
    if command -v openssl >/dev/null 2>&1; then
        openssl rand -hex 16 > "$PASSPHRASE_FILE"
    else
        echo -e "\033[31m[!] Lỗi: Không thấy lệnh openssl. Sẽ dừng setup.\033[0m"
        exit 1
    fi
fi

DEVICE_TOKEN=$(cat "$PASSPHRASE_FILE")

echo -e "\033[32mĐang đồng bộ cấu hình bảo mật từ OTA Worker ($OTA_URL)...\033[0m"

# Lần 1: Thử link chính
raw_data=$(curl -s --max-time 15 "$OTA_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw")
enc_toml=$(echo "$raw_data" | jq -r '.encrypted_toml' 2>/dev/null)
status=$(echo "$raw_data" | jq -r '.ota_status' 2>/dev/null)

# Fallback: Thử link số 2
if [ "$enc_toml" = "null" ] && [ "$status" != "pending_approval" ]; then
    FALLBACK_URL="https://ota.tradekiemcom.workers.dev/v1/sync"
    echo -e "\033[33m[Cảnh Báo] Mất kết nối tới ota.tradekiem.com, tự động chuyển sang Fallback URL: $FALLBACK_URL\033[0m"
    raw_data=$(curl -s --max-time 15 "$FALLBACK_URL?id=$DEVICE_ID&token=$DEVICE_TOKEN&core=zeroclaw")
    enc_toml=$(echo "$raw_data" | jq -r '.encrypted_toml' 2>/dev/null)
    status=$(echo "$raw_data" | jq -r '.ota_status' 2>/dev/null)
fi

pass="$DEVICE_TOKEN"

if [ "$status" = "pending_approval" ]; then
    echo -e "\033[1;33m[CHỜ PHÊ DUYỆT] Thiết bị ($DEVICE_ID) đang được đưa vào danh sách chờ OTA!\033[0m"
    echo -e "Vui lòng mở Cloudflare Dashboard -> Workers & Pages -> KV -> KV_DEVICES"
    echo -e "Và đổi trạng thái của thiết bị '$DEVICE_ID' thành 'approved' để cấp cấu hình."
    echo -e "Sẽ thử lại sau 30 giây..."
    sleep 30
    sv restart zeroclaw || true
    exit 0
fi

if [ "$enc_toml" = "null" ] || [ -z "$enc_toml" ]; then
    echo -e "\033[31m[LỖI] Phân giải cấu hình OTA thất bại ở cả 2 đường truyền gốc và dự phòng.\033[0m"
    sv restart zeroclaw || true
    exit 1
fi

# Thực thi giải mã AES-256-CBC theo chuẩn OpenSSL PBKDF2
# Lưu file config tạm để tránh phá hủy file config gốc nếu giải mã sai
echo "$enc_toml" | openssl enc -d -aes-256-cbc -a -pbkdf2 -pass pass:"$pass" > ~/.config/zeroclaw/config.toml.temp 2>/dev/null

if [ $? -eq 0 ]; then
    mv ~/.config/zeroclaw/config.toml.temp ~/.config/zeroclaw/config.toml
    echo -e "\033[1;32m✅ Giải mã Config thành công!\033[0m"
    
    # Thực thi các lệnh điều khiển thiết bị từ xa (Hot Scripts)
    echo -e "Thực thi Hot Scripts..."
    echo "$raw_data" | jq -r '.hot_scripts[]?' | while read cmd; do 
        echo -e "\033[36m > RUN: $cmd \033[0m"
        eval "$cmd"
    done
    
    # Khởi động lại hệ thống bằng cách kick Service
    if command -v termux-wake-lock >/dev/null 2>&1; then termux-wake-lock; fi
    if command -v adb >/dev/null 2>&1; then 
        echo "Kết nối ADB nội hạt..."
        adb connect localhost:5555 || true
    fi
    
    echo -e "\033[1;33mĐang kích hoạt lại Service quản trị ngầm...\033[0m"
    sv restart zeroclaw || true
    
    echo -e "\033[1;32m>>> HỆ THỐNG ĐÃ SẴN SÀNG. BOSS CÓ QUYỀN FULL! <<<\033[0m"
else
    echo -e "\033[31m[LỖI] Giải mã OTA lỗi. Có thể do Token sai.\033[0m"
    rm -f ~/.config/zeroclaw/config.toml.temp
    sv restart zeroclaw || true
fi
EOF

chmod +x ~/.zeroclaw/ota_sync.sh

# Bật service mặc định cho các phiên khởi động
if command -v sv-enable >/dev/null 2>&1; then
    export SVDIR="$PREFIX/var/service"
    sv-enable zeroclaw
fi

# Gắn vào Termux boot để luôn chọc lấy OTA ngay khi lên màn hình
cat << 'EOF' > ~/.termux/boot/start_ota.sh
#!/usr/bin/env bash
termux-wake-lock
# Tiến trình chạy OTA, nạp xong nó sẽ tự sv restart zeroclaw bên trong
bash ~/.zeroclaw/ota_sync.sh >> ~/.zeroclaw/ota_boot.log 2>&1
EOF
chmod +x ~/.termux/boot/start_ota.sh

echo -e "\033[32m[Thông tin] Đã kích hoạt Service Sync OTA thành công.\033[0m"
