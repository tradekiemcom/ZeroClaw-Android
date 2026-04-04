#!/bin/bash
set -e

# Đảm bảo môi trường chạy là Termux, chặn Linux thuần hoặc quyền Root (Sudo)
if [ -z "$PREFIX" ] || [[ "$PREFIX" != *"/com.termux"* ]]; then
    echo -e "\033[31m[LỖI] Kịch bản này chỉ dành cho môi trường Android/Termux.\033[0m"
    echo "Việc biên dịch từ source không chạy dưới quyền Sudo."
    exit 1
fi

TOTAL_RAM_KB=$(free | awk '/Mem:/ {print $2}')
TOTAL_RAM_GB=$(awk "BEGIN {print $TOTAL_RAM_KB/1024/1024}")

echo "[Thông tin] RAM Thiết bị khả dụng: ~${TOTAL_RAM_GB} GB"

if awk "BEGIN {exit !($TOTAL_RAM_GB < 4.0)}"; then
    echo -e "\033[33m[Cảnh Báo] Thiết bị có dưới 4GB RAM.\033[0m"
    echo "Tiến trình biên dịch 'rustc' có thể bị OOM (Out Of Memory) Killed."
    echo "Khuyến nghị thêm Swap (swapfile) trước khi cài đặt hoặc dùng máy cấu hình cao hơn."
fi
