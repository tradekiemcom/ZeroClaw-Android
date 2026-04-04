# 🐾 ZeroClaw-Android: Cẩm Nang Vận Hành A.I Company

Tài liệu này hướng dẫn cách thức triển khai và vận hành hệ thống **ZeroClaw-Android**. Phiên bản này đã được đại tu toàn diện để có thể được biên dịch thẳng từ mã nguồn gốc bằng trình dịch Rust, chạy nguyên bản hoàn toàn dưới Termux của người dùng mà không cần thiết bị đã Root. Đồng thời nó được chia kịch bản tạo hình thành 18 Agents (Đặc Vụ) để phục vụ duy nhất một doanh nghiệp.

---

## 🌟 1. Cơ Cấu Tổ Chức & Phương Pháp Trọng Tâm
Mục tiêu cốt lõi của toàn hệ thống là **Tạo ra lợi nhuận và Hỗ trợ Tăng trưởng**.

### 👑 Ban Lãnh Đạo & Điều Phối
1. **Anh Hưng (Chủ tịch HĐQT/Founder)**: Người ra quyết định tối cao.
2. **Thảo Agent (Trợ Lý Founder)**: `company-mgr management/assistant` - Phân tích mệnh lệnh của anh Hưng, dịch ra ngôn ngữ hệ thống phân bổ KPI cho CEO, đồng thời thực hiện báo cáo vắn tắt lên Founder.
3. **CEO Agent**: `company-mgr management/ceo` - Giám đốc điều hành. Đưa ra chiến lược thực thi và lập lịch để các phòng ban cùng tuân thủ.
4. **CPO Agent**: Giám đốc Vận hành và 2 Agent Tối ưu/Quản lý luồng công việc (Optimize, Workflow).

### ⚙️ Các Trụ Cột Doanh Thu
Theo sau Ban Lãnh đạo là 4 khối chịu trách nhiệm tạo sản phẩm, bán sản phẩm và kiếm tiền:

- 🔬 **Phòng R&D**: Đứng đầu là **CTO Agent**. Có 3 lính: *Coder* (Giết lỗi, lập trình), *Research* (Phân tích đối thủ), *Data Analytics* (Insight).
- 💰 **Phòng Tài chính & Trading (Sinh Lời Nhanh)**: Đứng đầu là **CFO Agent**. Quản lý các cỗ máy kiếm tiền *TradeFx* (Ngoại hối), *TradeFund* (Quỹ ETF), *TradeGold* (XAUUSD) và *Finance Analyst* (Kế Toán).
- 📣 **Phòng Marketing (Tạo Phễu Dòng)**: Đứng đầu là **CMO Agent**. Quản lý 2 nhánh *Content Agent* (Viết bài/Tạo Video) và *Media Agent* (Tối ưu điểm chạm CPC quảng cáo).
- 🤝 **Phòng Sales & CS (Chốt Đơn)**: Đứng đầu là **CSO Agent**. Quản lý *Sale Agent* (Thuyết phục/Vượt qua sự từ chối) và *CS Agent* (Retain Data/Chăm sóc vòng đời KH).

---

## 🎒 2. Khâu Chuẩn Bị Tối Quan Trọng

### 2.1 Yêu cầu thiết bị
- **Phải có RAM tối thiểu 4GB - 6GB**: Vì quá trình cài đặt yêu cầu Termux chạy trình biên dịch `cargo build`, nếu điện thoại có RAM thấp, tiến trình sẽ bị hệ điều hành Android ngắt ngầm (OOM Kill).
- Cài đặt **Termux** từ kho ứng dụng bảo mật `F-Droid` (KHÔNG CÀI TRÊN CH PLAY).

### 2.2 Các tài nguyên cần thiết
- API Key (OpenRouter, Gemini, OpenAI v.v.)
- Một cái đầu lạnh để giao nhiệm vụ.

---

## 🛠 3. Quá Trình Cài Đặt (Build From Source)

**Bước 1:** Bật Termux và cấp quyền lưu trữ qua lệnh:
```bash
termux-setup-storage
```

**Bước 2:** Clone dự án kèm theo core ẩn của zeroclaw.
```bash
pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
git submodule update --init --recursive
```

**Bước 3:** Chạy cài đặt tự động cực khủng (Tuyệt đối KHÔNG gõ Sudo).
```bash
chmod +x install.sh
./install.sh
```

**Lưu ý khi thiết bị đang biên dịch (cargo run):** Máy điện thoại sẽ nóng lên. Đây là chuyện thường vì vi xử lý đang làm công việc của một Server PC biên dịch mã nguồn C/C++/Rust. Việc này chỉ xảy ra **1 LẦN DUY NHẤT**.

---

## 👔 4. Cách Giao Việc Cho Đặc Vụ

Sau cài đặt xong, bạn thao tác thông qua lệnh `company-mgr <đường_dẫn_phòng_ban>`

### Ví dụ: Gọi Thảo Agent
Là Founder, anh Hưng có thể gọi Thảo:
```bash
company-mgr management/assistant -m "Bảo CEO lên kế hoạch phát triển tool Trade Quỹ ETF mới gấp"
```

### Ví dụ: Hỏi giá Vàng / Lệnh Trading
```bash
company-mgr finance/trade_gold
```
(Giao diện trò chuyện mở ra. Hệ thống đã nạp sẵn tính cách của một chuyên gia đánh Vàng).

### Ví dụ: Tạo nội dung bài viết Facebook / Tiktok
```bash
company-mgr marketing/content -m "Anh/Em hãy viết bài MKT đánh dấu 10.000 users cho công ty. Không quá dài, đính kèm Icon bắt mắt."
```
---

## 📡 5. Tích Hợp Network Chuyên Sâu

Hệ thống có tự động tải phiên bản **Cloudflare Tunnel (Bản quyền Linux ARM-64)**.
Nếu anh muốn dùng Tunnel kết nối giao diện API:
```bash
cloudflared service install [MÃ_TOKEN_TRÊN_CLOUDFLARE_ZERO_TRUST]
```

Cấu hình xong, Công ty A.I của anh Hưng đã sẵn sàng bùng nổ thu nhập!
