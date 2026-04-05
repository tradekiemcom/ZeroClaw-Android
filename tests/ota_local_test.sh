#!/usr/bin/env bash
# ============================================================================
# ZERO-TOUCH OTA SYNC - LOCAL TESTING
# Tải và giải mã cấu hình từ Localhost (Genymotion Mac IP)
# ============================================================================

# Giả lập Genymotion trỏ về máy Mac qua IP 10.0.3.2 (hoặc 10.0.2.2 ở máy ảo chuẩn)
# Chạy local server trên cổng 8787 của máy tính.
OTA_URL="http://10.0.3.2:8787/v1/sync"
DEVICE_ID=$(getprop ro.serialno 2>/dev/null || echo "note10_boss")

echo -e "\033[33m[DEBUG] Đang thực thi tải OTA từ Local Development ($OTA_URL)...\033[0m"

raw_data=$(curl -s "$OTA_URL?id=$DEVICE_ID&core=zeroclaw-test")

# Bóc tách JSON
enc_toml=$(echo "$raw_data" | jq -r '.encrypted_toml' 2>/dev/null)

if [ "$enc_toml" = "null" ] || [ -z "$enc_toml" ]; then
    echo -e "\033[31m[LỖI] Server trả về phản hồi không hợp lệ hoặc không bắt được máy chủ tại 10.0.3.2:8787\033[0m"
    exit 1
fi

echo -e "\033[32m[+] Raw payload đã gọi về thành công. Kịch bản giải mã nội hạt sẽ được bỏ qua ở file Test này.\033[0m"
echo "Nội dung nhận:"
echo "$enc_toml"
