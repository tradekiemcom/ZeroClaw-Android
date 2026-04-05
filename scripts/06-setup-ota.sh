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
mkdir -p "$PREFIX/var/service/zeroclaw/log"
echo "#!/usr/bin/env bash" > "$PREFIX/var/service/zeroclaw/run"
echo "exec zeroclaw gateway 2>&1" >> "$PREFIX/var/service/zeroclaw/run"
chmod +x "$PREFIX/var/service/zeroclaw/run"

echo "#!/usr/bin/env bash" > "$PREFIX/var/service/zeroclaw/log/run"
echo "svlogd -tt ~/.zeroclaw/log" >> "$PREFIX/var/service/zeroclaw/log/run"
chmod +x "$PREFIX/var/service/zeroclaw/log/run"
mkdir -p ~/.zeroclaw/log

# Tạo script đồng bộ OTA
cat << 'EOF' > ~/.zeroclaw/ota_sync.sh
#!/usr/bin/env bash
# ============================================================================
# ZERO-TOUCH OTA SYNC
# Tải và giải mã cấu hình tập trung từ Sếp Trade Kiếm Cơm
# ============================================================================

OTA_URL="https://ota.tradekiem.com/v1/sync"
DEVICE_ID=$(getprop ro.serialno 2>/dev/null || echo "note10_boss")
PASSPHRASE_FILE="$HOME/.zeroclaw/.secret_pass"

# Nhập mật khẩu 1 lần duy nhất để lưu lại
if [ ! -f "$PASSPHRASE_FILE" ]; then
    echo -e "\033[1;33m[BẢO MẬT] Trạm chỉ huy yêu cầu Khóa Giải Mã Config:\033[0m"
    echo -e "(Nếu bỏ trống hoặc đợi 10s, hệ thống sẽ BỎ QUA tải cấu hình từ OTA và dùng cấu hình mặc định)"
    
    if ! read -t 10 -sp "Nhập Passphrase Sếp Trade Kiếm Cơm: " p; then
        echo ""
    else
        echo ""
    fi
    
    if [ -z "$p" ]; then
        echo -e "\033[33m[Thông Báo] Bỏ qua OTA Sync. Hệ thống tiếp tục chạy cấu hình nội bộ mặc định.\033[0m"
        sv restart zeroclaw || true
        exit 0
    else
        echo "$p" > "$PASSPHRASE_FILE"
    fi
fi

echo -e "\033[32mĐang đồng bộ cấu hình bảo mật từ OTA Worker ($OTA_URL)...\033[0m"
raw_data=$(curl -s "$OTA_URL?id=$DEVICE_ID&core=zeroclaw")
pass=$(cat "$PASSPHRASE_FILE")

# Bóc tách JSON
enc_toml=$(echo "$raw_data" | jq -r '.encrypted_toml')
status=$(echo "$raw_data" | jq -r '.ota_status')

if [ "$enc_toml" = "null" ] || [ -z "$enc_toml" ]; then
    echo -e "\033[31m[LỖI] Máy chủ OTA không trả về file mã hoá. Kiểm tra lại Mạng/Domain.\033[0m"
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
    echo -e "\033[31m[LỖI] Sai Passphrase! Giữ nguyên cấu hình gốc. Mật khẩu lưu trữ đã bị xoá.\033[0m"
    rm -f ~/.config/zeroclaw/config.toml.temp
    rm -f "$PASSPHRASE_FILE"
    sv restart zeroclaw || true
fi
EOF

chmod +x ~/.zeroclaw/ota_sync.sh

# Bật service mặc định cho các phiên khởi động
if command -v sv-enable >/dev/null 2>&1; then
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
