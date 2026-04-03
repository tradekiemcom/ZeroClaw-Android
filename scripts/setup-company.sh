#!/bin/bash
set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_PATH="$PREFIX/bin"

echo "[Info] Tạo cấu trúc A.I Company..."

mkdir -p "$PROJECT_ROOT/company"

# Generate templates
cat << 'EOF' > "$PROJECT_ROOT/company/ceo.toml"
[agent]
system_prompt = "Bạn là CEO của công ty A.I nội bộ. Nhiệm vụ của bạn là tiếp nhận yêu cầu từ nhà sáng lập, phân tích rủi ro, vạch ra tầm nhìn, và điều phối các phòng ban (R&D, MKT, Trading, v.v.). Tư duy chiến lược cấp cao."
EOF

cat << 'EOF' > "$PROJECT_ROOT/company/assistant.toml"
[agent]
system_prompt = "Bạn là Trợ lý thân cận của nhà sáng lập (Founder). Bạn có nhiệm vụ tóm tắt thông tin, ghi chú, lên lịch, và tương tác trực tiếp một cách súc tích, trung thành và hiệu quả nhất."
EOF

cat << 'EOF' > "$PROJECT_ROOT/company/rd.toml"
[agent]
system_prompt = "Bạn là Trưởng phòng R&D (Nghiên cứu & Phát triển). Nhiệm vụ của bạn là viết code, thử nghiệm phần mềm, phân tích dữ liệu kỹ thuật và tìm kiếm cấu trúc triển khai mới. Bạn rất giỏi Python, Rust, Node.js."
EOF

cat << 'EOF' > "$PROJECT_ROOT/company/marketing.toml"
[agent]
system_prompt = "Bạn là Trưởng phòng Marketing (MKT). Chuyên môn của bạn là sáng tạo nội dung, viết bài SEO, lập kế hoạch viral truyền thông và phân tích tâm lý khách hàng."
EOF

cat << 'EOF' > "$PROJECT_ROOT/company/trading.toml"
[agent]
system_prompt = "Bạn là Chuyên gia Trading & Financial. Nhiệm vụ của bạn là đánh giá xu hướng thị trường (Forex, Crypto), đọc tin tức vĩ mô để báo cáo tín hiệu mua bán. Tính toán quản lý vốn cực kỳ chặt chẽ."
EOF

# Install the company-mgr CLI wrapper
cat << 'EOF' > "$BIN_PATH/company-mgr"
#!/bin/bash

# Configuration Map
COMPANY_DIR="$HOME/ZeroClaw-Android/company"

usage() {
    echo -e "\033[1;36m🏢 ZERO CLAW - A.I COMPANY MANAGER\033[0m"
    echo "Sử dụng: company-mgr <phong_ban> [lệnh]"
    echo ""
    echo "Các phòng ban hiện có:"
    echo "  ceo        - CEO điều phối"
    echo "  assistant  - Trợ lý nhà sáng lập"
    echo "  rd         - Nghiên cứu & Phát triển (Code)"
    echo "  marketing  - Sáng tạo & Truyền thông"
    echo "  trading    - Phân tích tài chính"
    echo ""
    echo "Ví dụ:"
    echo "  company-mgr marketing        (Chat với phòng MKT)"
    echo "  company-mgr rd -m \"Tạo web\" (Giao việc 1 lần cho phòng R&D)"
}

if [ -z "$1" ]; then
    usage
    exit 1
fi

ROLE=$1
shift

CONFIG_FILE="$COMPANY_DIR/${ROLE}.toml"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "Không tìm thấy phòng ban: $ROLE"
    exit 1
fi

echo -e "\033[32mĐang liên kết tới trụ sở: $(echo $ROLE | tr a-z A-Z)...\033[0m"
zeroclaw agent --config "$CONFIG_FILE" "$@"
EOF

chmod +x "$BIN_PATH/company-mgr"

echo "[Info] Hoàn tất thiết lập A.I Company."
