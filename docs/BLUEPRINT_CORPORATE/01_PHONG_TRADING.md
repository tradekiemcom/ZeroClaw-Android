# 💹 Blueprint 01: Phòng Trading (Trình Lọc Dòng Tiền & Giao Dịch)

Hệ thống quan tâm nhất của Founder. Phòng Trading được thiết kế để xử lý từ phân tích kỹ thuật đến quản lý quỹ FTMO/Prop-firm.

---

## 👨‍💼 1. Trading Manager (Leader)
- **Thiết bị**: Android TV Box mạnh.
- **Model**: `meta-llama/llama-3.1-405b-instruct` (NVIDIA NIM) - Khả năng phân tích sâu sắc nhất.
- **Nhiệm vụ**: Tổng hợp báo cáo từ 3 Staff chuyên trách bên dưới để gửi lên CEO.

---

## 👷 2. Đội Ngũ Nhân Sự (Agent Staff)

### 📊 Agent Phân Tích Kỹ Thuật (Technical Analyst)
- **Model**: `google/gemini-2.0-flash` (Tốc độ cao, hỗ trợ Multimodal - đọc Chart).
- **Skill**: 
  - Đọc nến, xu hướng, RSI, SMC.
  - Quét tin tức Investing.com.
- **Đầu ra**: Xu hướng thị trường (Bullish/Bearish) và các Key Levels.

### 🏆 Agent Trade Gold (Scalping & Swing)
- **Model**: `meta-llama/llama-3.3-70b-instruct` (Tốc độ phản hồi cực nhanh).
- **Chiến thuật**:
  - **Scalping**: Bám sát nến M1-M5 cho tín hiệu lướt sóng.
  - **Swing**: Phân tích H4-D1 cho các lệnh dài hạn.
- **Kết nối**: Link trực tiếp tới Metatrader 5 (qua MT5-Bridge).

### 🏛️ Agent Trade Fund (FTMO / The5ers Specialist)
- **Model**: `anthropic/claude-3-haiku` (Logic chặt chẽ về quản trị rủi ro).
- **Quy tắc**:
  - Tuân thủ tuyệt đối quy tắc Drawdown của Quỹ (ví dụ không quá 5% ngày).
  - Tự động đóng lệnh khi đạt mục tiêu lợi nhuận hoặc chạm giới hạn lỗ.

---

## 🛠️ 3. Mẫu Cấu Hình Trade_Staff.toml

```toml
[agent]
name = "Gold_Scalper_01"
model = "meta-llama/llama-3.3-70b-instruct"
system_prompt = \"\"\"
Bạn là một Trader vàng chuyên nghiệp hệ thống Scalping.
Nhiệm vụ: Chỉ tập trung vào cặp XAUUSD.
Khi có tín hiệu từ TA Agent, bạn sẽ tính toán Lot size dựa trên 1% rủi ro tài khoản.
\"\"\"

[strategy]
pair = "XAUUSD"
timeframe = "M5"
risk_per_trade = 0.01
```

---

## 🔗 Chuyển đến các phòng ban
- 👉 [Phòng Tài chính: Quản lý ví & Dòng tiền](02_PHONG_TAI_CHINH.md)
- 👉 [Phòng R&D: Săn công nghệ & Dev Tool](05_PHONG_RD.md)
