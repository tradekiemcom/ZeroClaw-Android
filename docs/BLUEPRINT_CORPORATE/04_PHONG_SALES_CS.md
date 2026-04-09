# 🤝 Blueprint 04: Phòng Sales & CS (Chốt Đơn & Chăm Sóc Khách Hàng)

Phòng này là mặt tiền của công ty, nơi trực tiếp mang về doanh thu và duy trì lòng trung thành của khách hàng.

---

## 👨‍💼 1. Sales Manager (Leader)
- **Thiết bị**: Android TV Box.
- **Model**: `google/gemini-1.5-pro` (Khả năng hiểu ngữ cảnh và hội thoại dài rất tốt).
- **Nhiệm vụ**: Giám sát hiệu quả chốt đơn và điều phối các khiếu nại khó của khách hàng lên CEO.

---

## 👷 2. Đội Ngũ Nhân Sự (Agent Staff)

### 💰 Agent Sales Closer (Chốt đơn & Tư vấn)
- **Model**: `meta-llama/llama-3.3-70b-instruct`.
- **Nhiệm vụ**:
  - Trực tiếp chat với khách hàng trên Telegram/Web.
  - Tư vấn gói sản phẩm phù hợp (ví dụ gói tín hiệu Trading, gói học tập).
  - Xử lý các từ chối (Handling Objections) dựa trên bộ kịch bản chốt sale chuyên sâu (Sales Script).

### 💝 Agent Customer Support (Chăm sóc khách hàng)
- **Model**: `meta-llama/llama-3.1-70b`.
- **Nhiệm vụ**:
  - Hướng dẫn khách hàng sử dụng sản phẩm sau khi mua.
  - Giải đáp các thắc mắc kỹ thuật cơ bản.
  - Gửi các thông tin cập nhật, lời chúc cá nhân hóa (Personalized messages) để duy trì kết nối.

---

## 🛠️ 3. Luồng Giao Tiếp (Internal Flow)

1. **Khách hàng**: Nhắn tin hỏi về sản phẩm qua Telegram PA.
2. **Sales Closer**: Tiếp cận, gửi báo giá và chốt đơn.
3. **Sales Closer**: Báo cáo Finance Agent: "Có giao dịch mới 200$".
4. **Finance Agent**: Xác nhận tiền đã về ví.
5. **Support Agent**: Tự động gửi tài liệu hướng dẫn và link nhóm kín cho khách hàng.

---

## 🔗 Chuyển đến các phòng ban
- 👉 [Phòng R&D: Săn công nghệ & Dev Tool](05_PHONG_RD.md)
- 👉 [Hướng dẫn triển khai A-Z](99_HUONG_DAN_TRIEN_KHAI_FULL.md)
