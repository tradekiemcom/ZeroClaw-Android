#!/bin/bash
# ==============================================================================
# ZeroClaw-Android (Termux Native) Installer
# An architecture learned from openclaw-android to natively adopt
# zeroclaw-labs/zeroclaw for local Android environments.
# ==============================================================================

set -e
trap 'echo -e "\n\033[31m[ERROR] Installation failed at line $LINENO.\033[0m"; exit 1' ERR

# --- Environment Detection -----------------------------
if [ -z "$PREFIX" ] || [[ "$PREFIX" != *"/com.termux"* ]]; then
    echo -e "\033[31m[ERROR] This installer is strictly for Termux on Android.\033[0m"
    echo "Running it on standard Linux is unsupported."
    exit 1
fi

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_PATH="$PREFIX/bin"
SCRIPTS_DIR="$PROJECT_ROOT/scripts"

echo -e "\033[36m============================================================\033[0m"
echo -e "\033[1;36m           ZeroClaw-Android: A.I Company Edition            \033[0m"
echo -e "\033[36m============================================================\033[0m"

# Ensure executable permissions on all inner scripts
chmod +x "$SCRIPTS_DIR"/*.sh 2>/dev/null || true

# Execution Pipeline
echo -e "\n\033[32m[1/3] Kiểm tra Môi trường (RAM & Dependencies)...\033[0m"
bash "$SCRIPTS_DIR/check-env.sh"

echo -e "\n\033[32m[2/3] Cài đặt / Cập nhật ZeroClaw Native...\033[0m"
bash "$SCRIPTS_DIR/install-zeroclaw.sh"

echo -e "\n\033[32m[3/3] Trích xuất A.I Company Agents...\033[0m"
bash "$SCRIPTS_DIR/setup-company.sh"

echo -e "\n\033[1;32m✅ Hệ thống đã được thiết lập thành công!\033[0m"
echo -e "Hãy chạy lệnh: \033[1;33mcompany-mgr\033[0m để điều phối công ty AI của bạn."
