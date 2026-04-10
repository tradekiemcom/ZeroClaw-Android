#!/bin/bash
# ==============================================================================
# ZeroClaw-Android Installer
# Native Compilation/Installation for Termux without sudo.
# ==============================================================================

set -e
trap 'echo -e "\n\033[31m[ERROR] Quá trình cài đặt thất bại tại luồng chính.\033[0m"; exit 1' ERR

# Đảm bảo PATH của Termux luôn được nạp
export PATH="$PREFIX/bin:$PATH"

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="$PROJECT_ROOT/scripts"

chmod +x "$SCRIPTS_DIR"/*.sh 2>/dev/null || true

# ==============================================================================
# QUY TRÌNH KIỂM TRA & DỌN DẸP HỆ THỐNG (Interactive Mode)
# ==============================================================================

while true; do
    bash "$SCRIPTS_DIR/01-sys-diagnostics.sh"
    
    echo -e "\n\033[36mBạn muốn thực hiện hành động nào?\033[0m"
    echo -e "  1. [DỌN DẸP] Quét rác, Giải phóng Port & RAM (Khuyên dùng)"
    echo -e "  2. [CÀI ĐẶT] Bắt đầu cài đặt ngay"
    echo -e "  3. [THOÁT] Dừng lại"
    read -p "Lựa chọn (1/2/3): " choice
    if [[ "$choice" == "1" ]]; then
        bash "$SCRIPTS_DIR/00-deep-clean.sh"
        echo -e "\n--- Đang đánh giá lại hệ thống sau dọn dẹp ---\n"
        sleep 2
        continue
    elif [[ "$choice" == "2" ]]; then
        break
    else
        echo -e "\033[33mHủy bỏ quá trình cài đặt.\033[0m"
        exit 0
    fi
done

# ==============================================================================
# BẮT ĐẦU CÀI ĐẶT CHÍNH THỨC v17.1
# ==============================================================================

echo -e "\n\033[32m[>>>] Đang khởi động tiến trình cài đặt ZeroClaw-Android v17.1...\033[0m\n"

echo -e "\n\033[32m[1/4] Cài đặt Dependencies (curl, openssl, adb...)\033[0m"
bash "$SCRIPTS_DIR/02-install-deps.sh"

echo -e "\n\033[32m[2/4] Tải và cấu hình lõi ZeroClaw (Native Optimization)...\033[0m"
bash "$SCRIPTS_DIR/03-install-binary.sh"

echo -e "\n\033[32m[3/4] Cài đặt trạm trung chuyển OTA, Service & Remote ADB...\033[0m"
bash "$SCRIPTS_DIR/06-setup-ota.sh"

echo -e "\n\033[32m[4/4] Kiểm tra chéo toàn bộ hệ thống...\033[0m"
bash "$SCRIPTS_DIR/99-verify-final.sh"

echo -e "\n\033[1;32m✅ CÀI ĐẶT HOÀN TẤT - TIẾN TỚI OMNI-AGENT v17.1!\033[0m"

# Tự động kích hoạt OTA Daemon v8.0 chạy ngầm
echo -e "\n\033[36m[6/6] Kích hoạt tiến trình Tự động Đồng bộ (OTA Daemon)...\033[0m"
nohup bash ~/.zeroclaw/ota_sync.sh > ~/.zeroclaw/ota_daemon.log 2>&1 &
echo -e "Dịch vụ đồng bộ đang chạy ngầm. Hãy chờ để kích hoạt cấu hình từ Server quản trị."
