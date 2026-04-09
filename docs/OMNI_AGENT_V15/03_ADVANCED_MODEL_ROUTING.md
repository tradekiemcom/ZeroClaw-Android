# 🧠 Master Manual 03: Kỹ Thuật Điều Hướng Model LLM (v15.0)

Một trong những sức mạnh lớn nhất của Omni-Agent là khả năng "Đổi Model" theo từng kỹ năng, ngay cả khi ZeroClaw chỉ chạy trên một lõi LLM duy nhất.

---

## 1. Tại Sao Cần Đổi Model?
- **Trading/Code**: Cần sự chính xác tuyệt đối (Claude 3.5 Sonnet / Llama 3 405B).
- **Marketing/Chat**: Cần nội dung sáng tạ và rẻ tiền (Gemini Flash / GPT-4o-mini).
- **Phân tích tin tức**: Cần Model có khả năng đọc Web cực tốt (Perplexity / Gemini 2.0).

---

## 2. Kỹ Thuật "Model Routing" Qua Tool-Script

Vì ZeroClaw chưa hỗ trợ Model-override trực tiếp trong cấu hình Tool, chúng ta sử dụng cơ chế **Bridge Script**.

### Mẫu: `marketing_skill.sh`
Thay vì để Agent tự suy nghĩ, chúng ta ép Agent gọi một script có sẵn Model riêng:
```bash
#!/bin/bash
# Script này ép buộc sử dụng Gemini Flash để viết bài cho rẻ và nhanh
CONTENT_INPUT=$1
curl https://openrouter.ai/api/v1/chat/completions \
  -H "Authorization: Bearer $OPENROUTER_KEY" \
  -d "{
    \"model\": \"google/gemini-2.0-flash-exp:free\",
    \"messages\": [{\"role\": \"user\", \"content\": \"Viết bài marketing cho nội dung: $CONTENT_INPUT\"}]
  }"
```

### Cách cấu hình trong `Omni-Config.toml`:
```toml
[tool.creative_writer]
command = "bash ~/.zeroclaw/skills/marketing_skill.sh {{topic}}"
description = "Sử dụng Model Gemini Flash để viết bài sáng tạo."
```

---

## 3. Lợi Ích Của Việc Điều Hướng Model
1. **Kiểm Soát Chi Phí**: Bạn không lãng phí Token của Claude 3.5 cho những việc vặt như tóm tắt báo.
2. **Hiệu Suất Tối Ưu**: Tác vụ nào dùng Model đó, giúp Omni-Agent không bao giờ bị "quá tải" hay nhầm lẫn logic.
3. **Tính Tự Trị**: Agent đóng vai trò là "Người điều phối" (Orchestrator), biết lúc nào nên dùng AI nào cho việc gì.
