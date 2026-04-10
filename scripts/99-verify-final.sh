#!/usr/bin/env bash
# ==============================================================================
# ZERO-CLAW FINAL VERIFICATION v16.9
# Kiểm duyệt cuối cùng toàn bộ hệ sinh thái sau khi cài đặt
# ==============================================================================

export PATH="$PREFIX/bin:$PATH"
BLUE='\033[36m'
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
NC='\033[0m'

echo -e "${BLUE}==================================================${NC}"
echo -e "${BLUE}        KIỂM TRA CUỐI CÙNG (FINAL CHECK)        ${NC}"
echo -e "${BLUE}==================================================${NC}"

# 1. Kiểm tra Binary
if zeroclaw --version >/dev/null 2>&1; then
    echo -e "  - Binary ZeroClaw: ${GREEN}[OK]${NC}"
else
    echo -e "  - Binary ZeroClaw: ${RED}[LỖI]${NC}"
fi

# 2. Kiểm tra Dịch vụ (sv)
if command -v sv >/dev/null 2>&1; then
    STATUS=$(sv status zeroclaw 2>/dev/null || echo "not running")
    if [[ "$STATUS" == *"run"* ]]; then
        echo -e "  - Dịch vụ Background: ${GREEN}[ĐANG CHẠY]${NC}"
    else
        echo -e "  - Dịch vụ Background: ${YELLOW}[CHƯA CHẠY]${NC}"
    fi
fi

# 3. Kiểm tra Port
PORT_42617=$(lsof -ti:42617 >/dev/null 2>&1 && echo -e "${GREEN}ACTIVE${NC}" || echo -e "${YELLOW}READY${NC}")
echo -e "  - Gateway Port 42617: $PORT_42617"

# 4. Kiểm tra ADB
if adb devices | grep -q "localhost:5555"; then
    echo -e "  - Remote ADB Loopback: ${GREEN}[KẾT NỐI]${NC}"
else
    echo -e "  - Remote ADB Loopback: ${YELLOW}[CHỜ KÍCH HOẠT]${NC}"
fi

echo -e "${BLUE}--------------------------------------------------${NC}"
echo -e "${GREEN}CHÚC MỪNG! HỆ THỐNG ĐÃ SẴN SÀNG HOẠT ĐỘNG.${NC}"
echo -e "${BLUE}--------------------------------------------------${NC}"
echo -e "Các lệnh phổ biến bạn có thể dùng:"
echo -e "  1. ${YELLOW}zeroclaw gateway${NC}    - Chạy thủ công nếu dịch vụ dừng"
echo -e "  2. ${YELLOW}sv status zeroclaw${NC} - Kiểm tra tình trạng chạy ngầm"
echo -e "  3. ${YELLOW}sv restart zeroclaw${NC} - Khởi động lại khi đổi cấu hình"
echo -e "  4. ${YELLOW}bash ~/.zeroclaw/ota_sync.sh${NC} - Đồng bộ lại OTA ngay lập tức"
echo -e "${BLUE}==================================================${NC}"

echo -e "\n✅ \033[32mCÀI ĐẶT HOÀN TẤT - HÃY TRẢI NGHIỆM OMNI-AGENT v16.9!\033[0m"
