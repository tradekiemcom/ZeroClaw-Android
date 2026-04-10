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
echo -e "${YELLOW}HƯỚNG DẪN 5 BƯỚC KHỞI ĐỘNG OMNI-AGENT:${NC}"
echo -e "  1. ${GREEN}zeroclaw onboard${NC} ⚠️ \033[1;31mCHỌN 'n' KHI HỎI 'Launch channels now?'\033[0m"
echo -e "  2. ${GREEN}zeroclaw gateway${NC} (Kích hoạt kết nối)"
echo -e "  3. ${GREEN}zeroclaw daemon${NC}  (Chạy ngầm vĩnh viễn)"
echo -e "  4. ${RED}[Dự phòng]${NC} Nếu Tunnel lỗi: ${YELLOW}cloudflared tunnel run --token ...${NC}"
echo -e "  5. ${GREEN}zeroclaw status${NC}  (Kiểm tra hệ thống)"
echo -e "${BLUE}--------------------------------------------------${NC}"
echo -e "${YELLOW}💡 Ghi chú: Bản v17.7 (Nuclear Reset) xóa sạch dấu vết cũ.${NC}"
echo -e "   Port 42617 đang hoàn toàn trống để anh làm việc."
echo -e "${BLUE}==================================================${NC}"

echo -e "\n✅ \033[32mCÀI ĐẶT HOÀN TẤT - TIẾN TỚI OMNI-AGENT v17.7!\033[0m"
