# 💹 Master Manual 05: Tích Hợp API cTrader & MT5 Gateway

Tài liệu này hướng dẫn các Agent Staff trong phòng Trading kết nối với thị trường thực tế thông qua các Bridge (Cầu nối) kỹ thuật.

---

## 1. Kết Nối cTrader Open API (Node.js Bridge)

ZeroClaw sử dụng script `scripts/skills/ctrader_bridge.js` để giao tiếp với trạm Spotware.

### A. Chuẩn bị tài khoản
1. Truy cập [cTrader Open API Portal](https://openapi.ctrader.com/).
2. Tạo một App mới để lấy: `Client ID` và `Client Secret`.
3. Lấy `Access Token` của tài khoản Trading của bạn.

### B. Khai báo biến môi trường (Trên Termux)
```bash
export CTRADER_CLIENT_ID="abc"
export CTRADER_CLIENT_SECRET="xyz"
export CTRADER_ACCESS_TOKEN="123"
```

---

## 2. Kết Nối MT5 Web Gateway (REST API)

Sử dụng script `scripts/skills/mt5_gateway.sh` để đẩy lệnh qua cổng Web.

### A. Cách Agent gọi lệnh Trade
Trong `config.toml` của Gold Scalper Agent, bạn định nghĩa công cụ như sau:
```toml
[tool.execute_trade]
command = "bash ~/.zeroclaw/skills/mt5_gateway.sh --trade {{symbol}} {{type}} {{lots}}"
description = "Mở lệnh giao dịch thực tế trên tài khoản MT5."
```

### B. Quản lý an toàn
- Hãy luôn chạy **MT5 Web Gateway** trên cùng một lớp mạng nội bộ (hoặc cùng máy Note 10+ nếu chạy được emulator) để giảm độ trễ (latency).

---

## 🛰️ 3. Tầm Quan Trọng Của Web Gateway
Việc sử dụng Web Gateway giúp các Agent AI không cần cài đặt MetaTrader trực tiếp lên Android (vốn rất nặng và tốn pin). Toàn bộ việc xử lý đồ họa được lược bỏ, Agent chỉ làm việc với các gói tin JSON siêu nhẹ.
