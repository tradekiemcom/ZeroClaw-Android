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
    echo -e "  3. [NUCLEAR] Xóa sạch toàn bộ cấu hình cũ & Cài mới"
    echo -e "  4. [THOÁT] Dừng lại"
    read -p "Lựa chọn (1/2/3/4): " choice
    if [[ "$choice" == "1" ]]; then
        bash "$SCRIPTS_DIR/00-deep-clean.sh"
        echo -e "\n--- Đang đánh giá lại hệ thống sau dọn dẹp ---\n"
        sleep 2
        continue
    elif [[ "$choice" == "2" ]]; then
        break
    elif [[ "$choice" == "3" ]]; then
        echo -e "\n\033[31m[NUCLEAR] Đang tiến hành xóa sạch dấu vết cũ...\033[0m"
        rm -rf ~/.zeroclaw 2>/dev/null || true
        if command -v sv-disable >/dev/null 2>&1; then
            sv-disable zeroclaw 2>/dev/null || true
        fi
        rm -rf $PREFIX/var/service/zeroclaw 2>/dev/null || true
        echo -e "\033[32m[DONE] Đã dọn sạch. Bắt đầu cài mới.\033[0m"
        break
    else
        echo -e "\033[33mHủy bỏ quá trình cài đặt.\033[0m"
        exit 0
    fi
done

# ==============================================================================
# BẮT ĐẦU CÀI ĐẶT CHÍNH THỨC v17.8
# ==============================================================================

echo -e "\n\033[32m[>>>] Đang khởi động tiến trình cài đặt ZeroClaw-Android v17.8...\033[0m\n"

echo -e "\n\033[32m[1/4] Cài đặt Dependencies (curl, openssl, adb...)\033[0m"
bash "$SCRIPTS_DIR/02-install-deps.sh"

echo -e "\n\033[32m[2/4] Tải và cấu hình lõi ZeroClaw (Native Optimization)...\033[0m"
bash "$SCRIPTS_DIR/03-install-binary.sh"

# echo -e "\n\033[32m[3/4] Cài đặt trạm trung chuyển OTA, Service & Remote ADB...\033[0m"
# bash "$SCRIPTS_DIR/06-setup-ota.sh"
echo -e "\n\033[33m[3/4] Bỏ qua Module OTA (Bản Zero-OTA Clean v17.5)...\033[0m"

echo -e "\n\033[32m[4/4] Kiểm tra chéo toàn bộ hệ thống...\033[0m"
bash "$SCRIPTS_DIR/99-verify-final.sh"

echo -e "\n\033[1;32m✅ CÀI ĐẶT HOÀN TẤT - TIẾN TỚI OMNI-AGENT v17.8 (NATIVE LOCAL)!\033[0m"
# (Vô hiệu hóa toàn bộ nền tảng OTA để kiểm tra tình trạng Port)
echo -e "\n\033[36mHệ thống đã sẵn sàng ở chế độ Local. Hãy làm theo hướng dẫn 5 bước.\033[0m"
