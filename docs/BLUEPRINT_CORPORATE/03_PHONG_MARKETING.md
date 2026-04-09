# 📣 Blueprint 03: Phòng Marketing (Nhà Máy Sản Xuất Nội Dung)

Phòng Marketing chịu trách nhiệm xây dựng thương hiệu và thu hút khách hàng tiềm năng cho công ty qua tất cả các kênh mạng xã hội.

---

## 👨‍💼 1. Marketing Manager (Leader)
- **Thiết bị**: Android TV Box.
- **Model**: `anthropic/claude-3.5-sonnet` (OpenRouter) - Khả năng sáng tạo và ngôn ngữ mượt mà nhất.
- **Nhiệm vụ**: Lên kế hoạch chiến dịch và duyệt nội dung từ Staff trước khi đăng.

---

## 👷 2. Đội Ngũ Nhân Sự (Agent Staff)

### ✍️ Agent Content Creator (Sáng tạo nội dung)
- **Model**: `google/gemini-2.0-flash`.
- **Nhiệm vụ**:
  - Viết kịch bản video TikTok/Youtube Short.
  - Viết bài phân tích chuyên sâu cho Blog và Facebook.
  - Sáng tạo chuỗi Tweet (X threads) thu hút.
- **Skill**: Đọc hiểu insight người dùng từ Google Trends.

### 📱 Agent Social Media Manager (Quản lý kênh)
- **Model**: `meta-llama/llama-3.3-70b-instruct`.
- **Nhiệm vụ**:
  - Quản lý lịch đăng bài tự động trên: Web, Blog, FB, YT, TikTok, X.
  - Theo dõi chỉ số tương tác (Views, Likes, Comments) để báo cáo cho Manager.
  - Tự động hóa việc gắn link bio, link đăng ký.

---

## 🛠️ 3. Mẫu Kịch Bản Hoạt Động

1. **Manager**: "Tuần này chúng ta tập trung chủ đề: 'Bí mật quỹ FTMO'."
2. **Content Agent**: Viết 1 kịch bản video 60s và 3 bài post Facebook.
3. **Manager**: Chỉnh sửa và Duyệt.
4. **Social Agent**: Đặt lịch bài viết vào khung giờ vàng (8h tối).
5. **Social Agent**: Sau 24h, báo cáo: "Bài viết đạt 2,000 lượt xem và 50 lượt click."

---

## 🔗 Chuyển đến các phòng ban
- 👉 [Phòng Sale & CS: Chốt đơn & Chăm sóc](04_PHONG_SALES_CS.md)
- 👉 [Phòng R&D: Săn công nghệ & Dev Tool](05_PHONG_RD.md)
