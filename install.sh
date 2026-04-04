#!/bin/bash
# ==============================================================================
# ZeroClaw-Android (A.I Company Edition) Installer
# Native Compilation for Termux without sudo.
# ==============================================================================

set -e
trap 'echo -e "\n\033[31m[ERROR] Quá trình cài đặt thất bại tại luồng chính.\033[0m"; exit 1' ERR

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="$PROJECT_ROOT/scripts"

echo -e "\033[36m============================================================\033[0m"
echo -e "\033[1;36m           ZeroClaw-Android: A.I Company Edition            \033[0m"
echo -e "\033[1;36m    Tự động biên dịch mã nguồn (Native Cargo Build)         \033[0m"
echo -e "\033[36m============================================================\033[0m"

chmod +x "$SCRIPTS_DIR"/*.sh 2>/dev/null || true

echo -e "\n\033[32m[1/5] Kiểm tra Môi trường & Thiết bị...\033[0m"
bash "$SCRIPTS_DIR/01-check-env.sh"

echo -e "\n\033[32m[2/5] Cài đặt Dependencies nền tảng (Rust, Clang, Binutils)...\033[0m"
bash "$SCRIPTS_DIR/02-install-deps.sh"

echo -e "\n\033[32m[3/5] Tải thuật toán lõi ZeroClaw (Pre-built Android Binary)...\033[0m"
bash "$SCRIPTS_DIR/03-install-binary.sh"

echo -e "\n\033[32m[4/5] Trích xuất A.I Company...\033[0m"
bash "$SCRIPTS_DIR/04-setup-company.sh"

echo -e "\n\033[32m[5/5] Cài đặt mạng lưới kết nối Zero Trust...\033[0m"
bash "$SCRIPTS_DIR/05-setup-tunnel.sh"

echo -e "\n\033[1;32m✅ Hệ thống đã được thiết lập thành công!\033[0m"
echo -e "Hãy chạy lệnh: \033[1;33mcompany-mgr\033[0m để điều hành tập đoàn AI của bạn."
