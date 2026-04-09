# 🎒 Master Manual 02: Kho Kỹ Năng (The Skill Vault)

Omni-Agent sở hữu sức mạnh thông qua các **Kỹ năng (Skills)**. Mỗi kỹ năng là một bộ công cụ hoặc kiến thức chuyên sâu được nạp từ thư mục `~/.zeroclaw/skills/`.

---

## 💹 1. Kỹ Năng Giao Dịch (Trading Skillset)
- **Công cụ**: cTrader Open API, MT5 Web Gateway.
- **Vai trò**: Tự động chuyển đổi thành "Trade Leader" khi phát hiện yêu cầu về tài khoản hoặc phân tích kỹ thuật.
- **Tiến trình**:
  - Giao diện cTrader (`ctrader_bridge.js`).
  - Quản lý danh mục (`account_registry.json`).

---

## 📣 2. Kỹ Năng Sáng Tạo (Marketing Skillset)
- **Công cụ**: Web Search, Image Generator (nếu có), Viral Trend Scraper.
- **Vai trò**: Tự động chuyển đổi thành "Content Creator" khi Founder yêu cầu làm truyền thông.
- **Tiến trình**:
  - Quét trend -> Viết kịch bản -> Tối ưu hashtag -> Sẵn sàng đăng bài.

---

## 💻 3. Kỹ Năng Phát Triển (R&D / Coding Skillset)
- **Công cụ**: Terminal Access, Python Runner, GitHub API.
- **Vai trò**: Tự động chuyển đổi thành "AI Developer" khi phát hiện yêu cầu về lỗi code hoặc viết EA mới.
- **Tiến trình**:
  - Nghiên cứu mã nguồn -> Viết script -> Chạy Backtest -> Bàn giao.

---

## 🏦 4. Kỹ Năng Tài Chính & Quản Trị (Finance & Admin)
- **Công cụ**: Wallet Balancer, Log Analyzer.
- **Vai trò**: Quản lý ví doanh nghiệp và tối ưu hóa chi phí API.

---

## ⚡ Cách Sử Dụng Skills Hiệu Quả
Founder chỉ cần nói: 
- *"Hãy dùng kỹ năng Trading để xem giá vàng"*
- *"Hãy dùng kỹ năng Marketing viết bài này"*
- Hoặc đơn giản: *"Check giá vàng và làm bài post Tiktok"* (Omni-Agent sẽ tự nhận diện và dùng cả 2 kỹ năng cùng lúc).
