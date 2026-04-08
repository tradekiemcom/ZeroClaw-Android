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
    
    echo -e "\nBạn có muốn thực hiện [Dọn dẹp & Giải phóng dung lượng] không? (y/n)"
    read -p "Lựa chọn của bạn (Mặc định n): " choice
    if [[ "$choice" == "y" || "$choice" == "Y" ]]; then
        bash "$SCRIPTS_DIR/00-deep-clean.sh"
        echo -e "\n--- Đang đánh giá lại hệ thống sau dọn dẹp ---\n"
        continue
    fi
    break
done

echo -e "\nBạn có muốn bắt đầu tiến trình cài đặt ZeroClaw ngay bây giờ không? (y/n)"
read -p "Xác nhận cài định (y/n): " confirm
if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
    echo -e "\033[33mHủy bỏ quá trình cài đặt theo yêu cầu.\033[0m"
    exit 0
fi

# Nhập Token bảo mật OTA nếu có
echo -e "\n\033[36m>>> Cấu hình Token Bảo Mật OTA <<<\033[0m"
echo -e "Nếu anh đã cài ENCRYPTION_KEY trên Worker (ví dụ: TradeKiemCom123@!), hãy nhập vào đây."
echo -e "Nếu bỏ trống, máy sẽ tự sinh mã ngẫu nhiên (Zero-Touch)."
read -p "Nhập mã bảo mật (Để trống nếu không rõ): " OTA_TOKEN
export OTA_TOKEN

# ==============================================================================
# BẮT ĐẦU CÀI ĐẶT CHÍNH THỨC
# ==============================================================================

echo -e "\n\033[32m[>>>] Đang khởi động tiến trình cài đặt ZeroClaw-Android...\033[0m\n"

echo -e "\n\033[32m[1/4] Cấu hình môi trường bảo mật...\033[0m"
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
