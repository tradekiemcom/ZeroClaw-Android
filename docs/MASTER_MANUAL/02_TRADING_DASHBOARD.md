# 💹 Master Manual 02: Thiết Lập Phòng Trading (Hệ Thống 3 Lớp)

Phòng Trading vận hành theo sơ đồ: **Analyst (Lọc tin) -> Leaders (Chiến thuật) -> Scalper/Fund (Thực thi)**. Toàn bộ chạy trên `localhost` với các cổng riêng biệt.

---

## 1. Trading Analyst (Staff) - Port: 42619
- **Model**: `google/gemini-2.0-flash` (Ưu tiên khả năng quét web cực nhanh).
- **Cấu hình**: `~/.zeroclaw/agents/trading_analyst/config.toml`
```toml
[agent]
name = "Market_Analyst_AI"
model = "google/gemini-2.0-flash"
system_prompt = \"\"\"
Bạn là chuyên gia phân tích dữ liệu vĩ mô. 
Nhiệm vụ: 
1. Quét tin tức từ investing.com và forexfactory hàng giờ.
2. Trích xuất các chỉ số quan trọng (CPI, FOMC, NFP).
3. Báo cáo tình trạng 'Nóng' hay 'Lạnh' của thị trường cho Trading Leader.
\"\"\"
[tools]
enabled = ["web_search", "rss_reader"]
```

---

## 2. Gold Scalper (Staff) - Port: 42621
- **Model**: `meta-llama/llama-3.3-70b-instruct` (Xử lý thời gian thực).
- **Cấu hình**: `~/.zeroclaw/agents/gold_scalper/config.toml`
```toml
[agent]
name = "Gold_Scalper_Pro"
model = "meta-llama/llama-3.3-70b-instruct"
system_prompt = \"\"\"
Bạn là một Robot Scalping Vàng (XAUUSD). 
Nhiệm vụ: 
1. Chỉ thực hiện lệnh khi Analyst báo cáo thị trường ổn định.
2. Theo dõi khung M5. 
3. Luôn luôn đặt Stop Loss và Take Profit theo tỷ lệ RR 1:2.
4. Giao tiếp với MT5-Bridge để đẩy lệnh.
\"\"\"
```

---

## 3. FTMO Fund Manager (Staff) - Port: 42622
- **Model**: `anthropic/claude-3-haiku`.
- **Nhiệm vụ**: Quản lý tài khoản Quỹ, kiểm soát Drawdown không quá 4% mỗi ngày.

---

## 🔗 Luồng Phối Hợp Nội Bộ (Workflows)

1. **Manager (CEO gọi)**: "Yêu cầu báo cáo tình hình Vàng định kỳ."
2. **Trading Leader**: Gọi `Analyst` lấy tin tức -> Gọi `Scalper` kiểm tra tín hiệu nến.
3. **Trading Leader**: Tổng hợp: "Tin tức Non-Farm đang ảnh hưởng, nến M5 đang cho tín hiệu Buy ở 2315, rủi ro thấp." -> Gửi CEO.
4. **CEO Agent**: Chuyển tiếp kết quả lên PA để báo cáo Sếp.
