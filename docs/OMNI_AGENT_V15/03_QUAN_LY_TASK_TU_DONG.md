# ⏰ Master Manual 03: Quản Lý Task Tự Động (Cron)

Hệ thống ZeroClaw cho phép bạn lập lịch để Omni-Agent tự động thực hiện các Kỹ năng mà không cần bạn phải ra lệnh thủ công trên Dashboard.

---

## 📅 1. Thiết Lập Task Tự Động (GUI)

1. Đăng nhập vào Dashboard -> Chọn tab **"Tasks"** hoặc **"Cron"**.
2. Bấm **"Create New Task"**.
3. **Cấu hình**:
   - **Task Name**: Ví dụ: `daily_gold_report`.
   - **Command**: Lệnh hoặc yêu cầu (Ví dụ: `"Quét tin tức tài chính và báo cáo cho tôi qua Telegram"`).
   - **Schedule**: Theo cú pháp Cron (Ví dụ: `0 8 * * *` để chạy vào 8 giờ sáng mỗi ngày).
4. Bấm **"Enable"**.

---

## 🤖 2. Luồng Tự Động Hóa Thực Chiến

Hệ thống Task giúp xâu chuỗi các Skill bạn đã cài đặt:

- **6:00 AM**: Chạy Task "Quét tin tức thế giới" -> Agent dùng Skill **News Fetcher**.
- **8:00 AM**: Chạy Task "Phân tích giá Vàng" -> Agent dùng Skill **Analyst**.
- **9:00 AM**: Chạy Task "Gửi kế hoạch giao dịch" -> Agent dùng Skill **Planner** để tổng hợp 2 kết quả trên và gửi cho bạn qua Telegram.

---

## 🛡️ 3. Lưu Ý Về Hiệu Năng

- Đừng đặt các Task quá dày đặc (Ví dụ mỗi 1 phút). Thiết bị có thể xử lý tốt nhưng Model LLM sẽ tốn nhiều Token.
- Khuyên dùng: Khoảng cách giữa các Task nên từ **30 - 60 phút** để đảm bảo dữ liệu thị trường có sự thay đổi đáng kể.

---

*(Bây giờ, thiết bị của bạn đã trở thành một Robot tự hành 24/7 thực thụ)*
