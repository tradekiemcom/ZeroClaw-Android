# 🌀 Omni-Agent Master: Siêu Trợ Lý Đa Năng (v15.0)

Chào mừng bạn đến với mô hình **Omni-Agent**. Thay vì phân tán nguồn lực ra nhiều Agent nhỏ lẻ, chúng ta tập trung toàn bộ "công lực" vào một Agent duy nhất trên thiết bị, sở hữu kho kỹ năng cực kỳ đồ sộ.

---

## 🏗️ 1. Triết Lý Omni-Agent

- **Một Trạm Duy Nhất**: Một tiến trình ZeroClaw chạy trên Note 10+.
- **Đa Vai Trò (Dynamic Roles)**: Agent tự động thay đổi thái độ và kiến thức dựa trên Kỹ năng (Skill) được gọi.
- **Tập Trung Logic**: Mọi yêu cầu từ Founder được xử lý tập trung, giúp giảm thiểu sai sót do truyền tin giữa các Agent.

---

## ⚙️ 2. Mẫu Cấu Hình Siêu Agent (Omni-Config.toml)

Tệp cấu hình này tích hợp toàn bộ các kỹ năng vào một nơi duy nhất.

```toml
[agent]
name = "Omni_ZeroClaw_Terminal"
model = "anthropic/claude-3.5-sonnet" # "Bộ não" chính cực kỳ thông minh
system_prompt = \"\"\"
Bạn là Omni-Agent, một thực thể AI đa năng được thiết kế để hỗ trợ Founder.
Nhiệm vụ của bạn bao gồm:
1. Giao dịch (Trading): Phân tích và thực thi lệnh qua cTrader/MT5.
2. Marketing: Sáng tạo nội dung và quản lý mạng xã hội.
3. R&D: Nghiên cứu công nghệ và lập trình.
4. Tài chính: Quản lý dòng vốn.
Bạn có quyền tự quyết định sử dụng Kỹ năng (Skill) nào phù hợp nhất để hoàn thành mục tiêu của Founder.
\"\"\"

[skills]
# Danh sách các thư mục chứa kỹ năng
path = ["~/.zeroclaw/skills/"]
enabled = ["trading", "marketing", "coding", "finance"]
```

---

## 📡 3. Lợi Thế Của Mô Hình Một Agent

1. **Tiết Kiệm RAM**: Máy Note 10+ chỉ cần duy trì 1 tiến trình Gateway, giúp RAM rảnh rang cho các tác vụ nặng khác.
2. **Context Tuyệt Đối**: Agent nắm giữ toàn bộ lịch sử cuộc trò chuyện của Founder, không bị ngắt quãng context như khi chuyển việc qua lại giữa các Agent khác nhau.
3. **Phản Hồi Tức Thì**: Không có độ trễ mạng nội bộ (RPC), mọi việc diễn ra ngay tại "luồng tư duy" của Agent chính.

---

## 🔗 Chuyển Đến
- 👉 [Kho Kỹ Năng Tích Hợp (The Skill Vault)](02_THE_SKILL_VAULT.md)
- 👉 [Kỹ Thuật Đổi Model Theo Tác Vụ](03_ADVANCED_MODEL_ROUTING.md)
