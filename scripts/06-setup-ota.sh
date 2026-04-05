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
mkdir -p ~/.termux/boot/

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
    read -sp "Nhập Passphrase Sếp Trade Kiếm Cơm: " p
    echo "$p" > "$PASSPHRASE_FILE"
    echo ""
fi

echo -e "\033[32mĐang đồng bộ cấu hình bảo mật từ OTA Worker ($OTA_URL)...\033[0m"
raw_data=$(curl -s "$OTA_URL?id=$DEVICE_ID&core=zeroclaw")
pass=$(cat "$PASSPHRASE_FILE")

# Bóc tách JSON
enc_toml=$(echo "$raw_data" | jq -r '.encrypted_toml')
status=$(echo "$raw_data" | jq -r '.ota_status')

if [ "$enc_toml" = "null" ] || [ -z "$enc_toml" ]; then
    echo -e "\033[31m[LỖI] Máy chủ OTA không trả về file mã hoá. Kiểm tra lại Mạng/Domain.\033[0m"
    exit 1
fi

# Thực thi giải mã AES-256-CBC theo chuẩn OpenSSL PBKDF2
echo "$enc_toml" | openssl enc -d -aes-256-cbc -a -pbkdf2 -pass pass:"$pass" > ~/.config/zeroclaw/config.toml 2>/dev/null

if [ $? -eq 0 ]; then
    echo -e "\033[1;32m✅ Giải mã Config thành công!\033[0m"
    
    # Thực thi các lệnh điều khiển thiết bị từ xa (Hot Scripts)
    echo -e "Thực thi Hot Scripts..."
    echo "$raw_data" | jq -r '.hot_scripts[]?' | while read cmd; do 
        echo -e "\033[36m > RUN: $cmd \033[0m"
        eval "$cmd"
    done
    
    # Khởi động lại hệ thống sạch
    pkill -f "zeroclaw daemon" || true
    if command -v termux-wake-lock >/dev/null 2>&1; then termux-wake-lock; fi
    if command -v adb >/dev/null 2>&1; then 
        echo "Kết nối ADB nội hạt..."
        adb connect localhost:5555 || true
    fi
    
    echo -e "\033[1;33mĐang kích hoạt lại Daemon ngầm...\033[0m"
    zeroclaw daemon &
    
    echo -e "\033[1;32m>>> HỆ THỐNG ĐÃ SẴN SÀNG. BOSS CÓ QUYỀN FULL! <<<\033[0m"
else
    echo -e "\033[31m[LỖI] Sai Passphrase hoặc nội dung cấu hình bị hỏng. Cập nhật thất bại!\033[0m"
    rm -f ~/.config/zeroclaw/config.toml
fi
EOF

chmod +x ~/.zeroclaw/ota_sync.sh

# Gắn vào Termux boot (Khởi động cùng thiết bị)
cat << 'EOF' > ~/.termux/boot/start_ota.sh
#!/usr/bin/env bash
# Termux Wake Lock để chống ngủ gật màn hình
termux-wake-lock
# Gọi tiến trình OTA Check
bash ~/.zeroclaw/ota_sync.sh >> ~/.zeroclaw/ota_boot.log 2>&1
EOF
chmod +x ~/.termux/boot/start_ota.sh

echo -e "\033[32m[Thông tin] Đã kích hoạt Service Sync OTA thành công.\033[0m"
