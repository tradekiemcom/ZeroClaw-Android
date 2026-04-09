# 🐾 ZeroClaw-Android

Dự án cài đặt **[ZeroClaw Core](https://github.com/zeroclaw-labs/zeroclaw)** chuyên biệt để chạy tự nhiên (native) trên môi trường **Termux (Android)** mà không cần quyền Root hay bất kỳ công cụ Linux Proot nào.

---

## 🌟 Chức Năng Cốt Lõi
- Cung cấp kịch bản cài đặt tĩnh (tải bản Binary aarch64 native) lược bỏ quyền sudo.
- Thiết lập sẵn các công cụ giao tiếp liên mạng thông qua Cloudflare Tunnel cho Android.
- Tự động hóa đánh giá năng lực phần cứng để khuyến cáo sử dụng phù hợp.

## 🆕 Tính Năng Mới (v14.0 - Portfolio Release)
- **Portfolio Management**: Trade Leader quản lý danh mục tài khoản (Master, Investor, Fund).
- **EA Lifecycle**: Quy trình khép kín từ yêu cầu nâng cấp EA -> R&D Coding -> Backtest -> Deploy.
- **Strategic Orchestration**: CEO Agent điều phối chiến dịch đa tầng.
- **Trading Connectors**: Tích hợp cTrader Open API & MT5 Web Gateway.

## 🚀 Cài Đặt Nhanh (trên thiết bị Android > Termux)
```bash
pkg update -y && pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
chmod +x install.sh && ./install.sh
```

## 🏢 Cẩm Nang Doanh Nghiệp AI (Autonomous Corp v14.0)
Hệ thống tài liệu **"Cầm tay chỉ việc"** để thiết lập 15+ nhân sự AI trên một thiết bị:
- 👑 **[MASTER MANUAL: CORE PA & CEO](docs/MASTER_MANUAL/01_CORE_STRUCTURE.md)**: Điều hành & Chiến dịch chiến lược.
- 💹 **[MARKET ANALYST & TRADING](docs/MASTER_MANUAL/02_TRADING_DASHBOARD.md)**: Hệ thống soi kèo và thực thi lệnh.
- ⚖️ **[PORTFOLIO & RISK MGMT](docs/MASTER_MANUAL/06_TRADING_PORTFOLIO_MGMT.md)**: Quản lý danh mục tài khoản & Vòng đời EA.
- 🔌 **[TRADING API INTEGRATION](docs/MASTER_MANUAL/05_TRADING_API_INTEGRATION.md)**: Kết nối cTrader & MT5 Gateway.
- 📣 **[CONTENT CREATOR & SOCIAL](docs/MASTER_MANUAL/03_MARKETING_ENGINE.md)**: Nhà máy sản xuất nội dung tự động.
- 🏦 **[FINANCE, SALES & R&D](docs/MASTER_MANUAL/04_FINANCE_SALES_RD.md)**: Quản lý dòng tiền và săn công nghệ.
- 🛠️ **[KỸ THUẬT CHẠY ĐA AGENT (SINGLE-DEVICE)](docs/MASTER_MANUAL/99_TECH_GUIDE_MULTI_DAEMON.md)**: Xác minh xử lý song song trên Android.

## 📙 Tài liệu kỹ thuật
- 👉 **[Hướng dẫn sử dụng Dashboard v8.2](docs/HUONG_DAN_SU_DUNG.md)**
- 👉 **[Cách thiết lập OTA Server (Worker & Dashboard)](docs/HUONG_DAN_TAO_OTA_WORKER.md)**

*(Sau khi cài đặt thành công, hãy truy cập `/admin` trên Worker của bạn để phê duyệt thiết bị)*
