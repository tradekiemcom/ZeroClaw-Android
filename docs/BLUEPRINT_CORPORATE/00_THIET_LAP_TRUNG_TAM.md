# 🏛️ Blueprint 00: Thiết Lập Trung Tâm Điều Hành (The Core)

Đây là tầng quan trọng nhất của công ty "A.I Do Anything". Tầng này chịu trách nhiệm giao tiếp với Founder (Sếp) và điều phối toàn bộ các phòng ban bên dưới.

---

## 🏗️ 1. Cấu Trúc Phân Cấp (Core Hierarchy)

### 🤵 Agent PA (Personal Assistant) - Người Gác Cổng
- **Vai trò**: Đại diện duy nhất giao tiếp với Founder qua Telegram Bot.
- **Thiết bị**: Note 10+.
- **Model**: `meta-llama/llama-3.3-70b-instruct` (NVIDIA NIM) - Tốc độ cao, suy luận tốt.
- **Nhiệm vụ**:
  - Tiếp nhận mệnh lệnh từ Sếp.
  - Tóm tắt báo cáo từ CEO gửi lên.
  - Nhắc nhở lịch trình và quản lý quyền truy cập.

### 👑 Agent CEO (Chief Executive Officer) - Tổng Tư Lệnh
- **Vai trò**: Bộ não điều hành, hiểu sâu về mục tiêu công ty.
- **Thiết bị**: Note 10+ (Chạy trên cổng API khác hoặc Daemon riêng).
- **Model**: `anthropic/claude-3.5-sonnet` (OpenRouter) - Độ thông minh và khả năng lập kế hoạch cao nhất.
- **Nhiệm vụ**:
  - Nhận lệnh từ PA, phân tích và chia việc cho các Trưởng phòng (Leaders).
  - Giám sát tiến độ của các Agent Staff.
  - Tổng hợp dữ liệu đa nguồn thành báo cáo chiến lược.

---

## 📡 2. Cơ Chế Kết Nối (Internal RPC)

Không dùng Telegram để các Agent nói chuyện với nhau nhằm đảm bảo tính bảo mật và độc lập.

- **Cổng liên lạc**: Mặc định sử dụng cổng `42617`.
- **Cấu hình PA Agent**: PA sẽ coi CEO Agent là một "Công cụ" (Tool).
  - URL Tool: `http://127.0.0.1:42618/v1/chat` (Nếu CEO chạy trên cùng máy).
  - Hoặc: `https://ceo.tradekiem.com/v1/chat` (Nếu qua Cloudflare Tunnel).

---

## 🛠️ 3. Mẫu Cấu Hình CEO Agent (CEO_Config.toml)

```toml
[agent]
name = "CEO_AI_Do_Anything"
model = "anthropic/claude-3.5-sonnet"
system_prompt = \"\"\"
Bạn là CEO của công ty 'A.I Do Anything'. 
Dưới quyền bạn có 5 phòng ban: Trading, Finance, Marketing, Sales, R&D.
Nhiệm vụ: Bạn không trực tiếp làm việc chi tiết, bạn điều phối các Agent Leader.
Khi nhận lệnh từ PA, bạn phải xác định xem phòng ban nào chịu trách nhiệm và gọi công cụ tương ứng.
\"\"\"

[tools]
# CEO có danh sách các 'Công cụ' chính là API của các Trưởng phòng
enabled = ["marketing_leader", "trading_leader", "finance_leader"]

[tool.marketing_leader]
url = "http://box-mkt.local:42617/v1/chat"
description = "Gọi phòng Marketing để yêu cầu viết bài, lên kịch bản hoặc quản lý mạng xã hội."
```

---

## 🔗 Chuyển đến các phòng ban
- 👉 [Phòng Trading: Trợ lý giao dịch & Quỹ](01_PHONG_TRADING.md)
- 👉 [Phòng Tài chính: Quản lý ví & Dòng tiền](02_PHONG_TAI_CHINH.md)
