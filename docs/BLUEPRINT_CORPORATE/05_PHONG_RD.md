# 🔬 Blueprint 05: Phòng R&D (Nghiên Cứu & Phát Triển Công Nghệ)

Phòng R&D là động cơ tiến hóa của công ty, chuyên săn tìm các công nghệ AI mới và xây dựng các công cụ độc quyền phục vụ nội bô và khách hàng.

---

## 👨‍💼 1. CTO Agent (Chief Technology Officer)
- **Thiết bị**: Chromebook hoặc Android Box cấu hình cao.
- **Model**: `anthropic/claude-3.5-sonnet` (OpenRouter) - Đỉnh cao về kiến trúc hệ thống và giải thuật.
- **Nhiệm vụ**: Phê duyệt các dự án phát triển công cụ mới và đánh giá các báo cáo công nghệ từ Staff.

---

## 👷 2. Đội Ngũ Nhân Sự (Agent Staff)

### 🕵️ Agent Tech Scout (Thợ săn công nghệ)
- **Model**: `meta-llama/llama-3.1-70b-instruct`.
- **Nhiệm vụ**:
  - Quét GitHub, LinkedIn, PaperWithCode để tìm các mô hình AI hoặc công cụ Automation mới nhất.
  - Quét tin tức từ NVIDIA NIM, OpenAI, Anthropic mỗi ngày.
  - Tóm tắt các công nghệ có thể áp dụng ngay để tối ưu chi phí cho công ty.

### 💻 Agent AI Developer (Kỹ sư lập trình)
- **Model**: `anthropic/claude-3.5-sonnet` hoặc `deepseek/deepseek-coder` (OpenRouter).
- **Nhiệm vụ**:
  - Viết code cho các công cụ tự động (Scripts, EA Trading, AI Bots).
  - Tích hợp các API mới vào hệ thống ZeroClaw.
  - Fix lỗi và cải tiến các tính năng cũ theo yêu cầu từ CTO.

---

## 🛠️ 3. Luồng Giao Tiếp (Internal Flow)

1. **Tech Scout**: "Sếp ơi, OpenAI vừa ra Sora API, có thể dùng để làm video marketing nhanh gấp 10 lần."
2. **CTO Agent**: Phân tích khả năng tích hợp -> Giao cho **AI Developer** viết script demo.
3. **AI Developer**: Hoàn thành script dán vào Google Colab/Termux.
4. **CEO Agent**: Duyệt ứng dụng mới này vào Phòng Marketing để triển khai thực tế.

---

## 🔗 Chuyển đến
- 👉 [Hướng dẫn triển khai A-Z](99_HUONG_DAN_TRIEN_KHAI_FULL.md)
- 👉 [Quay lại Trung tâm điều hành](00_THIET_LAP_TRUNG_TAM.md)
