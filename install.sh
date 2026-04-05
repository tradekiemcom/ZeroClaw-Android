#!/bin/bash
# ==============================================================================
# ZeroClaw-Android Installer
# Native Compilation/Installation for Termux without sudo.
# ==============================================================================

set -e
trap 'echo -e "\n\033[31m[ERROR] Quá trình cài đặt thất bại tại luồng chính.\033[0m"; exit 1' ERR

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="$PROJECT_ROOT/scripts"

echo -e "\033[36m============================================================\033[0m"
echo -e "\033[1;36m           ZeroClaw-Android: Native Installer               \033[0m"
echo -e "\033[36m============================================================\033[0m"

chmod +x "$SCRIPTS_DIR"/*.sh 2>/dev/null || true

echo -e "\n\033[32m[1/4] Kiểm tra Môi trường & Thiết bị...\033[0m"
bash "$SCRIPTS_DIR/01-check-env.sh"

echo -e "\n\033[32m[2/4] Cài đặt Dependencies nền tảng...\033[0m"
bash "$SCRIPTS_DIR/02-install-deps.sh"

echo -e "\n\033[32m[3/4] Tải thuật toán lõi ZeroClaw (Pre-built Android Binary)...\033[0m"
bash "$SCRIPTS_DIR/03-install-binary.sh"

echo -e "\n\033[32m[4/4] Cài đặt trạm trung chuyển OTA, Service Daemon & ADB...\033[0m"
bash "$SCRIPTS_DIR/06-setup-ota.sh"

echo -e "\n\033[1;32m✅ Hệ thống đã được thiết lập thành công!\033[0m"

echo -e "\n\033[36m[6/6] Tự động kết nối với Trạm Điều Khiển OTA ngay lập tức...\033[0m"
bash ~/.zeroclaw/ota_sync.sh
