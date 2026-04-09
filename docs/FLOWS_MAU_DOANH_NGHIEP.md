# 📋 Thư Viện Flow Mẫu Cho Nhân Sự AI (ZeroClaw Workforce)

Tài liệu này cung cấp các "Công thức Cấu hình" (Prompts & Configs) để biến ZeroClaw thành các nhân viên chuyên trách.

---

## 📈 1. Flow: Trợ Lý Phân Tích Thị Trường (Trading Analyst)
Dành cho phòng Trading chuyên theo dõi Giá Vàng, BTC và Forex.

### A. Cấu hình .toml mẫu
```toml
[agent]
model = "google/gemini-2.0-flash" 
system_prompt = "Bạn là một chuyên gia phân tích thị trường tài chính cấp cao. Nhiệm vụ của bạn là theo dõi nến H4 và D1 để đưa ra các nhận định về Vàng (XAUUSD) và BTC."

[tools]
enabled = ["web_search", "rss_reader"]
rss_feeds = ["https://www.investing.com/rss/news.rss"]
```

### B. Mẫu lệnh vận hành
- **Check Signal**: `zeroclaw agent -m "Phân tích xu hướng Vàng hiện tại và cho tôi 3 mức hỗ trợ quan trọng."`
- **Cronjob Tin tức**: `zeroclaw cron add "0 9 * * *" "Quét tin tức Non-Farm hôm nay và tóm tắt rủi ro cho lệnh Sell Gold."`

---

## 🎞️ 2. Flow: Chuyên Viên Sáng Tạo Nội Dung (Content Creator)
Dành cho phòng Marketing chuyên sản xuất kịch bản TikTok và bài viết Fanpage.

### A. Cấu hình .toml mẫu
```toml
[agent]
model = "meta-llama/llama-3.3-70b-instruct"
system_prompt = "Bạn là Giám đốc Sáng tạo nội dung cho kênh TradeKiemCom. Phong cách của bạn là sắc sảo, thực tế và mang tính giáo dục tài chính cao."
```

### B. Mẫu lệnh vận hành
- **Lên Kịch bản**: `zeroclaw agent -m "Viết kịch bản TikTok 60s về tâm lý gồng lỗ của trader mới. Yêu cầu có 3 cảnh quay cụ thể."`
- **SEO Title**: `zeroclaw agent -m "Tạo 5 tiêu đề click-bait cho bài viết về: 'Tại sao 95% trader thua lỗ?'"`

---

## 🤝 3. Flow: Nhân Viên Tư Vấn & CSKH (Sales Agent)
Dành cho phòng Sale giúp tự động hóa trả lời Telegram.

### A. Cấu hình .toml mẫu
```toml
[agent]
model = "google/gemini-1.5-pro"
system_prompt = "Bạn là nhân viên tư vấn của iZ.Life. Hãy trả lời khách hàng một cách lịch sự, chuyên nghiệp. Luôn hướng khách hàng đăng ký dùng thử bản Pro."
```

### B. Tự động hóa Telegram
- Agent sẽ tự động phản hồi khách hàng khi bạn chạy `zeroclaw daemon`.

---

## 🛠️ Cách Áp Dụng
1. Copy nội dung cấu hình vào file `~/.config/zeroclaw/config.toml`.
2. Khởi động lại Gateway: `zeroclaw gateway`.
3. Bắt đầu giao việc qua màn hình CMD hoặc Telegram.
