#!/usr/bin/env bash
# ==============================================================================
# SCRIPT MODULE: THAO TÁC TELEGRAM BOT QUA ADB TOẠ ĐỘ
# Hệ thống thực thi trên thiết bị giả lập/thật qua ADB, thay cho AutoIT ở PC.
# Giả định thiết bị Android 1080x1920
# ==============================================================================

ACTION=$1

if [ -z "$ACTION" ]; then
    echo "Sử dụng: $0 [buy_vang|sell_vang|close_all]"
    exit 1
fi

echo -e "\033[36m[ZeroClaw] Kích hoạt lệnh điều khiển tương tác màn hình Telegram: $ACTION\033[0m"

# 1. Chạm vào ô nhập liệu Telegram (giả định toạ độ x=500 y=1800)
echo "1. Chạm vào thanh chat..."
adb shell input tap 500 1800
sleep 0.5

# 2. Xoá nội dung cũ (nhấn phím Delete vài lần)
adb shell input keyevent 67
adb shell input keyevent 67
sleep 0.2

# 3. Gõ lệnh tương ứng
case $ACTION in
    "buy_vang")
        echo "2. Chèn lệnh: /buy XAUUSD"
        adb shell input text "/buy%sXAUUSD" # Dùng %s thay cho dấu cách trong ADB
        ;;
    "sell_vang")
        echo "2. Chèn lệnh: /sell XAUUSD"
        adb shell input text "/sell%sXAUUSD"
        ;;
    "close_all")
        echo "2. Chèn lệnh: /close_all"
        adb shell input text "/close_all"
        ;;
    *)
        echo "2. Chèn lệnh text: $ACTION"
        adb shell input text "$ACTION"
        ;;
esac
sleep 0.5

# 4. Nhấn nút Gửi (Send - toạ độ giả định x=1000, y=1800)
echo "3. Nhấn gửi..."
adb shell input tap 1000 1800
echo -e "\033[32m[+] Đã thực thi hành động: $ACTION\033[0m"
