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

# 2. Pin (Dùng termux-api nếu có)
if command -v termux-battery-status >/dev/null 2>&1; then
    BAT_JSON=$(termux-battery-status)
    BAT_PCT=$(echo "$BAT_JSON" | jq -r '.percentage')
    BAT_STAT=$(echo "$BAT_JSON" | jq -r '.status')
else
    # Fallback nếu chưa cài API app
    BAT_PCT=$(cat /sys/class/power_supply/battery/capacity 2>/dev/null || echo "N/A")
    BAT_STAT=$(cat /sys/class/power_supply/battery/status 2>/dev/null || echo "Unknown")
fi

# 3. CPU - Lấy thông tin số nhân và xung nhịp
CPU_CORES=$(grep -c ^processor /proc/cpuinfo 2>/dev/null || echo "1")
CPU_FREQ_KHZ=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq 2>/dev/null || echo "0")
CPU_SPEED_GHZ=$(echo "scale=2; $CPU_FREQ_KHZ / 1000000" | bc 2>/dev/null || echo "N/A")

# 4. RAM (Tổng và Trống)
RAM_TOTAL=$(free -m | grep Mem | awk '{print $2}')
RAM_FREE=$(free -m | grep Mem | awk '{print $4}')

# 5. Lưu trữ (Dùng cho thư mục /data)
STORAGE_INFO=$(df -h /data | tail -1)
STORAGE_TOTAL=$(echo $STORAGE_INFO | awk '{print $2}')
STORAGE_FREE=$(echo $STORAGE_INFO | awk '{print $4}')
STORAGE_FREE_MB=$(df -m /data | tail -1 | awk '{print $4}')

# HIỂN THỊ THÔNG TIN DASHBOARD v7.3
echo -e "  - Thiết Bị: ${GREEN}$DEVICE_MODEL${NC} (Android $ANDROID_VER)"
echo -e "  - Kiến Trúc: ${GREEN}$ARCH${NC} (Kernel: $KERNEL)"
echo -e "  - CPU: ${GREEN}$CPU_CORES Cores @ ${CPU_SPEED_GHZ}GHz${NC}"
echo -e "  - Trạng thái Pin: ${GREEN}$BAT_PCT%${NC} (${YELLOW}$BAT_STAT${NC})"
echo -e "  - RAM: ${GREEN}${RAM_TOTAL}MB${NC} (Còn trống: ${YELLOW}${RAM_FREE}MB${NC})"
echo -e "  - Lưu trữ: ${GREEN}$STORAGE_TOTAL${NC} (Còn trống: ${YELLOW}$STORAGE_FREE${NC})"
echo -e "${BLUE}--------------------------------------------------${NC}"

# 6. Radar Quét Port Conflict
echo -e "${YELLOW}[Radar] Quét xung đột cổng kết nối:${NC}"
for PORT in 42617 5555 8080 22; do
    OCCUPANT=$(lsof -ti:$PORT 2>/dev/null | head -n 1)
    if [ -n "$OCCUPANT" ]; then
        PROC_NAME=$(ps -p "$OCCUPANT" -o comm= 2>/dev/null || echo "Unknown")
        echo -e "  - Port $PORT : ${RED}BỊ CHIẾM${NC} bởi [$PROC_NAME] (PID: $OCCUPANT)"
    else
        echo -e "  - Port $PORT : ${GREEN}SẴN SÀNG${NC}"
    fi
done
echo -e "${BLUE}--------------------------------------------------${NC}"

# Logic đánh giá
STATUS="${GREEN}[SẴN SÀNG]${NC}"
NOTE=""

if [ "$RAM_FREE" -lt 150 ]; then
    STATUS="${YELLOW}[CẦN TỐI ƯU]${NC}"
    NOTE="RAM trống thấp (<150MB). Hãy chạy 'Dọn dẹp sâu' trước khi cài."
elif [ "$STORAGE_FREE_MB" -lt 500 ]; then
    STATUS="${RED}[CẢNH BÁO]${NC}"
    NOTE="Dung lượng trống thấp (<500MB). Cần cân nhắc giải phóng bộ nhớ."
fi

# Cảnh báo Pin yếu
if [ "$BAT_PCT" != "N/A" ] && [ "$BAT_PCT" -lt 20 ] && [ "$BAT_STAT" != "Charging" ]; then
    STATUS="${RED}[PIN YẾU]${NC}"
    NOTE="Pin dưới 20% và không sạc. Vui lòng cắm sạc để tránh lỗi khi đang cài."
fi

echo -e "Kết luận: $STATUS"
[ -n "$NOTE" ] && echo -e "Ghi chú: $NOTE"
echo -e "${BLUE}==================================================${NC}"

[ "$STORAGE_FREE_MB" -lt 100 ] && exit 1 || exit 0
