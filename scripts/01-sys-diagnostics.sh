#!/bin/bash
# ==============================================================================
# ZERO-CLAW SYSTEM DIAGNOSTICS
# Kiểm tra và đánh giá chi tiết thông số phần cứng thiết bị
# ==============================================================================

export PATH="$PREFIX/bin:$PATH"

# Định nghĩa màu sắc
BLUE='\033[36m'
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
NC='\033[0m'

echo -e "${BLUE}==================================================${NC}"
echo -e "${BLUE}          KIỂM TRA THÔNG SỐ HỆ THỐNG             ${NC}"
echo -e "${BLUE}==================================================${NC}"

# 1. Thông tin cơ bản
DEVICE_MODEL=$(getprop ro.product.model || echo "Unknown")
ANDROID_VER=$(getprop ro.build.version.release || echo "Unknown")
ARCH=$(uname -m)
KERNEL=$(uname -r)

# 2. CPU - Lấy thông tin số nhân
CPU_CORES=$(grep -c ^processor /proc/cpuinfo 2>/dev/null || echo "1")
# Thử lấy xung nhịp (thường tính bằng kHz)
CPU_FREQ_KHZ=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq 2>/dev/null || echo "0")
CPU_SPEED_GHZ=$(echo "scale=2; $CPU_FREQ_KHZ / 1000000" | bc 2>/dev/null || echo "Unknown")

# 3. RAM (Tổng và Trống)
RAM_TOTAL=$(free -m | grep Mem | awk '{print $2}')
RAM_FREE=$(free -m | grep Mem | awk '{print $4}')

# 4. Lưu trữ (Dùng cho thư mục /data là nơi Termux nằm)
STORAGE_INFO=$(df -h /data | tail -1)
STORAGE_TOTAL=$(echo $STORAGE_INFO | awk '{print $2}')
STORAGE_FREE=$(echo $STORAGE_INFO | awk '{print $4}')
STORAGE_FREE_MB=$(df -m /data | tail -1 | awk '{print $4}')

# HIỂN THỊ THÔNG TIN
echo -e "  - Thiết Bị: ${GREEN}$DEVICE_MODEL${NC}"
echo -e "  - Hệ điều hành: ${GREEN}Android $ANDROID_VER${NC}"
echo -e "  - Kiến Trúc: ${GREEN}$ARCH${NC}"
echo -e "  - Kernel: ${GREEN}$KERNEL${NC}"
echo -e "  - CPU: ${GREEN}$CPU_CORES Cores @ ${CPU_SPEED_GHZ}GHz${NC}"
echo -e "  - RAM: ${GREEN}${RAM_TOTAL}MB${NC} (Còn trống: ${YELLOW}${RAM_FREE}MB${NC})"
echo -e "  - Lưu trữ: ${GREEN}$STORAGE_TOTAL${NC} (Còn trống: ${YELLOW}$STORAGE_FREE${NC})"
echo -e "${BLUE}--------------------------------------------------${NC}"

# ĐÁNH GIÁ CẤU HÌNH ĐỀ NGHỊ
echo -e "${BLUE}Cấu hình đề xuất:${NC}"
echo -e "  - RAM trống: > 200MB"
echo -e "  - Lưu trữ trống: > 300MB"

# Logic đánh giá
STATUS="[ĐAT]"
NOTE=""

if [ "$RAM_FREE" -lt 150 ]; then
    STATUS="${RED}[CẢNH BÁO]${NC}"
    NOTE="RAM còn lại rất ít, dịch vụ có thể bị tắt bất ngờ bởi Android."
elif [ "$STORAGE_FREE_MB" -lt 300 ]; then
    STATUS="${RED}[KHÔNG ĐẠT]${NC}"
    NOTE="Không đủ dung lượng để cài đặt. Cần tối thiểu 300MB trống."
fi

echo -e "Đánh giá: $STATUS"
if [ -n "$NOTE" ]; then
    echo -e "Ghi chú: $NOTE"
fi
echo -e "${BLUE}==================================================${NC}"

# Trả về mã lỗi nếu không đạt yêu cầu tối thiểu
if [ "$STORAGE_FREE_MB" -lt 100 ]; then
    exit 1
fi
exit 0
